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
    // use regex::Regex;
    use std::env;

    #[derive(Debug, Clone, Merge)]
    pub struct Config {
        pub target: Option<String>,
        pub search_dirs: Option<Vec<String>>
    }

    impl Default for Config {
        fn default() -> Config {
            Config {
                target: Some(String::from("pgadmin4")),
                search_dirs: Some(vec![String::from("/usr/share/applications")])
            }
        }
    }
    
    impl Config {
        pub fn build() -> Config {
            let cli_config = Config::from_cli();
            // let file_config = Config::from_file();

            println!("cli config = {:?}", cli_config);

            Config {
                target: Some(String::from("pgadmin4")),
                search_dirs: Some(vec![String::from("/usr/share/applications")])
            }
        }


        fn from_cli() -> Config {
            let args = env::args().skip(1);
            let mut config = Config { ..Default::default() };
            let mut search_dirs: Vec<String> = Vec::new();

            let mut is_option = false;
            let mut option_arg = String::new();
            for arg in args {
                if arg.starts_with("--") {
                    is_option = true;
                    option_arg = arg.clone();
                    continue;
                }
                if is_option {
                    println!("option arg = {option_arg}");
                    match option_arg.as_str() {
                        "--search-dirs" => search_dirs.push(arg.clone()),
                        _ => println!("Error parsing option arguments")
                    }
                    is_option = false;
                }

                // Is the target
                config.target = Some(arg.clone());
            }
            config.search_dirs = Some(search_dirs);
            config
        }
    }
}