#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use] extern crate log;
#[macro_use] mod macros;
#[macro_use] extern crate quick_error;

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

use config::Loader;

const CONFIG_PATH: &'static str = "/etc/serapis/agent.toml";
const PLUGIN_PATH: &'static str = "/etc/serapis/plugins";

fn main() {
    env_logger::init().unwrap();

    let agent_config = match config::AgentConfig::new_from_file( Path::new( &CONFIG_PATH ) ) {
        Ok(v)  => Arc::new( v ), //Arc because threads
        Err(e) => die!("{}", e ),
    };

    let client = Arc::new( client::Client::new( agent_config.clone() ) );

    let plugins = match plugin::load_all( Path::new( &PLUGIN_PATH ) ) {
        Ok(v)  => v,
        Err(e) => die!("{}", e),
    };

    let handles: Vec<_> = plugins.into_iter().map(|p| {
        let client: Arc<client::Client> = client.clone();
        match p.run( client ) {
            Ok(v)   => v,
            Err(e)  => die!("failed to spawn thread {}", e),
        }
    }).collect();

    for h in handles {
        //XXX should we try and restart the thread?

        //did the thread builder fail?
        match h.join() {
            Err(e) => die!( "failed to join thread {:?}", e ),
            Ok(v) => match v {
                Err(e) => die!( "plugin failed to run {}", e ),
                Ok(v) => v,
            }
        };
    }
}
