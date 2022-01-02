use std::collections::HashMap;
use crate::request_controller::{RequestControllerClient};
use std::env;
use std::fs;
use std::io;
use std::io::{Write};
use std::path::Path;

mod response_controller;
mod request_controller;

fn main() {
    let args = process_user_input();

    let mut urls: Vec<String> = Vec::new();
    for arg in args.iter() {
        urls.push(arg.to_string());
    }


    let path = Path::new("configfile.txt");
    let api_key = get_api_key_from_configfile(path);

    let client = RequestControllerClient::new(api_key.as_str());
    if urls.is_empty() {
        std::process::exit(1);
    } else {
        let mut all_scan_results = HashMap::new();
        for url in urls {
            let res = client.send_url_scan(&url);

            for re in res {
                let vec_url_scan_result = re.analyze_url_report();
                all_scan_results.insert(vec_url_scan_result.0, vec_url_scan_result.1);
            }
            //println!("{:?}", all_scan_results);
        }
        print_hashmap(all_scan_results);
        let mut guard = String::new();
        io::stdin().read_line(&mut guard).expect("Could not write to guard");
    }
}

fn get_api_key_from_configfile(path: &Path) -> String {
    let value_from_configfile = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(_e) => panic!("Could not read from configfile!"),
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

fn process_user_input() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let mut cleared_args = Vec::new();

    let mut urls_without_cli = String::new();

    //TODO: Add linux example.
    if args.len() <= 1 {
        io::stdin().read_line(&mut urls_without_cli).expect("Failed to read line.");
    } else {
        for arg in args {
            cleared_args.push(arg);
            return cleared_args;
        };
    }

    let urls_as_vec = if urls_without_cli.is_empty() {
        eprintln!("Either provide commands via CLI like that:\n\
         Windows: .\\virustotal_folderscanner.exe [URLs]\nor enter them directly into the window when
         starting the .exe");
        std::process::exit(1)
    } else {
        urls_without_cli.split(' ').map(|s| s.to_owned()).collect()
    };
    urls_as_vec
}

fn print_hashmap(map: HashMap<String, i32>) {
    for x in map {
        let fmt = format!("{} {}\n", x.0, x.1);
        io::stdout().write_all(fmt.as_bytes()).expect("Error while printing to std::out");
        io::stdout().flush().expect("Could not flush std::out.");
    }
}
