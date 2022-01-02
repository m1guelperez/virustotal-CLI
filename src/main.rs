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
    let cli_argument: Vec<String> = env::args().collect();
    let args = process_user_input(cli_argument);

    //Retrieve API_KEY
    let path = Path::new("configfile.txt");
    let api_key = get_api_key_from_configfile(path);

    let mut urls: Vec<String> = Vec::new();
    for arg in args.iter() {
        urls.push(arg.to_string());
    }

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
        } else if guard && char != ',' {
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
///Processes the user input. Checks for existing CLI arguments and handles stdinputs and outputs.
fn process_user_input(args: Vec<String>) -> Vec<String> {
    let mut cleared_args = Vec::new();
    let mut urls_without_cli = String::new();

    //TODO: Add linux example.
    if args.len() <= 1 {
        io::stdin().read_line(&mut urls_without_cli).expect("Failed to read line.");
    } else {
        for arg in args.iter().skip(1) {
            cleared_args.push(arg.to_string());
        }
        return cleared_args;
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
///Prints the results from the scans in a 'pretty' way to stdout.
fn print_hashmap(map: HashMap<String, i32>) {
    for x in map {
        let fmt = format!("{} {}\n", x.0, x.1);
        io::stdout().write_all(fmt.as_bytes()).expect("Error while printing to std::out");
        io::stdout().flush().expect("Could not flush std::out.");
    }
}
