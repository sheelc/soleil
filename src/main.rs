mod apps;
mod ui;

use std::sync::mpsc::channel;
use std::thread;

fn main() {
  let (app_events_sender, app_events_receiver) = channel();
  let (ui_events_sender, ui_events_receiver) = channel();

  let apps_config = apps::AppsConfig::new("test.toml");

  let mut ui = ui::Ui::new(apps_config.clone(), app_events_receiver, ui_events_sender);
  thread::spawn(move || {
    let mut app_handler =
      apps::AppHandler::new(apps_config.clone(), app_events_sender, ui_events_receiver);
    app_handler.start();
  });
  ui.start();
}
