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

    match std::fs::metadata(&path) {
        Ok(meta) => {
            if meta.is_file() {
                let file = NamedFile::open(path).unwrap();
                Ok(Either::A(file.use_last_modified(true)))
            } else {
                // Temporary: profile this function call
                let now = Instant::now();
                match get_dir(&path) {
                    Ok(dir) => {
                        println!("Time elapsed {}ms", now.elapsed().as_micros());
                        Ok(Either::B(Json(dir)))
                    }
                    Err(_) => Err(ErrorNotFound("Dir does not exist")),
                }
            }
        }
        Err(_) => Err(ErrorNotFound("Path Not Found")),
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
