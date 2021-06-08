use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub server_url: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // read the local file only in development mode
        #[cfg(debug_assertions)]
        s.merge(File::with_name("assets/blog-server-config.ini"))?;

        #[cfg(not(debug_assertions))]
        s.merge(File::with_name("/usr/local/etc/blog-server-config.ini"))?;

        s.try_into()
    }
}
