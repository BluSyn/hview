/**
*
* Override of FromSegements trait on std::path::PathBuf
* https://github.com/SergioBenitez/Rocket/blob/master/core/lib/src/request/param.rs
* -> line 328
*
* We need to accept hidden files (".dotfiles") in given routes, which Rocket doesn't allow.
*
* Workaround to override in Rust :
* Wrap the wanted object into an other one.
*
* https://github.com/SergioBenitez/Rocket/issues/560
*
*/
use rocket::http::uri::{SegmentError, Segments, Uri};
use rocket::request::FromSegments;
use std::path::PathBuf;

pub struct CustomPathBuf {
    path: PathBuf,
}

impl CustomPathBuf {
    pub fn new() -> Self {
        Self {
            path: PathBuf::new(),
        }
    }

    pub fn from(path: &str) -> Self {
        Self {
            path: PathBuf::from(path),
        }
    }

    pub fn push(&mut self, suffix: &str) {
        self.path.push(suffix);
    }

    pub fn pop(&mut self) {
        self.path.pop();
    }

    pub fn path(&self) -> PathBuf {
        self.path.to_path_buf()
    }
}

impl<'a> FromSegments<'a> for CustomPathBuf {
    type Error = SegmentError;

    fn from_segments(segments: Segments<'a>) -> Result<CustomPathBuf, SegmentError> {
        let mut path = CustomPathBuf::new();

        for segment in segments {
            let decoded =
                Uri::percent_decode(segment.as_bytes()).map_err(|e| SegmentError::Utf8(e))?;

            if decoded == ".." {
                path.pop();
            } else if decoded.starts_with('*') {
                return Err(SegmentError::BadStart('*'));
            } else if decoded.ends_with(':') {
                return Err(SegmentError::BadEnd(':'));
            } else if decoded.ends_with('>') {
                return Err(SegmentError::BadEnd('>'));
            } else if decoded.ends_with('<') {
                return Err(SegmentError::BadEnd('<'));
            } else if decoded.contains('/') {
                return Err(SegmentError::BadChar('/'));
            } else if cfg!(windows) && decoded.contains('\\') {
                return Err(SegmentError::BadChar('\\'));
            } else {
                path.push(&*decoded)
            }
        }

        Ok(path)
    }
}
