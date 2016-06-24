use super::Table;
use postgres::Connection;
use std::rc::Rc;

//Table 
#[derive(Debug)]
pub struct Agents {
   conn: Rc<Connection>,
}

//Row
#[derive(Debug)]
pub struct Agent {
    id: i32
}

impl Agents {
    pub fn get_agents(&self) -> Vec<Agent> {
        let mut agents = vec![];

        for row in self.conn.query("SELECT id FROM agents", &[]).unwrap().iter() {
            agents.push(Agent {
                id: row.get(0),
            });
        }

        agents
    }
}

impl Table for Agents {
    fn new(conn: Rc<Connection>) -> Self {
        Agents {
            conn: conn
        }
    }
}
