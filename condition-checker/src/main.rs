#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use] extern crate log;
#[macro_use] extern crate quick_error;

extern crate env_logger;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
extern crate config_loader;
extern crate amqp;
extern crate serde_json;

use serde_json::Value;

mod db;
mod tables;
mod error;

#[macro_use] mod macros;

use config_loader::Loader;
use std::path::Path;

use amqp::{Session, Options, Table, Basic, protocol, Channel};
use std::default::Default;

#[derive(Deserialize)]
struct RabbitMqConfig {
    vhost: String,
    username: String,
    password: String,
    host: String
}
 
#[derive(Deserialize)]
struct Config {
    postgres_url: String,
    rabbitmq: RabbitMqConfig
}

impl Loader for Config {}


struct MetricConsumer {
    db: db::Db,
}

impl amqp::Consumer for MetricConsumer {
    fn handle_delivery(&mut self, channel: &mut Channel, deliver: protocol::basic::Deliver, headers: protocol::basic::BasicProperties, body: Vec<u8>){
        let obj = match self.get_data_obj_from_msg_body(body) {
            Some(v) => v,
            None => return //we've already called error! 
        };

        let components = self.db.tables.conditions.get_agent_condition_components(
            self.get_obj_value_as_string(&obj, "agent").as_str(),
            self.get_obj_value_as_string(&obj, "type").as_str(),
        );

        if ! components.is_empty() {
            self.check_condition(&obj, components, channel);
        }

        channel.basic_ack(deliver.delivery_tag, false).expect("Error acknowledging message");
    }
}

impl MetricConsumer {
    fn get_data_obj_from_msg_body(&self, body: Vec<u8>) -> Option<std::collections::BTreeMap<String, serde_json::Value>> {

        let body_str = match String::from_utf8(body) {
            Ok(v) => v,
            Err(e) => {
                //XXX Should we somehow complain louder abour this?
                error!("Could not parse message body as utf string {}", e);
                return None;
            }
        };

        let deserialized_value: Value = match serde_json::from_str(body_str.as_str()) {
            Ok(v) => v,
            Err(e) => {
                error!("Unable to deserialize JSON message {}", e);
                return None;
            }
        };

        let obj = match deserialized_value.as_object() {
            Some(v) => v,
            None => {
                error!("Unable to load deserialized JSON as an object");
                return None;
            }
        };

        Some(obj.to_owned())
    }
    
    fn get_obj_value_as_string(&self, obj: &std::collections::BTreeMap<std::string::String, serde_json::Value>, key: &str) -> String {
        match obj.get(key) {
            Some(v) => match v.as_string() {
                Some(v) => v.to_string(),
                None => {
                    error!("Failed to get {} from queued object", key);
                    "".to_string()
                }
            },
            None => {
                error!("Failed to get {} from queued object", key);
                "".to_string()
            }
        }
    }


    fn get_obj_value_as_f64(&self, obj: &std::collections::BTreeMap<std::string::String, serde_json::Value>, key: &str) -> f64 {
        match obj.get(key) {
            Some(v) => match v.as_f64() {
                Some(v) => v,
                None => {
                    error!("Failed to get {} from queued object", key);
                    0.0 //XXX we should raise an error here, not return 0.0
                }
            },
            None => {
                error!("Failed to get {} from queued object", key);
                0.0 //XXX we should raise an error here not return 0.0
            }
        }
    }

    fn check_condition(&self, metric: &std::collections::BTreeMap<std::string::String, serde_json::Value>, components: Vec<tables::conditions::AgentCondition>, channel: &mut Channel) {
        debug!("METRIC: {:?}", metric);
        let mut changed: bool = true; //set to true then if any haven't changed set to false
        let mut agent_condition_id = 0;

        for component in &components {
            agent_condition_id = component.agent_condition_id;
            changed = self.check_component(&metric, &component);
            if !self.check_component(&metric, &component) {
                changed = false;
            }
        }
    
        if changed && agent_condition_id > 0 {
            //All components changed
            self.update_check_trigger(agent_condition_id, );
            debug!("****ALERT TRIGGERED****");
            channel.basic_publish(
                "exchange", "alert.new", true, false, self.get_basic_properties(), format!("{{agent_condition_id: {} }}", agent_condition_id).to_string().into_bytes()
            ).expect("Failed publishing");
        }
    
    }

