/**
 * "hrocket" -> "hview" rocket customizations
 * Includes custom responder, pathbuf, and other work arounds
 */
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{NamedFile, Responder, Result as ResponseResult};
use rocket_contrib::templates::Template;

pub struct CustomResponder {
    pub file: Option<NamedFile>,
    pub tmpl: Option<Template>,
}

impl CustomResponder {
    pub fn new() -> Self {
        Self {
            file: None,
            tmpl: None,
        }
    }
}

// Response flow:
// If path correspondes to static file, return raw static file
// otherwise return template if defined
// otherwise return 404
impl<'a> Responder<'a> for CustomResponder {
    fn respond_to(self, req: &Request) -> ResponseResult<'a> {
        if self.file.is_some() {
            return self.file.unwrap().respond_to(&req);
        } else if self.tmpl.is_some() {
            return self.tmpl.unwrap().respond_to(&req);
        }

        Err(Status::NotFound)
    }
}
