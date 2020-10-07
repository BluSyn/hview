/**
 * File system calls
 */

use std::path::PathBuf;
use std::fs::read_dir;
use std::io::Result as ioResult;

use serde::{Serialize, Deserialize};
use rand::seq::IteratorRandom;

use crate::app::config::{CFG, BASEPATH, THUMB_FORMAT};

#[derive(Serialize, Deserialize, Debug)]
struct TemplateEntry {
    name: String,
    path: String,
    size: u64,
    thumb: Option<PathBuf>,
    ext: Option<String>
}

impl TemplateEntry {
    fn new() -> Self {
        Self {
            name: String::from(""),
            path: String::from(""),
            size: 0,
            thumb: None,
            ext: None
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplatePage {
    title: String,
    base_path: &'static str,
    read_only: bool,
    files:  Vec<TemplateEntry>,
    folders: Vec<TemplateEntry>
}

impl TemplatePage {
    fn new() -> Self {
        Self {
            title: String::from(""),
            base_path: *BASEPATH,
            read_only: CFG.read_only,
            files: Vec::new(),
            folders: Vec::new()
        }
    }
}

pub fn get_dir(dir: &PathBuf) -> ioResult<TemplatePage> {
    let mut page = TemplatePage::new();
    page.title = dir.to_str().unwrap().to_string();

    let thpath = dir.join(".th");

    for entry in read_dir(dir)? {
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
        details.path = path.to_path_buf().to_str().unwrap().to_string();
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

    Ok(page)
}

// Convert /dir/file.jpg -> /dir/.th/file.jpg.avif
pub fn file_path_to_thumb(file: &PathBuf) -> Result<PathBuf, &'static str> {
	let thumb_name = format!(".th/{}.{}",
		file
			.file_name().ok_or("Filename")?
			.to_str().ok_or("Filename String")?,
		*THUMB_FORMAT);
	let file_thumb = file
		.parent().ok_or("Parent directory")?
		.join(&thumb_name);

	Ok(file_thumb)
}

// Get file inside dir that ends with THUMB_FORMAT, if exists
pub fn get_random_thumb(path: &PathBuf) -> Option<PathBuf> {
    if !path.is_dir() || !path.exists() {
        return None;
    }

    let thumbs = read_dir(&path).unwrap()
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
