use super::Table;
use postgres::Connection;

//Table 
#[derive(Debug)]
pub struct Accounts<'a> {
   conn: &'a Connection
}

//Row
#[derive(Debug)]
pub struct Account {
    id: i32
}

impl<'a> Accounts<'a> {
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

impl<'a> Table<'a> for Accounts<'a> {
    fn new(conn: &'a Connection) -> Self {
        Accounts {
            conn: conn
        }
    }
}
