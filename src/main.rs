mod dt_config;
use std::{fs::DirEntry, panic::panic_any};

use dt_config::Config;


fn main() {
    let config = Config::build();
    println!("final config = {:?}", config);
    search_for_file(&config);
}

fn search_for_file(config: &Config) {
    let target= config.target
        .as_ref()
        .expect("Error parsing target!");
    for path in &config.search_dirs {
        let dir = std::fs::read_dir(path)
            .expect("Error reading path");

        for file_path in dir {
            match file_path {
                Ok(file_path) => {
                    if let Some(file) = check_file(file_path, target) {
                        find_exec(file);
                    }
                },
                Err(error) => panic_any(error.to_string())
            }
        }
    }
}

fn check_file(file_path: DirEntry, target: &String) -> Option<DirEntry> {
    println!("{:?}", file_path);
    let file_name_os = file_path
        .file_name();
    let file_name = file_name_os
        .into_string()
        .expect("Error parsing dir!");

    if file_name.ends_with(".desktop") && file_name.contains(target) {
        println!("Found target {target}");
        return Some(file_path);
    }
    return None;
}


fn find_exec(file_path: DirEntry) {
    let contents = std::fs::read_to_string(
            file_path.path()
        )
        .expect("Error reading target file");

    let exec_line: String = contents
        .split('\n')
        .filter(| x | { x.contains("Exec=")} )
        .take(1)
        .collect();

    println!("Exec line: {exec_line}");
}