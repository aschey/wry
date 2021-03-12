mod file_drop;

use crate::{
  webview::{mimetype::MimeType, WV},
  FileDropHandler, Result, RpcHandler,
};

use file_drop::FileDropController;

use std::{os::raw::c_void, rc::Rc};

use once_cell::unsync::OnceCell;
use url::Url;
use webview2::{Controller, PermissionKind, PermissionState};
use winapi::{shared::windef::HWND, um::winuser::GetClientRect};
use winit::{platform::windows::WindowExtWindows, window::Window};

pub struct InnerWebView {
  controller: Rc<OnceCell<Controller>>,

  // Store FileDropController in here to make sure it gets dropped when
  // the webview gets dropped, otherwise we'll have a memory leak
  #[allow(dead_code)]
  file_drop_controller: Rc<OnceCell<FileDropController>>,
}

impl WV for InnerWebView {
  type Window = Window;

  fn new<F: 'static + Fn(&str) -> Result<Vec<u8>>>(
    window: &Window,
    scripts: Vec<String>,
    url: Option<Url>,
    // TODO default background color option just adds to webview2 recently and it requires
    // canary build. Implement this once it's in official release.
    transparent: bool,
    custom_protocol: Option<(String, F)>,
    rpc_handler: Option<RpcHandler>,
    file_drop_handler: Option<FileDropHandler>,
  ) -> Result<Self> {
    let hwnd = window.hwnd() as HWND;

    let controller: Rc<OnceCell<Controller>> = Rc::new(OnceCell::new());
    let controller_clone = controller.clone();

    let file_drop_controller: Rc<OnceCell<FileDropController>> = Rc::new(OnceCell::new());
    let file_drop_controller_clone = file_drop_controller.clone();

    // Webview controller
    webview2::EnvironmentBuilder::new().build(move |env| {
      let env = env?;
      let env_ = env.clone();
      env.create_controller(hwnd, move |controller| {
        let controller = controller?;
        let w = controller.get_webview()?;

        // Enable sensible defaults
        let settings = w.get_settings()?;
        settings.put_is_status_bar_enabled(false)?;
        settings.put_are_default_context_menus_enabled(true)?;
        settings.put_is_zoom_control_enabled(false)?;
        settings.put_are_dev_tools_enabled(false)?;
        debug_assert_eq!(settings.put_are_dev_tools_enabled(true)?, ());

        // Safety: System calls are unsafe
        unsafe {
          let mut rect = std::mem::zeroed();
          GetClientRect(hwnd, &mut rect);
          controller.put_bounds(rect)?;
        }

        // Initialize scripts
        w.add_script_to_execute_on_document_created(
          "window.external={invoke:s=>window.chrome.webview.postMessage(s)}",
          |_| (Ok(())),
        )?;
        for js in scripts {
          w.add_script_to_execute_on_document_created(&js, |_| (Ok(())))?;
        }

        // Message handler
        w.add_web_message_received(move |webview, args| {
          let js = args.try_get_web_message_as_string()?;
          if let Some(rpc_handler) = rpc_handler.as_ref() {
            match super::rpc_proxy(js, rpc_handler) {
              Ok(result) => {
                if let Some(ref script) = result {
                  webview.execute_script(script, |_| (Ok(())))?;
                }
              }
              Err(e) => {
                eprintln!("{}", e);
              }
            }
          }
          Ok(())
        })?;

        let mut custom_protocol_name = None;
        if let Some((name, function)) = custom_protocol {
          // WebView2 doesn't support non-standard protocols yet, so we have to use this workaround
          // See https://github.com/MicrosoftEdge/WebView2Feedback/issues/73
          custom_protocol_name = Some(name.clone());
          w.add_web_resource_requested_filter(
            &format!("file://custom-protocol-{}*", name),
            webview2::WebResourceContext::All,
          )?;
          w.add_web_resource_requested(move |_, args| {
            let uri = args.get_request()?.get_uri()?;
            // Undo the protocol workaround when giving path to resolver
            let path = &uri.replace(
              &format!("file://custom-protocol-{}", name),
              &format!("{}://", name),
            );

            match function(path) {
              Ok(content) => {
                let mime = MimeType::parse(&content, &uri);
                let stream = webview2::Stream::from_bytes(&content);
                let response = env_.create_web_resource_response(
                  stream,
                  200,
                  "OK",
                  &format!("Content-Type: {}", mime),
                )?;
                args.put_response(response)?;
                Ok(())
              }
              Err(_) => Err(webview2::Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error loading requested file",
              ))),
            }
          })?;
        }

        // Enable clipboard
        w.add_permission_requested(|_, args| {
          let kind = args.get_permission_kind()?;
          if kind == PermissionKind::ClipboardRead {
            args.put_state(PermissionState::Allow)?;
          }
          Ok(())
        })?;

        // Navigation
        if let Some(url) = url {
          let mut url_string = String::from(url.as_str());
          if let Some(name) = custom_protocol_name {
            if name == url.scheme() {
              // WebView2 doesn't support non-standard protocols yet, so we have to use this workaround
              // See https://github.com/MicrosoftEdge/WebView2Feedback/issues/73
              url_string = url.as_str().replace(
                &format!("{}://", name),
                &format!("file://custom-protocol-{}", name),
              )
            }
          }
          w.navigate(&url_string)?;
        }

        let _ = controller_clone.set(controller);

        if let Some(file_drop_handler) = file_drop_handler {
          let mut file_drop_controller = FileDropController::new();
          file_drop_controller.listen(hwnd, file_drop_handler);
          let _ = file_drop_controller_clone.set(file_drop_controller);
        }

        Ok(())
      })
    })?;

    Ok(Self {
      controller,

      file_drop_controller,
    })
  }

  fn eval(&self, js: &str) -> Result<()> {
    if let Some(c) = self.controller.get() {
      let webview = c.get_webview()?;
      webview.execute_script(js, |_| (Ok(())))?;
    }
    Ok(())
  }
}

impl InnerWebView {
  pub fn resize(&self, hwnd: *mut c_void) -> Result<()> {
    let hwnd = hwnd as HWND;

    // Safety: System calls are unsafe
    unsafe {
      let mut rect = std::mem::zeroed();
      GetClientRect(hwnd, &mut rect);
      if let Some(c) = self.controller.get() {
        c.put_bounds(rect)?;
      }
    }

    Ok(())
  }
}
