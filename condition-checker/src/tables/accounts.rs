use super::Table;
use postgres::Connection;
use std::rc::Rc;

//Table 
#[derive(Debug)]
pub struct Accounts {
   conn: Rc<Connection>
}

//Row
#[derive(Debug)]
pub struct Account {
    id: i32
}

impl Accounts {
    pub fn get_accounts(&self) -> Vec<Account> {
        let mut accounts = vec![];

        for row in self.conn.query("SELECT id FROM accounts", &[]).unwrap().iter() {
            accounts.push(Account {
                id: row.get(0),
            });
        }

        accounts
    }
}

impl Table for Accounts {
    fn new(conn: Rc<Connection>) -> Self {
        Accounts {
            conn: conn
        }
    }
}
