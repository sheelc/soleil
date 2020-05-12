mod apps;
mod ui;

use std::sync::mpsc::channel;
use std::thread;
use crate::apps::AppEvent;
use crate::ui::UiEvent;

fn main() {
  let (app_events_sender, app_events_receiver) = channel();
  let (ui_events_sender, ui_events_receiver) = channel();
  let mut ui = ui::Ui::new(app_events_receiver, ui_events_sender);
  thread::spawn(move || {
    for msg in ui_events_receiver {
      match msg {
        UiEvent::SelectApp(appid) => {
          app_events_sender.send(AppEvent::SelectedApp(appid));
        }
      }
    }
  });
  ui.start();
}
