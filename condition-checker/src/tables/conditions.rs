use super::Table;
use r2d2_postgres::{PostgresConnectionManager};
use r2d2::{Pool};

//Table 
#[derive(Debug)]
pub struct Conditions {
   pool: Pool<PostgresConnectionManager>,
}

#[derive(Debug)]
pub struct AgentCondition {
    pub condition_name: String,
    pub component_id: i32,
    pub agent_condition_id: i32,
    pub metric_type: String,
    pub trigger_value: String,
    pub trigger_key: String,
    pub triggered: bool,
    pub operator: String,
}

impl Conditions {
    pub fn update_check_trigger(&self, id: i32 ) {
        let conn = self.pool.clone().get().unwrap();
        conn.execute("UPDATE agent_conditions SET triggered = NOT triggered WHERE id = $1", &[&id]).unwrap();
    }

    pub fn get_agent_condition_components(&self, agent_key: &str, metric_type: &str) -> Vec<AgentCondition>{
        let mut agent_conditions = vec![];
        let conn = self.pool.clone().get().unwrap();

        for row in conn.query("
                SELECT c.name, cc.id, ac.id as ac_id, cc.type, cc.trigger_value, ac.triggered, cc.opperator, cc.trigger_key FROM conditions c 
                INNER JOIN condition_components cc ON (c.id = cc.condition_id) 
                INNER JOIN agent_conditions ac ON (ac.condition_id = c.id) 
                INNER JOIN agents a ON (ac.agent_id = a.id) 
                WHERE a.key = $1 AND cc.type = $2", &[&agent_key, &metric_type]
            ).unwrap().iter() {
            agent_conditions.push(
                AgentCondition {
                    condition_name: row.get(0),
                    component_id: row.get(1),
                    agent_condition_id: row.get(2),
                    metric_type: row.get(3),
                    trigger_value: row.get(4),
                    triggered: row.get(5),
                    operator: row.get(6),
                    trigger_key: row.get(7),
                }
            );
        }

        agent_conditions
    }
}

impl Table for Conditions {
    fn new(pool: Pool<PostgresConnectionManager>) -> Self {
        Conditions {
            pool: pool
        }
    }
}
