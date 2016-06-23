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

    let conn = Connection::connect("postgres://serapis:reallysecure@localhost:5432/serapis_dev", SslMode::None).unwrap();
    let db = db::Db::new(conn);
    println!("{:?}", db.tables.accounts.get_accounts());
    println!("Hello, world!");
}
