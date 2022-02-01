use std::io;

///Processes the user input. Checks for existing CLI arguments or std::inputs.
pub fn process_user_input(mut args: Vec<String>, default_path: &str) -> (Vec<String>, &'static str) {
    let mut cleared_args = Vec::new();
    let mut stdin_input = String::new();

    //TODO: Add linux example.
    if args.len() <= 1 {
        io::stdin().read_line(&mut stdin_input).expect("Failed to read line.");
    } else {
        let mut valid_args = Vec::new();
        if args.len() > 1 && (args.last().unwrap().contains("-p") || args.last().unwrap().contains("-u")) {
            valid_args = catch_escaped_chars_in_old_powershell_versions(&mut args);
        } else {
            valid_args = args
        }
        let path_or_url = determine_path_or_url(&valid_args);
        for arg in valid_args.iter().skip(1) {
            cleared_args.push(arg.trim().to_string());
        }
        cleared_args.remove(cleared_args.len() - 1);
        return (cleared_args, path_or_url);
    }
    let path_or_url;
    let mut stdin_input_to_vec: Vec<String>;
    let urls_as_vec = if stdin_input.len() == 1 && default_path.is_empty() {
        println!("Either provide commands via CLI like that:\n\
         Windows: .\\virustotal_folderscanner.exe [URLs] [FLAG]\nor enter them directly into the window when starting the .exe\n\
         If you want to let the program only scan a default path, then make sure you added one in the configfile.");
        std::process::exit(1)
    } else {
        stdin_input_to_vec = stdin_input.split(' ').map(|s| s.to_owned()).collect();
        path_or_url = determine_path_or_url(&stdin_input_to_vec);
        stdin_input_to_vec.remove(stdin_input_to_vec.len() - 1);
        stdin_input_to_vec
    };
    (urls_as_vec, path_or_url)
}

fn determine_path_or_url(commands: &Vec<String>) -> &'static str {
    println!("Commands length: {}", commands.len());
    if commands.len() <= 1 {
        "default"
    } else if commands.last().unwrap().to_lowercase().trim() == "-p" {
        "path"
    } else if commands.last().unwrap().to_lowercase().trim() == "-u" {
        "url"
    } else if !commands.len() > 1 && (commands.last().unwrap().to_lowercase().trim() != "-p" && commands.last().unwrap().to_lowercase().trim() != "-u") {
        panic!("You have to provide either the '-u' or '-p' flag.");
    } else {
        println!("Could not determine if it is a path or url");
        std::process::exit(1);
    }
}

//Some Powershell versions have a problem it the path ends with a backslash. Here we try to filter this out
fn catch_escaped_chars_in_old_powershell_versions(commands: &mut Vec<String>) -> Vec<String> {
    let mut final_vec: Vec<String> = Vec::new();
    //Push everything before the last command, which contains the flag
    for command in commands.iter().take(commands.len() - 1) {
        final_vec.push(command.to_string());
    }
    if commands.last().unwrap().trim().contains("-p") || commands.last().unwrap().trim().contains("-u") {
        let mut last_path = commands.last().unwrap().trim().to_string();
        let mut flag_reversed = String::new();
        flag_reversed.push(last_path.pop().unwrap());
        flag_reversed.push(last_path.pop().unwrap());
        //Delete the quotation marks
        last_path.pop();
        last_path.pop();
        let mut flag = String::new();
        flag.push(flag_reversed.pop().unwrap());
        flag.push(flag_reversed.pop().unwrap());

        final_vec.push(last_path);
        final_vec.push(flag);
    }
    println!("Currently the final Vec is: {:?}", &final_vec);
    final_vec
}

#[cfg(test)]
mod tests {
    use crate::user_input::{catch_escaped_chars_in_old_powershell_versions, determine_path_or_url};

    #[test]
    fn determine_path_or_url_test() {
        let vec_url = vec!["google.de".to_string(), "-u".to_string()];
        let url = determine_path_or_url(&vec_url);
        assert_eq!(url, "url");

        let vec_path = vec!["default_path".to_string(), "-p".to_string()];
        let path = determine_path_or_url(&vec_path);
        assert_eq!(path, "path");

        let vec_path = vec!["".to_string()];
        let path = determine_path_or_url(&vec_path);
        assert_eq!(path, "default");
    }

    #[test]
    #[should_panic(expected = "Could not determine if it is a path or url")]
    fn determine_path_or_input_panic() {
        let vec_path = vec![];
        determine_path_or_url(&vec_path);
    }

    #[test]
    fn catch_escaped_chars_in_old_powershell_version_test() {

        let mut args = vec!["C:/Users/rust/virustotal_folderscanner.exe".to_string(), "C:/Users/X Y/Desktop/ -p".to_string()];
        let res = catch_escaped_chars_in_old_powershell_versions(&mut args);
        let final_vec = vec!["C:/Users/rust/virustotal_folderscanner.exe".to_string(), "C:/Users/X Y/Desktop".to_string(), "-p".to_string()];
        assert_eq!(res, final_vec);
    }
}