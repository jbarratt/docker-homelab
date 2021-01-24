use std::error::Error;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub listen_port: Option<u32>,
    pub systems: Vec<System>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct System {
    pub host: Option<String>,
    pub url: Option<String>,
    pub sn: Option<String>,
}

impl Config {
    pub fn from_file(file: &str) -> Result<Config, Box<Error>> {
        let mut f = File::open(file)?;
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);
        let config: Config = toml::from_str(&s)?;
        Ok(config)
    }
}
