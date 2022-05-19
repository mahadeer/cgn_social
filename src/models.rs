use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
  #[serde(default = "get_application_name")]
  pub application_name: String,
  #[serde(default = "get_default_host")]
  pub host: String,
  #[serde(default = "get_default_port")]
  pub port: u16,
  #[serde(default = "get_default_workers")]
  pub workers: usize,
}

fn get_application_name() -> String {
  format!("CGN Server")
}

fn get_default_host() -> String {
  format!("127.0.0.1")
}

fn get_default_port() -> u16 {
  8080
}

fn get_default_workers() -> usize {
  2
}
