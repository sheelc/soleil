use serde::Deserialize;
use std::fmt;
use std::sync::Arc;

use toml;

pub type AppId = String;

#[derive(Deserialize)]
struct Config {
  apps: Vec<AppConfig>,
}

#[derive(Deserialize)]
pub struct AppConfig {
  pub name: String,
  pub program: String,
  pub program_args: Vec<String>,
}

pub struct AppsConfig {
  config: Config,
}

#[derive(Debug, Clone)]
pub struct AppNotFoundError {
  appid: AppId,
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
        program = "echo"
        program_args = ["kafka echo going here"]

        [[apps]]
        name = "zookeeper"
        program = "echo"
        program_args = ["zookeeper"]

        [[apps]]
        name = "postgres"
        program = "echo"
        program_args = ["postgres"]
    "##,
    )
    .unwrap();

    Arc::new(AppsConfig { config })
  }

  pub fn apps(&self) -> &Vec<AppConfig> {
    &self.config.apps
  }

  pub fn app_config_from_id(&self, appid: &str) -> Result<&AppConfig, AppNotFoundError> {
    for app in self.apps().iter() {
      if app.name == appid {
        return Ok(app);
      }
    }

    Err(AppNotFoundError {
      appid: appid.to_string(),
    })
  }
}
