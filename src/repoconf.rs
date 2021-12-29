use config::{Config, ConfigError, File};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "connection")]
#[serde(rename_all = "camelCase")]
pub enum RepoConf {
    Ssh {
        directory: Option<String>,
        user: Option<String>,
        logfile: Option<String>
    },
    Http {
        directory: Option<String>,
        user: Option<String>,
        token: Option<String>,
        logfile: Option<String>
    }
}

impl RepoConf {
    pub fn new() -> Result<HashMap<String, RepoConf>, ConfigError> {
        let mut c = Config::default();
        c.merge(File::with_name("config").format(config::FileFormat::Yaml))?;

        let repos = c.get::<HashMap<String,RepoConf>>("repos")?;

        Ok(repos)
    }   
}