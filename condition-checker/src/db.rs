use postgres::{Connection, SslMode};
use tables::*;
use quick_error::ResultExt;
use std::rc::Rc;

//Add table definitions here
pub struct Tables {
    pub accounts: Accounts,
    pub agents: Agents,
}

pub struct Db {
    pub conn: Rc<Connection>,
    pub tables: Tables
}

impl Db {
    pub fn new() -> Db {
        let conn = Rc::new( Connection::connect("postgres://serapis:reallysecure@localhost:5432/serapis_dev", SslMode::None).unwrap() );

        Db{
            tables: Tables{
                accounts: Accounts::new( conn.clone() ),
                agents: Agents::new( conn.clone() ),
            },
            conn: conn,
        }

    }

}

quick_error! {
    #[derive(Debug)]
    pub enum TableLoadError {
        NoTableFound(err: String) {
            from()
            display( "{}", err )
        }
    }
}
