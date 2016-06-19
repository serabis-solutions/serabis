use super::{Loader, CustomDefault, CustomDeserialize};
use serde::{Deserialize, Deserializer};
use std::time::Duration;
use std::path::PathBuf;

pub type Timeout = Duration;
impl CustomDefault for Timeout {
    fn default() -> Self {
        Duration::from_secs( 60 )
    }
}
impl CustomDeserialize for Timeout {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let deserialized : u64 = try!(Deserialize::deserialize(de));

        Ok( Timeout::from_secs( deserialized ) )
    }
}

pub type CommandPath = PathBuf;
impl CustomDeserialize for CommandPath {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let deserialized : String = try!(Deserialize::deserialize(de));

        Ok( CommandPath::from( deserialized ) )
    }
}

#[derive(Deserialize, Debug)]
pub struct PluginConfig {
    #[serde(default="CustomDefault::default", deserialize_with="CustomDeserialize::deserialize")]
    pub timeout  : Timeout,
    #[serde(deserialize_with="CustomDeserialize::deserialize")]
    pub command  : CommandPath,
}

impl Loader for PluginConfig {}
