extern crate actix_web;
extern crate bytes;
extern crate reqwest;
extern crate serde_json;
extern crate toml;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;
#[macro_use]
extern crate serde_derive;

mod config;
mod handlers;
mod sonnen_reader;

use actix_web::{server, App};
use config::Config;
use handlers::{index, metrics};
use std::env;

static BUILD_TIME: Option<&'static str> = option_env!("BUILD_TIME");
static GIT_REVISION: Option<&'static str> = option_env!("GIT_REVISION");
static RUST_VERSION: Option<&'static str> = option_env!("RUST_VERSION");
static VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let version_info = if BUILD_TIME.is_some() {
        format!(
            "  version   : {}\n  revision  : {}\n  build time: {}\n",
            VERSION,
            GIT_REVISION.unwrap_or(""),
            BUILD_TIME.unwrap()
        )
    } else {
        format!("  version: {}\n", VERSION)
    };

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: sonnen-exporter [config_file]");
        println!("\n{}", version_info);
        return;
    }

    let config = match Config::from_file(&args[1]) {
        Ok(x) => x,
        Err(x) => {
            println!("Could not read '{}': {}", &args[1], x);
            return;
        }
    };

    let addr = format!("0.0.0.0:{}", config.listen_port.unwrap_or(9422));

    println!("Server started: {}", addr);

    server::new(move || {
        App::with_state(config.systems.clone())
            .resource("/", |r| r.f(index))
            .resource("/metrics", |r| r.f(metrics))
    }).bind(addr)
    .unwrap()
    .run();
}
