use r2d2_postgres::{SslMode, PostgresConnectionManager};
use r2d2::{Pool, Config};
use tables::*;
use error::CCError;

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
    pub fn new(uri: &str) -> Result<Db, CCError> {
        let config = Config::default();
        let manager = try!(PostgresConnectionManager::new(uri, SslMode::None));
        let pool = try!(Pool::new(config, manager));

        Ok(Db{
            tables: Tables{
                accounts: Accounts::new( pool.clone() ),
                agents: Agents::new( pool.clone() ),
                conditions: Conditions::new( pool.clone() ),
            },
            pool: pool,
        })
    }
}

