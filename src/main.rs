mod dt_config;
use dt_config::Config;


fn main() {
    let config = Config::build();
    println!("final config = {:?}", config);
}

// fn search_for_file(config: &Config) -> std::io::Result<String> {
//     for dir in config.search_dirs {
        
//     }
// }