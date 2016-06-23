use postgres::Connection;
pub mod accounts;
pub mod agents;
pub mod conditions;

pub use self::accounts::*;
pub use self::agents::*;
pub use self::conditions::*;

pub trait Table<'a> {
    fn new(conn: &'a Connection) -> Self;
}
