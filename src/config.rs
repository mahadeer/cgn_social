use std::net::IpAddr;
use std::net::Ipv4Addr;

pub struct AppConfig {
  pub application_name: String,
  pub host: IpAddr,
  pub port: u16,
  pub workers: usize,
}

pub fn get_config() -> AppConfig {
  AppConfig {
    application_name: format!("CGN Social"),
    host: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
    port: 8080,
    workers: 2,
  }
}
