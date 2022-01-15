use std::fs;

const CONFIGFILE_NAME: &str = "configfile.txt";

pub fn get_api_key_from_configfile() -> String {
    let value_from_configfile = match fs::read_to_string(CONFIGFILE_NAME) {
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

pub fn get_default_scan_path() {}