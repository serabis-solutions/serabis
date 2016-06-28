use super::Loader;

#[derive(Deserialize, Debug)]
pub struct AgentConfig {
    pub account_key: String,
    pub agent_key: String,
    pub base_url: String,
}

impl Loader for AgentConfig {}
