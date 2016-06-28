pub mod agent;
pub use self::agent::AgentConfig;

pub mod plugin;
pub use self::plugin::PluginConfig;

use serde::Deserializer;

pub trait CustomDefault {
    fn default() -> Self;
}
pub trait CustomDeserialize : Sized {
    fn deserialize<D>(de: &mut D) -> Result<Self, D::Error>
        where D: Deserializer;
}

