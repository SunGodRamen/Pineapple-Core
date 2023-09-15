// file_launcher.rs
use std::process::Command;
use crate::message_handler::interpreter::ResponseMessage;

pub fn open_file(request_body: &str, response: &mut ResponseMessage) -> Result<(), Box<dyn std::error::Error>> {
  //check for app
  let mut path = request_body.to_string();
  let mut app = None;
  if request_body.contains(" ") {
      let mut split = request_body.split(" ");
      app = Some(split.next().unwrap().to_string());
      path = split.next().unwrap().to_string();
  }

  if let Some(app) = app {
      Command::new(app)
          .arg(path)
          .spawn()?;
  } else {
      Command::new("start")
          .arg(path)
          .spawn()?;
  }
  response.status_code = 200;
  response.body = "File opened".to_string();
  Ok(())
}