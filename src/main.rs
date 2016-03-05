#![feature(plugin)]
#![plugin(clippy)]

#[macro_use] extern crate log;
#[macro_use] mod macros;

extern crate rustc_serialize;
extern crate toml;
extern crate hyper;

mod config;
mod worker;
mod server;

use worker::Worker;
use std::sync::Arc;
use std::thread;

fn main() {
    let client = Arc::new( hyper::Client::new() );

    let m = config::Monitor::parse( "/etc/serapis/monitor.toml" );

    let server = server::Server::new();
    println!( "{:?}", m );

    let workers = vec![
        Worker::new( "1", vec!["foo", "bar"], client.clone() ),
        Worker::new( "2", vec!["foo", "bar"], client.clone() ),
        Worker::new( "3", vec!["foo", "bar"], client.clone() ),
        Worker::new( "4", vec!["foo", "bar"], client.clone() ),
        Worker::new( "5", vec!["foo", "bar"], client.clone() ),
    ];

    let handles: Vec<_> = workers.into_iter().map(|p| {
        thread::spawn(move || {
            p.start();
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
