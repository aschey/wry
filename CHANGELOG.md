# Changelog

## \[0.6.2]

- Add draggable regions, just add `drag-region` class to the html element.
  - [b2a0bfc](https://github.com/tauri-apps/wry/commit/b2a0bfc289786d0a23dac0c8d9543771e70e3427) feat/ draggable-region ([#92](https://github.com/tauri-apps/wry/pull/92)) on 2021-03-25
- Add event listener in application proxy
  - [c49846c](https://github.com/tauri-apps/wry/commit/c49846cfc41bb548a685edeac5f8036501f7dcec) feat: event listener ([#129](https://github.com/tauri-apps/wry/pull/129)) on 2021-03-26
- Better result errror handling
  - [485035f](https://github.com/tauri-apps/wry/commit/485035f17d28560966b07b512935821814f0e951) chore: better result error handling ([#124](https://github.com/tauri-apps/wry/pull/124)) on 2021-03-21
- Fix visibility on webview2 when window was invisible previously and then shown.
  - [6d31706](https://github.com/tauri-apps/wry/commit/6d31706a6bff43e9b28100675cf8fc12f29db248) Fix visibility on webview2 when window was invisible previously ([#128](https://github.com/tauri-apps/wry/pull/128)) on 2021-03-24

## \[0.6.1]

- Add attribute option to allow WebView on Windows use user_data folder
  - [8dd58ee](https://github.com/tauri-apps/wry/commit/8dd58eec77d4c89491b1af427d06c4ee6cfa8e58) feat/ allow webview2 (windows) to use optional user_data folder provided by the attributes ([#120](https://github.com/tauri-apps/wry/pull/120)) on 2021-03-21

## \[0.6.0]

- Initialize covector!
  - [33b64ed](https://github.com/tauri-apps/wry/commit/33b64ed5c208b778d03dbb5f3f2808bb417c9f52) chore: covector init ([#55](https://github.com/tauri-apps/wry/pull/55)) on 2021-02-21
- Support Windows 7, 8, and 10
  - [fbf0d17](https://github.com/tauri-apps/wry/commit/fbf0d17164da455400aaa44104c3925eded09393) Adopt Webview2 on Windows ([#48](https://github.com/tauri-apps/wry/pull/48)) on 2021-02-20
- Dev tools are enabled on debug build
- Add skip task bar option
  - [395b6fb](https://github.com/tauri-apps/wry/commit/395b6fbcd66f6cbd0457cb609bea4afe734fadd4) feat: `skip_taskbar` for windows ([#49](https://github.com/tauri-apps/wry/pull/49)) on 2021-02-20
- Add custom protocol option
  - [a492806](https://github.com/tauri-apps/wry/commit/7a492806d716a30abe15a2104b64152c1ca370bb) Add custom protocol ([#65](https://github.com/tauri-apps/wry/pull/65)) on 2021-02-23
- Add transparent option to mac and linux
- Error type has Send/Sync traits
  - [3536b83](https://github.com/tauri-apps/wry/commit/3536b831ec30ee7436616ba4b262bbdd1e6279c8) Add .changes file in prepare of v0.6 on 2021-02-24
- Replace Callback with RPC handler
  - [e215157](https://github.com/tauri-apps/wry/commit/e215157146f0eab8ee6beab0628b036c68eea108) Implement draft RPC API ([#95](https://github.com/tauri-apps/wry/pull/95)) on 2021-03-04
- Add File drop handlers
  - [fed0ee7](https://github.com/tauri-apps/wry/commit/fed0ee772100ad19a344a85266618c7bcf7cb649) File drop handlers ([#96](https://github.com/tauri-apps/wry/pull/96)) on 2021-03-09
