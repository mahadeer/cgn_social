extern crate dotenv;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use std::{env, net::SocketAddr};

mod handlers;
mod models;
use handlers::{application_page_handler, fallback_handler};
use models::AppConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().expect("Failed to read .env file");
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
  let app_config = envy::prefixed("APP_")
    .from_env::<AppConfig>()
    .expect("Please provide port, host and workers values");
  let address = SocketAddr::from(([127, 0, 0, 1], get_server_port()));
  let log_message = format!(
    "starting {} server at http://{}:{}",
    app_config.application_name, address, app_config.port
  );
  log::info!("{}", log_message);
  HttpServer::new(|| {
    App::new()
      .configure(application_page_handler)
      .default_service(web::to(fallback_handler))
      .wrap(Logger::new("%r"))
  })
  .bind(address)?
  .workers(app_config.workers)
  .run()
  .await
}

fn get_server_port() -> u16 {
  env::var("APP_PORT")
    .ok()
    .and_then(|port| port.parse().ok())
    .unwrap_or_else(|| 8186)
}
