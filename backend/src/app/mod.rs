use std::fs::metadata;
use std::time::Instant;

use rocket::fs::NamedFile;
use rocket::get;
use rocket::response::status::NotFound;

mod fs;
mod pathbuf;
mod responder;

use crate::{app::fs::get_dir, config::DIR};
use pathbuf::CustomPathBuf;
use responder::CustomResponder;
use rocket::serde::json::Json;

#[get("/<file..>")]
pub async fn route(file: CustomPathBuf) -> Result<CustomResponder, NotFound<&'static str>> {
    let path = DIR.join(file.path());
    let mut response = CustomResponder::new();

    match metadata(&path) {
        Ok(meta) => {
            if meta.is_file() {
                match NamedFile::open(&path).await {
                    Ok(f) => {
                        response.file = Some(f);
                        return Ok(response);
                    }
                    Err(_) => return Err(NotFound("Could not load file")),
                }
            }

            // profile this function call
            let now = Instant::now();
            match get_dir(&path) {
                Ok(dir) => {
                    println!("Time elapsed {}ms", now.elapsed().as_micros());
                    response.dir = Some(Json(dir));
                    Ok(response)
                }
                Err(_) => Err(NotFound("Dir does not exist")),
            }
        }
        Err(_) => Err(NotFound("File not found")),
    }
}

#[get("/")]
pub async fn home() -> Result<CustomResponder, NotFound<&'static str>> {
    route(CustomPathBuf::from_str(".")).await
}
