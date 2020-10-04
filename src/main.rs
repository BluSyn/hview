#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use thiserror::Error;

use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, NamedFile};
use rocket::response::status::NotFound;
use rocket_contrib::templates::Template;

const BASE_DIR: &str = "./test-fixture/";

struct TemplateRow {
    name: String,
    href: String,
    size: String,
    thumb: String,
    ext: String
}

struct TemplatePage {
    title: String,
    base_path: String,
    read_only: bool,
    files: Vec<TemplateRow>,
    folders: Vec<TemplateRow>
}

struct CustomResponder {
    file: Option<NamedFile>,
    tmpl: Option<Template>
}

impl CustomResponder {
    fn new() -> Self {
        Self {
            file: None,
            tmpl: None
        }
    }
}

impl<'a> Responder<'a> for CustomResponder {
    fn respond_to(self, req: &Request) -> response::Result<'a> {
        if self.file.is_some() {
            return self.file.unwrap().respond_to(&req);
        } else if self.tmpl.is_some() {
            return self.tmpl.unwrap().respond_to(&req);
        }

        Err(Status::NotFound)
    }
}

fn get_dir(dir: fs::Metadata) -> Option<Template> {
    if !dir.is_dir() {
        return None;
    }

    let mut context = HashMap::new();
    context.insert("Title", "Testing");

    Some(Template::render("dir.tmpl", &context))
}

#[get("/<file..>")]
fn main_route(file: PathBuf) -> Result<CustomResponder, NotFound<&'static str>> {
    let path = Path::new(BASE_DIR).join(file);
    let mut response = CustomResponder::new();

    match fs::metadata(&path) {
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

            match get_dir(meta) {
                Some(t) => {
                    response.tmpl = Some(t);
                    Ok(response)
                },
                None => Err(NotFound("Dir does not exist"))
            }
        },
        Err(_) => Err(NotFound("sdfsdf"))
    }
}

fn main() {
    rocket::ignite()
        // .attach(Template::fairing())
        .mount("/", routes![main_route]).launch();
}
