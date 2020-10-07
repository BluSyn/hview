#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

use std::io;

use rocket::config::Config as RocketConfig;
use rocket_contrib::templates::Template;

mod hview;
use hview::config::{CFG, BASEPATH};

fn main() -> Result<(), io::Error> {
    let mut rocket_conf = RocketConfig::active().unwrap();
    rocket_conf.set_address(CFG.host.as_str()).expect("Unable to bind to host provided");
    rocket_conf.set_port(CFG.port);

    rocket::custom(rocket_conf)
        .attach(Template::fairing())
        .mount(*BASEPATH, routes![hview::route]).launch();

    Ok(())
}
