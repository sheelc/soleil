use crate::apps::{AppId, AppsConfig};
use crate::ui::UiEvent;

use std::collections::HashMap;
use std::error::Error;
use std::process::{Command, Stdio};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

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
  app_processes: HashMap<AppId, Process>,
}

struct Process {}

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
      app_processes: HashMap::new(),
    }
  }

  pub fn start(&mut self) {
    for msg in self.ui_events.iter() {
      match msg {
        UiEvent::SelectApp(appid) => {
          self.app_events.send(AppEvent::SelectedApp(appid)).unwrap();
        }
        UiEvent::AppAction(appid, action) => {
          let selected_app_config = self.apps_config.app_config_from_id(&appid).unwrap();
          match action {
            AppActionCmd::Start => {
              println!("started app w/ command {}", selected_app_config.program);
              self.spawn_process(&appid);
            }
            AppActionCmd::Stop => println!("stopped app {}", appid),
            AppActionCmd::Restart => println!("restarted app {}", appid),
          }
        }
      }
    }
  }

  fn spawn_process(&self, appid: &AppId) -> Result<String, Box<dyn Error>> {
    if let Some(_) = self.app_processes.get(appid) {
      Ok(String::from("process was already spawned"))
    } else {
      let selected_app_config = self.apps_config.app_config_from_id(&appid).unwrap();
      Command::new(&selected_app_config.program)
        .args(&selected_app_config.program_args)
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
      Ok(String::from("process spawned"))
    }
  }
}
