use super::Table;
use r2d2_postgres::{PostgresConnectionManager};
use r2d2::{Pool};

//Table 
#[derive(Debug)]
pub struct Agents {
   pool: Pool<PostgresConnectionManager>,
}

//Row
#[derive(Debug)]
pub struct Agent {
    id: i32
}

impl Agents {
    pub fn get_agents(&self) -> Vec<Agent> {
        let mut agents = vec![];
        let conn = self.pool.clone().get().unwrap();

        for row in conn.query("SELECT id FROM agents", &[]).unwrap().iter() {
            agents.push(Agent {
                id: row.get(0),
            });
        }

        agents
    }
}

impl Table for Agents {
    fn new(pool: Pool<PostgresConnectionManager>) -> Self {
        Agents {
            pool: pool
        }
    }
}
