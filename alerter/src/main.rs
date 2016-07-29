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
extern crate lettre;

mod db;
mod tables;
mod error;

#[macro_use] mod macros;

use config_loader::Loader;
use std::path::Path;

use lettre::email::EmailBuilder;
use lettre::transport::smtp::{SmtpTransport, SmtpTransportBuilder};
use lettre::transport::EmailTransport;
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
struct MailConfig {
    host: String,
    username: String,
    password: String,
    port: u16
}

#[derive(Deserialize)]
struct Config {
    postgres_url: String,
    rabbitmq: RabbitMqConfig,
    mail: MailConfig,
}

impl Loader for Config {}


struct AlertConsumer {
    db: db::Db,
    mailer: SmtpTransport
}

impl amqp::Consumer for AlertConsumer {
    fn handle_delivery(&mut self, channel: &mut Channel, deliver: protocol::basic::Deliver, headers: protocol::basic::BasicProperties, body: Vec<u8>){
        let obj = match self.get_data_obj_from_msg_body(body) {
            Some(v) => v,
            None => return //we've already called error! 
        };
        info!("Alert Received: {:?}", obj);
        let agent_id = self.get_obj_value_as_i64(&obj, "agent_condition_id");
        self.process_alert(agent_id);
        channel.basic_ack(deliver.delivery_tag, false).expect("Error acknowledging message");
    }
}

impl AlertConsumer {
    fn process_alert(&mut self, agent_condition_id: i64) {
        let contacts = self.db.tables.conditions.get_trigger_contacts(agent_condition_id);


        for contact in &contacts {
            //XXX Need to do more than email 
            let email = EmailBuilder::new()
                .to(contact.email.as_str())
                .from("alerts@serabis.com")
                .subject(format!("SERABIS ALERT: {}", contact.hostname).as_str())
                .body(self.get_email_body(contact).as_str())
                .build();

            match email {
                Ok(v) => {
                    match self.mailer.send(v) {
                        Ok(v) => debug!("Email Send Result: {:?}", v),
                        Err(e) => info!("Failed to send email alert {:?}", e)
                    }
                },
                Err(e) => {
                    info!("Failed to generate email: {}", e);
                }
            }
        }
    }

    fn get_email_body(&self, contact: &tables::conditions::TriggerContact) -> String {
        if contact.triggered {
            return format!("SERABIS ALERT:\n\t{} on host {} has triggered.", contact.name, contact.hostname);
        } else {
            return format!("SERABIS ALERT NORMAL:\n\t{} on host {} has resolved.", contact.name, contact.hostname);
        }
    }

    fn get_data_obj_from_msg_body(&self, body: Vec<u8>) -> Option<std::collections::BTreeMap<String, serde_json::Value>> {
        let body_str = match String::from_utf8(body) {
            Ok(v) => v,
            Err(e) => {
                //XXX Should we somehow complain louder abour this?
                error!("Could not parse message body as utf string {}", e);
                return None;
            }
        };

        let deserialized_value: serde_json::Value = match serde_json::from_str(body_str.as_str()) {
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

    fn get_obj_value_as_i64(&self, obj: &std::collections::BTreeMap<std::string::String, serde_json::Value>, key: &str) -> i64 {
        match obj.get(key) {
            Some(v) => match v.as_i64() {
                Some(v) => v,
                None => {
                    error!("Failed to get {} from queued object", key);
                    0 //XXX we should raise an error here, not return 0.0
                }
            },
            None => {
                error!("Failed to get {} from queued object", key);
                0 //XXX we should raise an error here not return 0.0
            }
        }
    }


}

fn main() {
    env_logger::init().unwrap();

    const CONFIG_PATH: &'static str = "/etc/serabis/alerter.toml";

    info!( "loading alerter config {}", &CONFIG_PATH );
    let config = match Config::new_from_file( Path::new( &CONFIG_PATH ) ) {
        Ok(v)  => v,
        Err(e) => die!("{}", e ),
    };

    let db = match db::Db::new(&config.postgres_url) {
        Ok(v) => v,
        Err(e) => die!("{}", e),
    };

    let mailer = match SmtpTransportBuilder::new((
            config.mail.host.as_str(),
            config.mail.port)
        ) {
            Ok(v) => {
                v.credentials(&config.mail.username, &config.mail.password)
                .connection_reuse(true)
                .build()
            },
            Err(e) => panic!("Failed to initialise mailer: {:?}", e)
    };

    let mut channel = get_channel(&config);
    let queue_name = "alerter-consumer";
    let exchange_name = "exchange";

    //queue names
    let queue = channel.queue_declare(queue_name, false, true, false, false, false, Table::new());
    debug!("Openned queue: {:?}", queue);

    debug!("binding queue to exchange");
    let binding = channel.queue_bind( queue_name, exchange_name, "alert.new", false, Table::new() );
    debug!( "binding result {:?}", binding );

    channel.basic_prefetch(10).expect("Failed to prefetch");
    //consumer, queue: &str, consumer_tag: &str, no_local: bool, no_ack: bool, exclusive: bool, nowait: bool, arguments: Table
    info!("Declaring consumers...");

    let alert_consumer = AlertConsumer { db: db, mailer: mailer };
    let consumer_name = channel.basic_consume( alert_consumer, queue_name, "", false, false, false, false, Table::new());

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


