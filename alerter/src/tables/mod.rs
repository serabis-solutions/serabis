pub mod conditions;

pub use self::conditions::*;
use r2d2_postgres::PostgresConnectionManager;
use r2d2::Pool;

pub trait Table {
    fn new(conn: Pool<PostgresConnectionManager>) -> Self;
}
