extern crate postgres;
#[macro_use] extern crate quick_error;

use postgres::{Connection, SslMode};
use tables::*;
use db::*;

mod db;
mod tables;

struct Agent {
    id: i32,
    hostname: String,
}

fn main() {

    let db = db::Db::new();
    println!("{:?}", db.tables.accounts.get_accounts());
    println!("Hello, world!");
}
