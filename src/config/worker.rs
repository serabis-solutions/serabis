//stolen from https://github.com/polyfractal/cormorant/blob/master/src/config.rs

use std::fs::File;
use std::io::prelude::*;
#[allow(unused_imports)]
use toml::{Parser, Value, Decoder};
use toml;

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Monitor {
    pub id : String,
    pub key: String,
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Something {
    pub foo: String,
}

impl Monitor {
    /// Returns a default configuration if we don't have/find a
    /// config file
    pub fn new() -> Monitor {
        Monitor {
            something: Something::new(),
        }
    }

    /// Attempt to load and parse the config file into our Monitor struct.
    /// If a file cannot be found, return a default Monitor.
    /// If we find a file but cannot parse it, panic
    pub fn parse(path: &str) -> Monitor {
        let mut config_toml = String::new();

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_)   => {
                warn!("Could not find config file, using default!");
                return Monitor::new();
            }
        };

        file.read_to_string(&mut config_toml)
                .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

        let mut parser = Parser::new(&config_toml);
        let toml = parser.parse();

        if toml.is_none() {
            for err in &parser.errors {
                let (loline, locol) = parser.to_linecol(err.lo);
                let (hiline, hicol) = parser.to_linecol(err.hi);
                println!("{}:{}:{}-{}:{} error: {}",
                         path, loline, locol, hiline, hicol, err.desc);
            }
            panic!("Exiting server");
        }

        let config = Value::Table( toml.unwrap() );
        toml::decode(config).unwrap_or_else( || die!("invalid config") )
    }
}

impl Something {
    pub fn new() -> Something {
        Something {
            foo : "DEFAULT VAKUE".to_owned(),
        }
    }
}
