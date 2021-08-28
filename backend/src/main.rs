mod config;
use config::{CFG, DIR};

mod dir;
use dir::{get_dir, Dir};

use std::net::IpAddr;
use std::path::PathBuf;
use std::time::Instant;

use actix_files::NamedFile;
use actix_web::{
    error::ErrorNotFound, get, middleware, web::Json, App, Either, Error, HttpRequest, HttpServer,
};

#[get("/{file:.*}")]
async fn route(req: HttpRequest) -> Result<Either<NamedFile, Json<Dir>>, Error> {
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
                println!("Time elapsed {}s", now.elapsed().as_secs_f64());
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
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(route)
    })
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
