/**
 * "hrocket" -> "hview" rocket customizations
 * Includes custom responder, pathbuf, and other work arounds
 */

use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Result as ResponseResult, Responder, NamedFile};
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
    fn respond_to(self, req: &Request) -> ResponseResult<'a> {
        if self.file.is_some() {
            return self.file.unwrap().respond_to(&req);
        } else if self.tmpl.is_some() {
            return self.tmpl.unwrap().respond_to(&req);
        }

        Err(Status::NotFound)
    }
}
