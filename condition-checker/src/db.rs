use postgres::{Connection, SslMode};
use tables::*;
use quick_error::ResultExt;

//Add table definitions here
pub struct Tables<'a> {
    pub accounts: Accounts<'a>,
    pub agents: Agents<'a>,
}

pub struct Db<'a> {
    pub conn: Connection,
    pub tables: Tables<'a>
}

impl<'a> Db<'a> {
    pub fn new(connection: Connection) -> Db<'a> {
        let connectiontwo = Connection::connect("postgres://serapis:reallysecure@localhost:5432/serapis_dev", SslMode::None).unwrap(); 
        let conn = &connectiontwo;
        Db{
            conn: connectiontwo,
            tables: Tables{
                accounts: Accounts::new(conn),
                agents: Agents::new(conn),
            }
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

