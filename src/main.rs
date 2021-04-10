mod apps;
mod ui;

use std::sync::mpsc::channel;
use std::thread;

fn main() {
  let (app_events_sender, app_events_receiver) = channel();
  let (ui_events_sender, ui_events_receiver) = channel();
  let mut ui = ui::Ui::new(app_events_receiver, ui_events_sender);
  thread::spawn(move || {
    let mut app_handler = apps::AppHandler::new(app_events_sender, ui_events_receiver);
    app_handler.start();
    println!("thread done")
  });
  ui.start();
}
