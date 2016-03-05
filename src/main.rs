#![feature(plugin)]
#![plugin(clippy)]

#[macro_use] extern crate log;
#[macro_use] mod macros;

extern crate rustc_serialize;
extern crate toml;
extern crate hyper;

mod config;
mod worker;
use worker::Worker;

mod server;

use std::thread;

fn main() {
    let m = config::Monitor::parse( "/etc/serapis/monitor.toml" );

    let server = server::Server::new();
    println!( "{:?}", m );

    let workers = vec![
        Worker::new( "echo", vec!["foo", "bar"]),
        Worker::new( "echo", vec!["foo", "bar"]),
        Worker::new( "echo", vec!["foo", "bar"]),
        Worker::new( "echo", vec!["foo", "bar"]),
        Worker::new( "echo", vec!["foo", "bar"]),
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
