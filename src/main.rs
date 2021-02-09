#![feature(decl_macro)]

use std::io::Error as ioError;
use rocket::routes;
use rocket::config::Config as RocketConfig;
use rocket_contrib::templates::Template;

mod app;
mod config;
use config::{CFG, BASEPATH};

fn main() -> Result<(), ioError> {
    let mut conf = RocketConfig::active().unwrap();
    conf.set_address(CFG.host.as_str()).expect("Unable to bind to host provided");
    conf.set_port(CFG.port);

    rocket::custom(conf)
        .attach(Template::fairing())
        .mount(*BASEPATH, routes![app::home, app::route]).launch();

    Ok(())
}
