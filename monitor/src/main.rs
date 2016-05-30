#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use] extern crate log;
#[macro_use] mod macros;

extern crate toml;
#[macro_use] extern crate hyper;
#[macro_use] extern crate mime;
extern crate rand;
extern crate env_logger;
extern crate time;
extern crate pine;
extern crate serde;

mod config;
mod plugin;
mod client;

use std::sync::Arc;
use std::path::Path;

fn main() {
    env_logger::init().unwrap();

    //these are Arc because threads
    let config_path = "/etc/serapis/monitor.toml";
    info!("loading config {}", config_path );
    let monitor_config = Arc::new( config::Monitor::parse( config_path ) );

    let client = Arc::new( client::Client::new( monitor_config.clone() ) );

    let plugins = plugin::load_all( Path::new("/etc/serapis/plugins") );

    let handles: Vec<_> = plugins.into_iter().map(|p| {
        let client: Arc<client::Client> = client.clone();
        p.run( client )
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
