#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate lazy_static;

use std::io::Error as ioError;
use rocket::config::Config as RocketConfig;
use rocket_contrib::templates::Template;

mod app;
use app::config::{CFG, BASEPATH};

fn main() -> Result<(), ioError> {
    let mut rocket_conf = RocketConfig::active().unwrap();
    rocket_conf.set_address(CFG.host.as_str()).expect("Unable to bind to host provided");
    rocket_conf.set_port(CFG.port);

    rocket::custom(rocket_conf)
        .attach(Template::fairing())
        .mount(*BASEPATH, routes![app::route]).launch();

    Ok(())
}
