use crate::apps::AppsConfig;
use crate::ui::UiEvent;
use std::sync::Arc;

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
  apps_config: Arc<AppsConfig>,
  app_events: Sender<AppEvent>,
  ui_events: Receiver<UiEvent>,
}

impl AppHandler {
  pub fn new(
    apps_config: Arc<AppsConfig>,
    app_events: Sender<AppEvent>,
    ui_events: Receiver<UiEvent>,
  ) -> AppHandler {
    AppHandler {
      apps_config,
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
        UiEvent::AppAction(appid, action) => {
          let selected_app_config = self.apps_config.app_config_from_id(appid.clone()).unwrap();
          match action {
            AppActionCmd::Start => println!(
              "started app w/ command {}",
              selected_app_config.start_command
            ),
            AppActionCmd::Stop => println!("stopped app {}", appid),
            AppActionCmd::Restart => println!("restarted app {}", appid),
          }
        }
      }
    }
  }
}
