use std::collections::HashMap;
use crate::request_controller::{RequestControllerClient};
use std::env;
use std::fs;
use std::path;
use std::path::Path;
use std::io;

mod response_controller;
mod request_controller;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new("configfile.txt");
    let api_key = get_api_key_from_configfile(path);

    let mut urls_without_cli = String::new();

    io::stdin().read_line(&mut urls_without_cli).expect("Failed to read line.");

    if args.len() == 1 && urls_without_cli.is_empty() {
        eprintln!("You should enter a [URLs]");
        std::process::exit(1);
    }

    let mut urls: Vec<String> = Vec::new();
    for arg in env::args().skip(1) {
        urls.push(arg);
    }

    let client = RequestControllerClient::new(api_key.as_str());
    if urls.is_empty() {
        std::process::exit(1);
    }else {
        for url in urls {
            let res = client.send_url_scan(&url);
            let mut all_scan_results = HashMap::new();

            for re in res {
                let vec_url_scan_result = re.analyze_url_report();
                all_scan_results.insert(vec_url_scan_result.0, vec_url_scan_result.1);
            }

            println!("{:?}", all_scan_results);
        }
    }
}

fn get_api_key_from_configfile(path: &Path) -> String {
    let value_from_configfile = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(e) => panic!("Could not read from configfile!"),
    };

    let mut guard = false;
    let mut key = String::new();
    for char in value_from_configfile.trim().chars() {
        if char == '=' && !guard {
            guard = true;
        } else if guard == true && char != ',' {
            key.push(char);
        }
    }
    println!("{}", key);
    key
}

fn translate_path(path: &String) {
    if cfg!(target_os ="Windows") {} else {
        println!("ubuntu")
    }
}
