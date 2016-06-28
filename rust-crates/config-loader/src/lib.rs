#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate toml;
#[macro_use] extern crate quick_error;

extern crate serde;

pub mod loader;
pub use loader::*;

#[cfg(test)]
mod tests {
    use std::path::Path;
    pub use loader::*;

    //Let's use our own Cargo.toml for testing
    const CONFIG_PATH: &'static str = "./Cargo.toml";

    pub type AliasTable = ::toml::Table;

    #[derive(Deserialize, Debug)]
    pub struct TestConfig {
        package: AliasTable,        
    }

    impl ::Loader for TestConfig {}

    #[test]
    fn load_config() {
        let test_config = match TestConfig::new_from_file( Path::new( &CONFIG_PATH ) ) {   
            Ok(v) => v,
            Err(e) => panic!("Error loading config {}", e),
        };
        
        assert_eq!(test_config.package.get("name").unwrap().as_str().unwrap(), "config-loader");
    }
}
