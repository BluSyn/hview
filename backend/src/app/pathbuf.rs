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
use rocket::http::uri::{error::PathError, fmt::Path, Segments};
use rocket::request::FromSegments;
use std::path::PathBuf;

pub struct CustomPathBuf {
    path: PathBuf,
}

impl CustomPathBuf {
    pub fn from(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn from_str(path: &str) -> Self {
        Self {
            path: PathBuf::from(path),
        }
    }

    pub fn path(&self) -> PathBuf {
        self.path.to_path_buf()
    }
}

impl FromSegments<'_> for CustomPathBuf {
    type Error = PathError;

    fn from_segments(segments: Segments<'_, Path>) -> Result<Self, Self::Error> {
        // Convert PathBuf -> CustomPathBuf
        match segments.to_path_buf(true) {
            Ok(path) => Ok(CustomPathBuf::from(path)),
            Err(e) => Err(e),
        }
    }
}
