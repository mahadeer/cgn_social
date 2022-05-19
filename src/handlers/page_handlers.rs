use actix_files::NamedFile;
use actix_web::{get, Responder, Result};

#[get("/")]
pub async fn root_page_handler() -> Result<impl Responder> {
  Ok(NamedFile::open("web/build/index.html"))
}
