use actix_files::NamedFile;
use actix_web::http::{Method, StatusCode};
use actix_web::web;
use actix_web::Either;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::Result;

mod file_handlers;
mod page_handlers;

use file_handlers::{
  css_handlers, favicon_handler, icons_handlers, mainfest_handler, media_handlers, scripts_handlers,
};
use page_handlers::root_page_handler;

pub fn application_page_handler(cfg: &mut web::ServiceConfig) {
  cfg
    .service(root_page_handler)
    .service(favicon_handler)
    .service(mainfest_handler)
    .route("/static/js/{filename:.*}", web::get().to(scripts_handlers))
    .route("/static/css/{filename:.*}", web::get().to(css_handlers))
    .route("/static/media/{filename:.*}", web::get().to(media_handlers))
    .route("/icons/{filename:.*}", web::get().to(icons_handlers));
}

pub async fn fallback_handler(req_method: Method) -> Result<impl Responder> {
  match req_method {
    Method::GET => {
      let file = NamedFile::open("web/build/index.html")?.set_status_code(StatusCode::NOT_FOUND);
      Ok(Either::Left(file))
    }
    _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
  }
}
