use super::Table;
use postgres::Connection;

//Table 
#[derive(Debug)]
pub struct Agents<'a> {
   conn: &'a Connection
}

//Row
#[derive(Debug)]
pub struct Agent {
    id: i32
}

impl<'a> Agents<'a> {
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

impl<'a> Table<'a> for Agents<'a> {
    fn new(conn: &'a Connection) -> Self {
        Agents {
            conn: conn
        }
    }
}
