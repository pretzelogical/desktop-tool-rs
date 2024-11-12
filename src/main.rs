mod dt_config;
use std::panic::panic_any;

use dt_config::Config;


fn main() {
    let config = Config::build();
    println!("final config = {:?}", config);
    search_for_file(&config);
}

fn search_for_file(config: &Config) {
    for path in &config.search_dirs {
        let dir = std::fs::read_dir(path)
            .expect("Error reading path");

        for file in dir {
            match file {
                Ok(file) => println!("{:?}", file),
                Err(error) => panic_any(error.to_string())
            }
        }
    }
}