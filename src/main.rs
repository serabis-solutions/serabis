#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use] extern crate log;
#[macro_use] mod macros;

extern crate rustc_serialize;
extern crate toml;
extern crate hyper;

//mod config;
mod plugin;
//mod server;

use std::sync::Arc;
use std::thread;
use std::path::Path;

fn main() {
    let plugin_path = Path::new("/etc/serapis/plugins");
    let plugins = plugin::find_plugins( &plugin_path );

//    let m = config::Monitor::parse( "/etc/serapis/monitor.toml" );

//    let server = server::Server::new();

    let client = Arc::new( hyper::Client::new() );

    let handles: Vec<_> = plugins.into_iter().map(|p| {
        let client = client.clone();
        thread::spawn(move || {
            p.run( client );
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
