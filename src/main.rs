use std::collections::HashMap;
use crate::request_controller::{RequestControllerClient};
use std::env;
use std::fs;
use std::io;
use std::io::{Write};
use std::path::Path;

mod response_controller;
mod request_controller;
mod user_input;

fn main() {
    let cli_arguments: Vec<String> = env::args().collect();
    let args = user_input::process_user_input(cli_arguments);

    //Retrieve API_KEY
    let path = Path::new("configfile.txt");
    let api_key = get_api_key_from_configfile(path);

    let mut seperated_input: Vec<String> = args.0;

    let client = RequestControllerClient::new(api_key.as_str());
    if seperated_input.is_empty() {
        std::process::exit(1);
    } else {
        let mut all_scan_results = HashMap::new();
        for url in seperated_input {
            let res = client.send_url_scan(&url);

            for re in res {
                let vec_url_scan_result = re.analyze_url_report();
                all_scan_results.insert(vec_url_scan_result.0, vec_url_scan_result.1);
            }
        }

        print_hashmap(all_scan_results);
        let mut guard = String::new();
        io::stdin().read_line(&mut guard).expect("Could not write to guard.");
    }
}

///Retrieves the API_KEY from the configfile, which is placed in the same folder as the executable.
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
        } else if guard && char != ';' {
            key.push(char);
        }
    }
    println!("{}", key);
    key
}

///Translates the copied path into the OS correct form, regarding backslashes.
fn translate_path(path: &String) {
    if cfg!(target_os ="Windows") {} else {
        println!("ubuntu")
    }
}

///Prints the results from the scans in a 'pretty' way to stdout.
fn print_hashmap(map: HashMap<String, i32>) {
    for x in map {
        let fmt = format!("{} {}\n", x.0, x.1);
        io::stdout().write_all(fmt.as_bytes()).expect("Error while printing to std::out");
        io::stdout().flush().expect("Could not flush std::out.");
    }
}
