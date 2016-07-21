use config_loader::Loader;

#[derive(Deserialize, Debug)]
pub struct AgentConfig {
    pub account_key : String,
    pub agent_key   : String,
    pub base_url    : String,
    pub htauth_user : Option<String>,
    pub htauth_pass : Option<String>,
}

impl Loader for AgentConfig {}
