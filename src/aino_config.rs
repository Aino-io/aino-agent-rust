use crate::AinoError;
use config::{Config, Environment, File};
use std::env;

/// The configuration needed for the [`Aino.io`](https://aino.io) agent.
#[derive(Deserialize, Debug, Clone)]
pub struct AinoConfig {
    /// `Aino.io` API URL. Should be https://data.aino.io/rest/v2/transaction.
    pub url: String,

    /// Your API key. Can be obtained from the API Access tab in the application.
    #[serde(alias = "apiKey")]
    pub api_key: String,

    /// The interval for the agent to send a batch of [`Transaction`](struct.Transaction.html)s.
    #[serde(alias = "sendInterval")]
    pub send_interval: u32,
}

impl AinoConfig {
    /// Reads in the configuration files and environment variables and constructs the configuration object.
    pub fn new() -> Result<Self, AinoError> {
        let mut config = Config::new();
        let env = env::var("RUN_MODE").unwrap_or("development".into());

        config
            .merge(File::with_name("config/default"))
            .map_err(|err| AinoError::new(err.to_string()))?;
        config
            .merge(File::with_name(&format!("config/{}", env)).required(false))
            .map_err(|err| AinoError::new(err.to_string()))?;
        config
            .merge(File::with_name("config/local").required(false))
            .map_err(|err| AinoError::new(err.to_string()))?;
        config
            .merge(Environment::with_prefix("aino"))
            .map_err(|err| AinoError::new(err.to_string()))?;

        config
            .try_into()
            .map_err(|err| AinoError::new(err.to_string()))
    }
}
