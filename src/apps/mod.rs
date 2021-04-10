use crate::ui::UiEvent;

use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug, Clone)]
pub enum AppActionCmd {
  Start,
  Stop,
  Restart,
}

#[derive(Debug)]
pub enum AppEvent {
  SelectedApp(String),
}

pub struct AppHandler {
  app_events: Sender<AppEvent>,
  ui_events: Receiver<UiEvent>,
}

impl AppHandler {
  pub fn new(app_events: Sender<AppEvent>, ui_events: Receiver<UiEvent>) -> AppHandler {
    AppHandler {
      app_events,
      ui_events,
    }
  }

  pub fn start(&mut self) {
    for msg in self.ui_events.iter() {
      match msg {
        UiEvent::SelectApp(appid) => {
          self.app_events.send(AppEvent::SelectedApp(appid)).unwrap();
        }
        UiEvent::AppAction(appid, action) => match action {
          AppActionCmd::Start => println!("started app {}", appid),
          AppActionCmd::Stop => println!("stopped app {}", appid),
          AppActionCmd::Restart => println!("restarted app {}", appid),
        },
      }
    }
  }
}
