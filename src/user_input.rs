use std::io;
use crate::Configfile;

///Processes the user input. Checks for existing CLI arguments or std::inputs.
pub fn process_user_input(args: Vec<String>, default_path: &str) -> (Vec<String>, &'static str) {
    let mut cleared_args = Vec::new();
    let mut stdin_input = String::new();

    //TODO: Add linux example.
    if args.len() <= 1 {
        io::stdin().read_line(&mut stdin_input).expect("Failed to read line.");
    } else {
        //catch_escaped_character();
        let path_or_url = determine_path_or_url(&args);
        for arg in args.iter().skip(1) {
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
        eprintln!("Could not determine if it is a path or url");
        std::process::exit(1);
    }
}

fn catch_escaped_character(commands: &Vec<String>) {
    let commands_length = commands.len();
    if commands.last().unwrap().as_bytes()[commands_length] == b'p' || commands.last().unwrap().as_bytes()[commands_length] == b'u' {
        println!("Yes works!");
    }

}