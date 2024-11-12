mod dt_config;
use nix::unistd;
use toml::value::Array;
use std::{ffi::CString, fs::DirEntry, panic::panic_any};

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
                        let exec_path = find_exec(file);
                        exec_desktop(exec_path);
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


fn find_exec(file_path: DirEntry) -> String {
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

    exec_line.split('=').skip(1).collect()
}

fn exec_desktop(exec_path: String) {
    let exec_path_c = std::ffi::CString::new(exec_path)
        .expect("Error converting exec path to CString");

    let args: [CString; 0] = [];
    let env: Vec<CString> = std::env::vars_os()
        .map(| (key, val) | {
            let combined = format!(
                "{}={}",
                key.to_string_lossy(),
                val.to_string_lossy()
            );
            CString::new(combined).expect("Error getting environment")
        })
        .collect();

    unistd::execve(&exec_path_c, &args, &env)
        .expect("Could not launch executable");
}
