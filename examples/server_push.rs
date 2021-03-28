use std::{thread, time::Duration};

use serde::{Deserialize, Serialize};
use wry::{Application, Result};

#[derive(Debug, Serialize, Deserialize)]
struct MessageParameters {
  message: String,
}

fn main() -> Result<()> {
  let mut app = Application::new()?;

  let window = app.add_window(Default::default())?;

  thread::spawn(move || {
    let mut i = 0;
    loop {
      window
        .evaluate_script(format!("document.body.innerText={}", i))
        .unwrap();

      i += 1;
      thread::sleep(Duration::from_millis(500));
    }
  });
  app.run();
  Ok(())
}
