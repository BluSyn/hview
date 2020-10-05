#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, NamedFile};
use rocket::response::status::NotFound;
use rocket_contrib::templates::Template;

const BASE_DIR: &str = "./test-fixture/";

#[derive(Serialize, Deserialize, Debug)]
struct TemplateEntry {
    name: String,
    href: String,
    size: u64,
    thumb: String,
    ext: String
}

impl TemplateEntry {
    fn new() -> Self {
        Self {
            name: String::from(""),
            href: String::from(""),
            size: 0,
            thumb: String::from(""),
            ext: String::from("")
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TemplatePage {
    title: String,
    base_path: String,
    read_only: bool,
    files: Vec<TemplateEntry>,
    folders: Vec<TemplateEntry>
}

impl TemplatePage {
    fn new() -> Self {
        Self {
            title: String::from(""),
            base_path: String::from(BASE_DIR),
            read_only: false,
            files: Vec::new(),
            folders: Vec::new()
        }
    }
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

fn get_dir(path: &PathBuf) -> io::Result<Template> {
    let mut page = TemplatePage::new();

    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        let meta = entry.metadata().unwrap();

        let mut details = TemplateEntry::new();
        details.name = entry.file_name().to_os_string().into_string().unwrap();
        details.href = String::from(path.to_path_buf().to_str().unwrap());
        details.size = meta.len();
        details.ext = path.extension().unwrap().to_os_string().into_string().unwrap();

        if path.is_dir() {
            page.folders.push(details);
        } else {
            page.files.push(details);
        }
    }

    // let mut data = HashMap::new();
    // data.insert("title", "Testing");

    Ok(Template::render("dir", &page))
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

            match get_dir(&path) {
                Ok(t) => {
                    response.tmpl = Some(t);
                    Ok(response)
                },
                Err(_) => Err(NotFound("Dir does not exist"))
            }
        },
        Err(_) => Err(NotFound("sdfsdf"))
    }
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![main_route]).launch();
}