    fn get_basic_properties(&self) -> protocol::basic::BasicProperties {
        let headers = Table::new();
        protocol::basic::BasicProperties {
            content_type: Some("text".to_owned()),
            headers: Some(headers),
            ..Default::default()
        }    
    }

    fn update_check_trigger(&self, agent_condition_id: i32) {
        self.db.tables.conditions.update_check_trigger(agent_condition_id);
    }

    fn check_component(&self, metric: &std::collections::BTreeMap<std::string::String, serde_json::Value>, condition: &tables::conditions::AgentCondition) -> bool {
        let previous = condition.triggered;
        debug!("PREVIOUS: {:?}", previous);
        debug!("CONDITION: {:?}", condition);
        
        let obj = match metric.get("data") {
            Some(v) => {
                match v.as_object() {
                    Some(v) => v,
                    None => {
                        error!("Unable to load metric data");
                        return false;
                    }
                }
            },
            None => {
                error!("Unable to load metric data");
                return false;
            }
        };

        let value = self.get_obj_value_as_f64(obj, &condition.trigger_key);
        let trigger_value = match condition.trigger_value.parse() {
            Ok(v) => v,
            Err(e) => {
                error!("Unable to parse trigger value from condition {}", e);
                return false;
            }
        };

        debug!("{} {} {}", value, condition.operator.as_str(), trigger_value);
        let new_value: bool = match condition.operator.as_str() {
            ">=" => (value >= trigger_value),
            ">" => (value > trigger_value ),
            "<=" => (value <= trigger_value),
            "<" => (value < trigger_value ),
            "=" => (value == trigger_value),
            _ => false
        };

        new_value != previous
    }
}

fn main() {
    env_logger::init().unwrap();

    const CONFIG_PATH: &'static str = "/etc/serabis/condition-checker.toml";

    info!( "loading agent config {}", &CONFIG_PATH );
    let config = match Config::new_from_file( Path::new( &CONFIG_PATH ) ) {
        Ok(v)  => v,
        Err(e) => die!("{}", e ),
    };

    let db = match db::Db::new(&config.postgres_url) {
        Ok(v) => v,
        Err(e) => die!("{}", e),
    };

    println!("{:?}", db.tables.accounts.get_accounts());
    println!("{:?}", db.tables.agents.get_agents());

    let mut channel = get_channel(&config);
    let queue_name = "condition-check-consumer";
    let exchange_name = "exchange";

    //queue names
    let queue = channel.queue_declare(queue_name, false, true, false, false, false, Table::new());
    debug!("Openned queue: {:?}", queue);

    debug!("binding queue to exchange");
    let binding = channel.queue_bind( queue_name, exchange_name, "metric.new", false, Table::new() );
    debug!( "binding result {:?}", binding );

    channel.basic_prefetch(10).expect("Failed to prefetch");
    //consumer, queue: &str, consumer_tag: &str, no_local: bool, no_ack: bool, exclusive: bool, nowait: bool, arguments: Table
    info!("Declaring consumers...");

    let metric_consumer = MetricConsumer { db: db };
    let consumer_name = channel.basic_consume( metric_consumer, queue_name, "", false, false, false, false, Table::new());

    info!("Starting consumer {:?}", consumer_name);
    channel.start_consuming();

    channel.close(200, "Bye").expect("Failed to close channel");
}


fn get_channel( config: &Config ) -> Channel {
    let mut session = Session::new( Options{
        vhost: &config.rabbitmq.vhost,
        host : &config.rabbitmq.host,
        login: &config.rabbitmq.username,
        password: &config.rabbitmq.password,
        .. Default::default()
    }).expect("Can't create session");

    //XXX Should we do something other than die if we can't open a channel?
    let channel = session.open_channel(1).expect("Error openning channel 1");
    info!("Openned channel: {:?}", channel.id);
    channel
}


