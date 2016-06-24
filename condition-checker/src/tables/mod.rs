use postgres::Connection;
pub mod accounts;
pub mod agents;
pub mod conditions;

pub use self::accounts::*;
pub use self::agents::*;
pub use self::conditions::*;

use std::rc::Rc;

pub trait Table {
    fn new(conn: Rc<Connection>) -> Self;
}
