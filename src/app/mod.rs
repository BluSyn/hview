/**
 * Main hview structs and config
 */

use std::fs::metadata;

use rocket::response::{NamedFile, status::NotFound};
use rocket_contrib::templates::Template;

mod responder;
mod pathbuf;
mod fs;
pub mod config;

use fs::get_dir;
use responder::CustomResponder;
use pathbuf::CustomPathBuf;
use config::DIR;

#[get("/<file..>")]
pub fn route(file: CustomPathBuf) -> Result<CustomResponder, NotFound<&'static str>> {
    let path = DIR.join(file.path());
    let mut response = CustomResponder::new();

    match metadata(&path) {
        Ok(meta) => {
            if meta.is_file() {
                match NamedFile::open(&path) {
                    Ok(f) => {
                        response.file = Some(f);
                        return Ok(response);
                    },
                    Err(_) => return Err(NotFound("Could not load file"))
                }
            }

            match get_dir(&path) {
                Ok(page) => {
                    response.tmpl = Some(Template::render("dir", page));
                    Ok(response)
                },
                Err(_) => Err(NotFound("Dir does not exist"))
            }
        },
        Err(_) => Err(NotFound("File not found"))
    }
}

#[get("/")]
pub fn home() -> Result<CustomResponder, NotFound<&'static str>> {
    route(CustomPathBuf::from("."))
}
