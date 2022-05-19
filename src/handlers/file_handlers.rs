use actix_files::NamedFile;
use actix_web::HttpRequest;
use actix_web::{get, Responder, Result};
use std::path::PathBuf;

#[get("/favicon")]
pub async fn favicon_handler() -> Result<impl Responder> {
  Ok(NamedFile::open("web/build/images/favicon.ico"))
}

#[get("/manifest.json")]
pub async fn mainfest_handler() -> Result<impl Responder> {
  Ok(NamedFile::open("web/build/manifest.json"))
}

pub async fn icons_handlers(req: HttpRequest) -> Result<impl Responder> {
  let path: PathBuf = req.match_info().query("filename").parse().unwrap();
  let file_name = path.to_str().unwrap_or("icon-512x512.png");
  Ok(NamedFile::open(format!(
    r"web/build/images/icons/{}",
    file_name
  )))
}

pub async fn scripts_handlers(req: HttpRequest) -> Result<impl Responder> {
  let path: PathBuf = req.match_info().query("filename").parse().unwrap();
  let file_name = path.to_str().unwrap_or("main.f5b9e117.js");
  Ok(NamedFile::open(format!(
    r"web/build/static/js/{}",
    file_name
  )))
}

pub async fn css_handlers(req: HttpRequest) -> Result<impl Responder> {
  let path: PathBuf = req.match_info().query("filename").parse().unwrap();
  let file_name = path.to_str().unwrap_or("main.f5b9e117.js");
  Ok(NamedFile::open(format!(
    r"web/build/static/css/{}",
    file_name
  )))
}

pub async fn media_handlers(req: HttpRequest) -> Result<impl Responder> {
  let path: PathBuf = req.match_info().query("filename").parse().unwrap();
  let file_name = path.to_str().unwrap_or("main.f5b9e117.js");
  Ok(NamedFile::open(format!(
    r"web/build/static/media/{}",
    file_name
  )))
}
