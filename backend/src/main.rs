#[macro_use]
extern crate rocket;

use rocket::config::Config as RocketConfig;
use rocket::routes;
use std::net::IpAddr;

mod app;
mod config;
use config::{BASEPATH, CFG};

#[launch]
fn rocket() -> _ {
    let conf = RocketConfig::figment()
        .merge((
            "address",
            CFG.host
                .parse::<IpAddr>()
                .expect("Invalid bind IP configured"),
        ))
        .merge(("port", CFG.port));

    rocket::custom(conf).mount(*BASEPATH, routes![app::home, app::route])
}
