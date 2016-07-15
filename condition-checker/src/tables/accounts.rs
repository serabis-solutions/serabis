use super::Table;
use r2d2_postgres::{PostgresConnectionManager};
use r2d2::{Pool};

//Table 
#[derive(Debug)]
pub struct Accounts {
   pool: Pool<PostgresConnectionManager>
}

//Row
#[derive(Debug)]
pub struct Account {
    id: i32
}

impl Accounts {
    pub fn get_accounts(&self) -> Vec<Account> {
        let mut accounts = vec![];
        let conn = self.pool.get().unwrap();
        for row in conn.query("SELECT id FROM accounts", &[]).unwrap().iter() {
            accounts.push(Account {
                id: row.get(0),
            });
        }
        accounts
    }
}

impl Table for Accounts {
    fn new(pool: Pool<PostgresConnectionManager>) -> Self {
        Accounts {
            pool: pool
        }
    }
}
