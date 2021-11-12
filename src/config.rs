use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Entry {
    pub name: String,
    pub trigger: String,
    pub disposables: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub entries: Vec<Entry>,
}

pub fn read_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u = serde_yaml::from_reader(reader)?;

    Ok(u)
}
