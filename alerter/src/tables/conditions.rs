use super::Table;
use r2d2_postgres::{PostgresConnectionManager};
use r2d2::{Pool};

//Table 
#[derive(Debug)]
pub struct Conditions {
   pool: Pool<PostgresConnectionManager>,
}

#[derive(Debug)]
pub struct TriggerContact {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub triggered: bool,
    pub hostname: String
}

impl Conditions {
    pub fn get_trigger_contacts(&self, id: i64) -> Vec<TriggerContact> {
        let iid = id as i32;

        let mut contacts = vec![]; 

        let conn = self.pool.clone().get().unwrap();

        for row in conn.query("
                SELECT ac.id, c.name, ct.email, ac.triggered, a.hostname
                FROM agent_conditions ac  
                INNER JOIN conditions c on (ac.condition_id = c.id)  
                INNER JOIN contacts ct on (ct.account_id = c.account_id)
                INNER JOIN agents a on (ac.agent_id = a.id)
                WHERE ac.id = $1",
                &[&iid]
            ).unwrap().iter() {
                contacts.push(
                    TriggerContact {
                        id: row.get(0),
                        email: row.get(2),
                        name: row.get(1),
                        triggered: row.get(3),
                        hostname: row.get(4)
                    }
                );
            }

        contacts
    }

}

impl Table for Conditions {
    fn new(pool: Pool<PostgresConnectionManager>) -> Self {
        Conditions {
            pool: pool
        }
    }
}
