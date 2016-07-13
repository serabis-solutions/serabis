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
extern crate serde_json;
extern crate config_loader;
extern crate eventual;

mod config;
mod plugin;
mod client;
mod metric;
mod cache_poller;

use std::sync::Arc;
use std::path::Path;
use std::thread;
use std::time::Duration;
use eventual::Async;

use config_loader::Loader;

const CONFIG_PATH: &'static str = "/etc/serapis/agent.toml";
const PLUGIN_PATH: &'static str = "/etc/serapis/plugins";

#[cfg(feature = "client_cache_tmp")]
pub const CLIENT_CACHE_DIR : &'static str = "/tmp/serapis";

//XXX make this work!!!!
#[cfg(not(feature = "client_cache_tmp"))]
pub const CLIENT_CACHE_DIR : &'static str = "/var/cache/serapis";

fn main() {
    env_logger::init().unwrap();

    info!( "loading agent config {}", &CONFIG_PATH );
    let agent_config = match config::AgentConfig::new_from_file( Path::new( &CONFIG_PATH ) ) {
        Ok(v)  => Arc::new( v ), //Arc because threads
        Err(e) => die!("{}", e ),
    };
    trace!( "{:?}", &agent_config );

    let client = Arc::new( client::Client::new( agent_config.clone() ) );

    let plugins = match plugin::load_all( Path::new( &PLUGIN_PATH ) ) {
        Ok(v)  => v,
        Err(e) => die!("{}", e),
    };

    //this would be better if it was a BTreeMap so we can have the plugin name
    let mut futures: Vec<_> = plugins.into_iter().map(|p| {
        let client = client.clone();
        eventual::Future::spawn( || {
            p.run( client )
        } )
    }).collect();


    let cache_poller_future = {
        let client = client.clone();
        eventual::Future::spawn( || {
            let cache_poller = cache_poller::CachePoller::new( client );
            cache_poller.run()
        } )
    };

    futures.push( cache_poller_future );

    //we can't block on the first thread, because the last thread might have
    // errored. so poll instead
    let plugin_count = futures.len();
    loop {
        //this is because .poll takes ownership of the future, and we need to put it back in the
        //queue
        let mut new_futures : Vec<eventual::Future<_, _>> = Vec::with_capacity( plugin_count );
        info!("thread error checker running");
        for future in futures {
            match future.poll() {
                Ok(r)   => match r {
                    Ok(async_result)   => match async_result {
                        Ok(())             => (),
                        Err(error)  => die!( "{}", error ),
                    },
                    Err(async_error)  => die!( "async error {:?}", async_error ),
                },
                Err(future)  => new_futures.push( future ),
            };
        }
        futures = new_futures;

        //XXX how long should this be?
        thread::sleep( Duration::from_secs( 10 ) );
    }
}
