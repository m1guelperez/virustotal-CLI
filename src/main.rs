use std::collections::HashMap;
use crate::request_controller::{RequestControllerClient};
use std::env;
use std::io;
use crate::configfile_controller::Configfile;

mod response_controller;
mod request_controller;
mod user_input;
mod configfile_controller;

fn main() {
    let cli_arguments: Vec<String> = env::args().collect();
    let arguments_and_type = user_input::process_user_input(cli_arguments);
    let configfile = Configfile::init();
    let api_key = configfile.api_key;
    let default_path = configfile.default_path;
    if default_path.is_empty() {
        println!("INFO: No default path provided.");
    }
    let seperated_input: Vec<String> = arguments_and_type.0;
    let client = RequestControllerClient::new(api_key.as_str());

    if seperated_input.is_empty() && !default_path.is_empty() {
        //TODO:Replace with custom made method.
        let mut all_file_scans = HashMap::new();
        println!("Current path input is: {}", default_path);
        let res = RequestControllerClient::send_file_scans(&client, default_path);

        for re in res {
            let vec_file_results = re.analyze_file_report();
            all_file_scans.insert(vec_file_results.0, vec_file_results.1);
        }
        print_hashmap(all_file_scans);
        let mut guard = String::new();
        io::stdin().read_line(&mut guard).expect("Could not write to guard.");
    } else if arguments_and_type.1 == "url" {
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
    } else if arguments_and_type.1 == "path" {
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
    } else {
        std::process::exit(1);
    }
}

///Prints the results from the scans in a 'pretty' way to stdout.
fn print_hashmap(map: HashMap<String, i32>) {
    for x in map {
        println!("Request: {} Positives: {}", x.0, x.1);
    }
}
