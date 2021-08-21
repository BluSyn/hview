/**
 * "hrocket" -> "hview" rocket customizations
 * Includes custom responder, pathbuf, and other work arounds
 */
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Result as ResponseResult};
use rocket_dyn_templates::Template;

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
impl<'r> Responder<'r, 'static> for CustomResponder {
    fn respond_to(self, req: &'r Request) -> ResponseResult<'static> {
        if self.file.is_some() {
            return self.file.unwrap().respond_to(&req);
        } else if self.tmpl.is_some() {
            return self.tmpl.unwrap().respond_to(&req);
        }

        Err(Status::NotFound)
    }
}
