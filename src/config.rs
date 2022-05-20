pub struct AppConfig {
  pub application_name: String,
  pub host: String,
  pub port: u16,
  pub workers: usize,
}

pub fn get_config() -> AppConfig {
  AppConfig {
    application_name: format!("CGN Social"),
    host: format!("127.0.0.1"),
    port: 8080,
    workers: 2,
  }
}
