extern crate dotenv;
use actix_web::{middleware::Logger, web, App, HttpServer};
use std::{env, net::SocketAddr};

mod config;
mod handlers;
use config::get_config;
use handlers::{application_page_handler, fallback_handler};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
  let app_config = get_config();
  let address = SocketAddr::from((app_config.host, get_server_port()));
  let log_message = format!(
    "starting {} server at http://{}",
    app_config.application_name, address
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
  env::var("PORT")
    .ok()
    .and_then(|port| port.parse().ok())
    .unwrap_or_else(|| 8080)
}
