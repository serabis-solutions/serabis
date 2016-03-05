//stolen from https://github.com/polyfractal/cormorant/blob/master/src/config.rs

use std::fs::File;
use std::io::prelude::*;
#[allow(unused_imports)]
use toml::{Parser, Value, Decoder};
use toml;

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Monitor {
    pub account_id: String,
    pub device_key: String,
}

impl Monitor {
    pub fn parse(path: &str) -> Monitor {
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

        let config = Value::Table( toml.unwrap() );
        toml::decode(config).unwrap_or_else( || die!("invalid config") )
    }
}
