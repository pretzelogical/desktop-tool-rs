// use clap::Parser;
// use serde::{Serialize, Deserialize};
// use toml;


fn main() {
    let config = Config::build();
    println!("search_dirs = {:#?}", config.config_file.search_dirs);
    println!("target = {}", config.target);
}

#[derive(Debug)]
struct Config {
    config_file: Box<ConfigFile>,
    target: String,
}

impl Config {
    fn build() -> Config {
        Config {
            config_file: Box::new(ConfigFile::build()),
            target: String::from("pgadmin4")
        }
    }
}

#[derive(Debug)]
struct ConfigFile {
    search_dirs: Vec<String>
}

impl ConfigFile {
    fn build() -> ConfigFile {
        ConfigFile {
            search_dirs: vec![String::from("/usr/bin/applications")]
        }
    }
}