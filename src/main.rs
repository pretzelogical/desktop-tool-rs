// use serde::{Serialize, Deserialize};
// use toml;


fn main() {
    let config = dtconfig::Config::build();
    println!("final config = {:?}", config);
}

mod dtconfig {
    use merge::Merge;
    use std::env;

    #[derive(Debug, Clone, Merge)]
    pub struct Config {
        pub target: Option<String>,

        #[merge(strategy = merge::vec::append)]
        pub search_dirs: Vec<String>
    }

    impl Default for Config {
        fn default() -> Config {
            Config {
                target: None,
                search_dirs: Vec::new()
            }
        }
    }
    
    impl Config {
        pub fn build() -> Config {
            let cli_config = Config::from_cli();
            let mut file_config = Config {
                target: Some(String::from("pgadmin4")),
                search_dirs: vec![String::from("/usr/share/applications")]
            };

            println!("cli config = {:?}", cli_config);

            file_config.merge(cli_config);
            file_config
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
                        _ => println!("Invalid option argument")
                    }
                    is_option = false;
                }

                // Is the target
                config.target = Some(arg.clone());
            }
            if search_dirs.len() > 0 {
                config.search_dirs = search_dirs;
            }
            config
        }
    }
}