use r2d2_postgres::{SslMode, PostgresConnectionManager};
use r2d2::{Pool, Config};
use tables::*;

//Add table definitions here
pub struct Tables {
    pub accounts: Accounts,
    pub agents: Agents,
    pub conditions: Conditions,
}

pub struct Db {
    pub pool: Pool<PostgresConnectionManager>,
    pub tables: Tables
}

impl Db {
    pub fn new(uri: &str) -> Db {
        let config = Config::default();
        let manager = PostgresConnectionManager::new(uri, SslMode::None).unwrap();
        let pool = Pool::new(config, manager).unwrap();

        Db{
            tables: Tables{
                accounts: Accounts::new( pool.clone() ),
                agents: Agents::new( pool.clone() ),
                conditions: Conditions::new( pool.clone() ),
            },
            pool: pool,
        }
    }
}

