use serde::Deserialize;
use std::fmt;
use std::sync::Arc;

use toml;

#[derive(Deserialize)]
struct Config {
  apps: Vec<AppConfig>,
}

#[derive(Deserialize)]
pub struct AppConfig {
  pub name: String,
  pub start_command: String,
}

pub struct AppsConfig {
  config: Config,
}

#[derive(Debug, Clone)]
pub struct AppNotFoundError {
  appid: String,
}

impl fmt::Display for AppNotFoundError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "app id not found: {}", self.appid)
  }
}

impl AppsConfig {
  pub fn new<S>(_apps_config_filename: S) -> Arc<AppsConfig>
  where
    S: Into<String>,
  {
    let config: Config = toml::from_str(
      r##"
        [[apps]]
        name = "kafka"
        start_command = "echo kafka"

        [[apps]]
        name = "zookeeper"
        start_command = "echo zookeeper"

        [[apps]]
        name = "postgres"
        start_command = "echo postgres"
    "##,
    )
    .unwrap();

    Arc::new(AppsConfig { config })
  }

  pub fn apps(&self) -> &Vec<AppConfig> {
    &self.config.apps
  }

  pub fn app_config_from_id(&self, appid: String) -> Result<&AppConfig, AppNotFoundError> {
    for app in self.apps().iter() {
      if app.name == appid {
        return Ok(app);
      }
    }

    Err(AppNotFoundError { appid })
  }
}
