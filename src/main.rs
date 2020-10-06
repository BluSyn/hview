#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::io;
use std::fs;
use std::path::{Path, PathBuf};

use rand::seq::IteratorRandom;
use serde::{Serialize, Deserialize};
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, NamedFile};
use rocket::response::status::NotFound;
use rocket_contrib::templates::Template;

const THUMB_FORMAT: &str = "avif";
const BASE_DIR: &str = "./test-fixture/";

#[derive(Serialize, Deserialize, Debug)]
struct TemplateEntry {
    name: String,
    href: String,
    size: u64,
    thumb: Option<PathBuf>,
    ext: Option<String>
}

impl TemplateEntry {
    fn new() -> Self {
        Self {
            name: String::from(""),
            href: String::from(""),
            size: 0,
            thumb: None,
            ext: None
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TemplatePage {
    title: String,
    base_path: String,
    read_only: bool,
    files:  Vec<TemplateEntry>,
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

// Convert /dir/file.jpg -> /dir/.th/file.jpg.avif
fn file_path_to_thumb(file: &PathBuf) -> Result<PathBuf, &'static str> {
	let thumb_name = format!(".th/{}.{}",
		file
			.file_name().ok_or("Filename")?
			.to_str().ok_or("Filename String")?,
		THUMB_FORMAT);
	let file_thumb = file
		.parent().ok_or("Parent directory")?
		.join(&thumb_name);

	Ok(file_thumb)
}

// Get file inside dir that ends with THUMB_FORMAT, if exists
fn get_random_thumb(path: &PathBuf) -> Option<PathBuf> {
    if !path.is_dir() || !path.exists() {
        return None;
    }

    let thumbs = fs::read_dir(&path).unwrap()
        .filter_map(|d| {
            let path = d.unwrap().path();
            if path.extension().unwrap() == "avif" {
                return Some(path);
            }
            None
        });

    let mut rng = rand::thread_rng();
    Some(thumbs.choose(&mut rng).unwrap())
}

fn get_dir(dir: &PathBuf) -> io::Result<Template> {
    let mut page = TemplatePage::new();
    let thpath = dir.join(".th");

    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();

        // Skip existing th paths
        if path == thpath {
            continue;
        }

        let meta = entry.metadata().unwrap();
        let ext = path.extension();

        let mut details = TemplateEntry::new();
        details.name = entry.file_name().to_str().unwrap().to_string();
        details.href = path.to_path_buf().to_str().unwrap().to_string();
        details.size = meta.len();
        details.ext = if ext.is_none() {
            None
        } else {
            Some(ext.unwrap().to_str().unwrap().to_string())
        };

        if path.is_dir() {
            details.thumb = get_random_thumb(&thpath);
            page.folders.push(details);
        } else {
            details.thumb = file_path_to_thumb(&path).ok();
            page.files.push(details);
        }
    }

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
