use std::fs::File;
use std::io::prelude::*;
use toml::{Parser, Value, Decoder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AgentConfig {
    pub account_key: String,
    pub agent_key: String,
    pub base_url: String,
}

impl AgentConfig {
    pub fn parse(path: &str) -> Self {
        let mut config_toml = String::new();

        let mut file = File::open(path).unwrap_or_else( |e| die!( "couldn't find config file, {}", e ) );

        file.read_to_string(&mut config_toml)
            .unwrap_or_else(|err| die!("Error while reading config: [{}]", err));

        let mut parser = Parser::new(&config_toml);
        let toml = parser.parse();

        if toml.is_none() {
            for err in &parser.errors {
                let (loline, locol) = parser.to_linecol(err.lo);
                let (hiline, hicol) = parser.to_linecol(err.hi);
                println!("{}:{}:{}-{}:{} error: {}",
                         path, loline, locol, hiline, hicol, err.desc);
            }
            die!("Exiting server");
        }

        let mut config = Decoder::new( Value::Table( toml.unwrap() ) );

        Deserialize::deserialize( &mut config ).expect("helpful error")
    }
}
