use std::fs::read_dir;
use std::path::PathBuf;
use thiserror::Error;

use chrono::{TimeZone, Utc};
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};

use crate::config::{BASEPATH, CFG, DIR, THUMB_FORMAT};

#[derive(Error, Debug)]
pub enum DirError {
    #[error("Directory not found")]
    NotFound,

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

#[derive(Serialize, Deserialize, Debug)]
struct DirEntry {
    name: String,
    path: String,
    size: u64,
    date: u64,
    date_string: String,
    thumb: Option<String>,
    ext: Option<String>,
}

impl DirEntry {
    fn new() -> Self {
        Self {
            name: String::from(""),
            path: String::from(""),
            size: 0,
            date: 0,
            date_string: String::from(""),
            thumb: None,
            ext: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dir {
    title: String,
    base_path: &'static str,
    read_only: bool,
    files: Vec<DirEntry>,
    folders: Vec<DirEntry>,
}

impl Dir {
    fn new() -> Self {
        Self {
            title: String::from(""),
            base_path: *BASEPATH,
            read_only: CFG.read_only,
            files: Vec::new(),
            folders: Vec::new(),
        }
    }
}

// Load DIR details into Dir struct
pub fn get_dir(dir: &PathBuf) -> Result<Dir, DirError> {
    if !dir.is_dir() {
        return Err(DirError::NotFound);
    }

    let mut page = Dir::new();
    let basedir = DIR.as_path();
    let thpath = dir.join(".th");
    page.title = dir.strip_prefix(&basedir).unwrap().display().to_string();

    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // Skip thumbnail dir
        if path == thpath {
            continue;
        }

        let meta = entry.metadata().unwrap();
        let mut details = DirEntry::new();

        details.name = entry.file_name().into_string().ok().unwrap();
        details.path = path.strip_prefix(&basedir).unwrap().display().to_string();
        details.size = meta.len();
        details.date = if let Ok(date) = meta.modified() {
            date.duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        } else {
            0
        };

        details.date_string = if details.date > 0 {
            Utc.timestamp(details.date as i64, 0)
                .format("%Y-%m-%d")
                .to_string()
        } else {
            String::from("")
        };

        details.ext = if let Some(ext) = path.extension() {
            Some(ext.to_str().unwrap().to_string())
        } else {
            None
        };

        // Folders display a random thumbnail from all their files (if available)
        // Files return their individual thumbnail (if available)
        if path.is_dir() {
            details.thumb = if let Some(th) = get_random_thumb(&path.join(".th")) {
                Some(th.strip_prefix(&basedir).unwrap().display().to_string())
            } else {
                None
            };
            page.folders.push(details);
        } else {
            details.thumb = if let Ok(tpath) = file_path_to_thumb(&path) {
                if tpath.exists() {
                    Some(tpath.strip_prefix(&basedir).unwrap().display().to_string())
                } else {
                    None
                }
            } else {
                None
            };

            page.files.push(details);
        }
    }

    // sort by name
    page.folders.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    page.files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    Ok(page)
}

// Convert /dir/file.jpg -> /dir/.th/file.jpg.avif
pub fn file_path_to_thumb(file: &PathBuf) -> Result<PathBuf, &'static str> {
    let thumb_name = format!(
        ".th/{}.{}",
        file.file_name()
            .ok_or("Filename")?
            .to_str()
            .ok_or("Filename String")?,
        *THUMB_FORMAT
    );
    let file_thumb = file.parent().ok_or("Parent directory")?.join(&thumb_name);

    Ok(file_thumb)
}

// Get file inside dir that ends with THUMB_FORMAT, if exists
pub fn get_random_thumb(path: &PathBuf) -> Option<PathBuf> {
    if !path.is_dir() || !path.exists() {
        return None;
    }

    let thumbs = read_dir(&path).ok()?.filter_map(|d| {
        let path = d.ok()?.path();
        if path.extension()? == *THUMB_FORMAT {
            return Some(path);
        }
        None
    });

    let mut rng = rand::thread_rng();
    Some(thumbs.choose(&mut rng)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_file_path_to_thumb() {
        let init = PathBuf::from("/dir/file.jpg");
        let result = Ok(PathBuf::from("/dir/.th/file.jpg.avif"));
        assert_eq!(file_path_to_thumb(&init), result, "Basic image path");

        let init = PathBuf::from("/Pictures/Special Photos!/file@example.com/video.avi");
        let result = Ok(PathBuf::from(
            "/Pictures/Special Photos!/file@example.com/.th/video.avi.avif",
        ));
        assert_eq!(file_path_to_thumb(&init), result, "Uncommon video path");

        let init = PathBuf::from("/home/user/Pictures/sub/../base.jpg");
        let result = Ok(PathBuf::from(
            "/home/user/Pictures/sub/../.th/base.jpg.avif",
        ));
        assert_eq!(file_path_to_thumb(&init), result, "Parent in path");

        let init = PathBuf::from("/etc/config_file");
        let result = Ok(PathBuf::from("/etc/.th/config_file.avif"));
        assert_eq!(file_path_to_thumb(&init), result, "Path without file ext");
    }

    #[test]
    fn test_get_random_thumb() {
        let dir = PathBuf::from(format!("{}imgs/.th", &DIR.to_str().unwrap()));
        let thumb = get_random_thumb(&dir);
        assert!(thumb.is_some());
    }
}
