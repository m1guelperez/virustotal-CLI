use std::fs;

const CONFIGFILE_NAME: &str = "personal_configfile.txt";

pub struct Configfile {
    pub api_key: String,
    pub default_path: String,
}

impl Configfile {
    pub fn init() -> Configfile {
        let mut configfile = Configfile { api_key: "".to_string(), default_path: "".to_string() };
        let api_key_and_index = get_api_key_from_configfile();
        let default_path_and_index = get_default_scan_path(api_key_and_index.1);

        configfile.api_key = api_key_and_index.0;
        configfile.default_path = default_path_and_index.0;
        configfile
    }
}

///Returning a tuple such that we can propagate the number of skips to the next method, for reading
/// out config values.
fn get_api_key_from_configfile() -> (String, i32) {
    let value_from_configfile = match fs::read_to_string(CONFIGFILE_NAME) {
        Ok(v) => v,
        Err(_e) => panic!("Could not read from configfile!"),
    };
    let key = iterate_through_configfile(value_from_configfile.as_str().trim(), 0);
    (key.0, key.1)
}

fn get_default_scan_path(skips_from_before: i32) -> (String, i32) {
    let value_from_configfile = match fs::read_to_string(CONFIGFILE_NAME) {
        Ok(v) => v,
        Err(_e) => panic!("Could not read from configfile!"),
    };
    let default_path = iterate_through_configfile(value_from_configfile.as_str().trim(), skips_from_before);
    (default_path.0, default_path.1)
}

///Returning a tuple such that we can skip n-iterations in the next loop, for reading out
/// the next config-part instead of iterating through the whole file again.
fn iterate_through_configfile(config_file_contents: &str, skips: i32) -> (String, i32) {
    let mut guard = false;
    let mut index = 0;
    let mut result = String::new();
    let bound = config_file_contents.trim().chars().skip(skips as usize);
    for char in bound {
        if char == '=' && !guard {
            guard = true;
        } else if guard && char != ';' {
            result.push(char);
        } else if char == ';' {
            break;
        }
        index += 1;
    }
    index += 1;
    (result, index)
}