/**
 * "hrocket" -> "hview" rocket customizations
 * Includes custom responder, pathbuf, and other work arounds
 */
use std::path::PathBuf;
use rocket::http::Status;
use rocket::http::uri::{Uri, Segments, SegmentError};
use rocket::request::{Request, FromSegments};
use rocket::response::{self, Responder, NamedFile};
use rocket_contrib::templates::Template;

pub struct CustomResponder {
    pub file: Option<NamedFile>,
    pub tmpl: Option<Template>
}

impl CustomResponder {
    pub fn new() -> Self {
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


/*
*
* Override of FromSegements trait on std::path::PathBuf
* https://github.com/SergioBenitez/Rocket/blob/master/lib/src/request/param.rs
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
pub struct CustomPathBuf {
    path: PathBuf
}

impl CustomPathBuf {
    pub fn new() -> Self {
        Self {
            path: PathBuf::new(),
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
            let decoded = Uri::percent_decode(segment.as_bytes())
                .map_err(|e| SegmentError::Utf8(e))?;

            if decoded == ".." {
                path.pop();
            }
            else if decoded.starts_with('*') {
                return Err(SegmentError::BadStart('*'))
            } else if decoded.ends_with(':') {
                return Err(SegmentError::BadEnd(':'))
            } else if decoded.ends_with('>') {
                return Err(SegmentError::BadEnd('>'))
            } else if decoded.ends_with('<') {
                return Err(SegmentError::BadEnd('<'))
            } else if decoded.contains('/') {
                return Err(SegmentError::BadChar('/'))
            } else if cfg!(windows) && decoded.contains('\\') {
                return Err(SegmentError::BadChar('\\'))
            } else {
                path.push(&*decoded)
            }
        }

        Ok(path)
    }
}
