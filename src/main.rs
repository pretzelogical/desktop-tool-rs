// use clap::Parser;
// use serde::{Serialize, Deserialize};
// use toml;


fn main() {
    let config = dtconfig::Config::build();
    match config.target {
        Some(target) => println!("target = {target}"),
        _ => ()
    }
    match config.search_dirs {
        Some(search_dirs) => println!("search_dirs = {:?}", search_dirs),
        _ => ()
    }
}

mod dtconfig {
    use merge::Merge;

    #[derive(Debug, Clone, Merge)]
    pub struct Config {
        pub target: Option<String>,
        pub search_dirs: Option<Vec<String>>
    }
    
    impl Config {
        pub fn build() -> Config {
    
            Config {
                target: Some(String::from("pgadmin4")),
                search_dirs: Some(vec![String::from("/usr/share/applications")])
            }
        }
    }
}