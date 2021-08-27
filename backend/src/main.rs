mod config;
use config::{CFG, DIR};

mod dir;
use dir::{get_dir, Dir};

use std::net::IpAddr;
use std::path::PathBuf;
use std::time::Instant;

use actix_files::NamedFile;
use actix_web::{
    error::ErrorNotFound, get, web::Json, App, Either, Error, HttpRequest, HttpServer,
};

// type FileOrJson = Either<NamedFile, Result<Json<Temp>, Error>>;
type FileOrJson = Result<Either<NamedFile, Json<Dir>>, Error>;

#[get("/{file:.*}")]
async fn route(req: HttpRequest) -> FileOrJson {
    let file: PathBuf = req.match_info().query("file").parse().unwrap();
    let path: PathBuf = DIR.join(file);

    if let Ok(meta) = std::fs::metadata(&path) {
        if meta.is_file() {
            if let Ok(file) = NamedFile::open(path) {
                Ok(Either::A(file.use_last_modified(true)))
            } else {
                Err(ErrorNotFound("File Not Found"))
            }
        } else {
            // Temporary: profile this function call
            let now = Instant::now();
            if let Ok(dir) = get_dir(&path) {
                println!("Time elapsed {}ms", now.elapsed().as_micros());
                Ok(Either::B(Json(dir)))
            } else {
                Err(ErrorNotFound("DIR Not Found"))
            }
        }
    } else {
        Err(ErrorNotFound("Path Not Found"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(route))
        .bind(format!(
            "{}:{}",
            CFG.host
                .parse::<IpAddr>()
                .expect("Invalid bind IP configured"),
            &CFG.port
        ))?
        .run()
        .await
}
