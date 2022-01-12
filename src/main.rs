use std::collections::HashMap;
use crate::request_controller::{RequestControllerClient};
use std::env;
use std::fs;
use std::io;
use std::io::{Write};
use std::path::Path;
use crate::response_controller::ResponseControllerFile;

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
    } else if args.1 == "url" {
        //TODO:Replace with custom made method.
        let mut all_scan_results = HashMap::new();

        let res = RequestControllerClient::send_url_scan(&client, seperated_input);
        for re in res {
            let vec_url_scan_result = re.analyze_url_report();
            all_scan_results.insert(vec_url_scan_result.0, vec_url_scan_result.1);
        }
        print_hashmap(all_scan_results);
        let mut guard = String::new();
        io::stdin().read_line(&mut guard).expect("Could not write to guard.");
    } else {
        //TODO:Replace with custom made method.
        let mut all_file_scans = HashMap::new();
        let mut path = String::new();
        for input in seperated_input {
            path = input;
        }
        println!("Current path input is: {}", path);
        let res = RequestControllerClient::send_file_scans(&client, path);

        for re in res {
            let vec_file_results = re.analyze_file_report();
            all_file_scans.insert(vec_file_results.0, vec_file_results.1);
        }

        print_hashmap(all_file_scans);
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
    key
}

///Translates the copied path into the OS correct form, regarding backslashes.
fn translate_path(path: &String) {
    if cfg!(target_os ="Windows") {
        println!("Windows detected.")
    } else {
        println!("Linux detected.")
    }
}

///Prints the results from the scans in a 'pretty' way to stdout.
fn print_hashmap(map: HashMap<String, i32>) {
    for x in map {
        let results = format!("URL: {} Positives: {}\n", x.0, x.1);
        io::stdout().write_all(results.as_bytes()).expect("Error while printing to std::out");
        io::stdout().flush().expect("Could not flush std::out.");
    }
}
