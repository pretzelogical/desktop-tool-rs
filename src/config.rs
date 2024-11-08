use merge::Merge;
use std::env;
const DEFAULT_DIRS: &'static [&str] = &["/usr/share/applications"];

#[derive(Debug, Clone, Merge)]
pub struct Config {
    pub target: Option<String>,

    #[merge(strategy = merge::vec::append)]
    pub search_dirs: Vec<String>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            target: None,
            search_dirs: DEFAULT_DIRS
                .into_iter()
                .map(|dir| dir.to_owned().to_string())
                .collect(),
        }
    }
}

impl Config {
    pub fn new() -> Config {
        Config {
            target: None,
            search_dirs: Vec::new(),
        }
    }

    pub fn build() -> Config {
        let cli_config = Config::from_cli();
        // let mut file_config = Config::from_file();
        let mut config: Config = Default::default();

        for dir in DEFAULT_DIRS {
            let ndir = dir.to_string();
            println!("{ndir}");
        }

        println!("cli config = {:?}", cli_config);

        config.merge(cli_config);
        config
    }

    fn from_cli() -> Config {
        let args = env::args().skip(1);
        let mut config = Config::new();
        let mut option_arg = String::new();

        for arg in args {
            if arg.starts_with("--") {
                option_arg.push_str(&arg);
                continue;
            }
            if option_arg != "" {
                println!("option arg = {option_arg}");
                match option_arg.as_str() {
                    "--search-dirs" => config.search_dirs.push(arg.clone()),
                    _ => println!("Invalid option argument"),
                }
                option_arg.clear();
            }

            // Is the target
            config.target = Some(arg.clone());
            break;
        }
        config
    }
}
