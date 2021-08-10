#![feature(decl_macro)]

use rocket::config::Config as RocketConfig;
use rocket::routes;
use rocket_contrib::templates::Template;
use std::io::Error as ioError;

mod app;
mod config;
use config::{BASEPATH, CFG};

fn main() -> Result<(), ioError> {
    let mut conf = RocketConfig::active().unwrap();
    conf.set_address(CFG.host.as_str())
        .expect("Unable to bind to host provided");
    conf.set_port(CFG.port);

    rocket::custom(conf)
        .attach(Template::fairing())
        .mount(*BASEPATH, routes![app::home, app::route])
        .launch();

    Ok(())
}
