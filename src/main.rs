mod config;
use config::Config;

fn main() {
    let config = Config::build();
    println!("final config = {:?}", config);
}
