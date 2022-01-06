use std::io;

///Processes the user input. Checks for existing CLI arguments and handles std::inputs and outputs.
pub fn process_user_input(args: Vec<String>) -> (Vec<String>, &'static str) {
    let mut cleared_args = Vec::new();
    let mut stdin_input = String::new();

    //TODO: Add linux example.
    if args.len() <= 1 {
        io::stdin().read_line(&mut stdin_input).expect("Failed to read line.");
    } else {
        let path_or_url = determine_path_or_url(&args);
        for arg in args.iter().skip(1) {
            cleared_args.push(arg.trim().to_string());
        }
        cleared_args.remove(cleared_args.len() - 1);
        return (cleared_args, path_or_url);
    }

    let path_or_url;
    let mut stdin_input_to_vec: Vec<String>;
    let urls_as_vec = if stdin_input.is_empty() {
        eprintln!("Either provide commands via CLI like that:\n\
         Windows: .\\virustotal_folderscanner.exe [URLs] [FLAG]\nor enter them directly into the window when
         starting the .exe");
        std::process::exit(1)
    } else {
        stdin_input_to_vec = stdin_input.split(' ').map(|s| s.to_owned()).collect();
        path_or_url = determine_path_or_url(&stdin_input_to_vec);
        stdin_input_to_vec.remove(stdin_input_to_vec.len()-1);
        stdin_input_to_vec
    };
    (urls_as_vec, path_or_url)
}

fn determine_path_or_url(commands: &Vec<String>) -> &'static str {
    if commands.last().unwrap().to_lowercase().trim() == "-u" {
        "url"
    } else if commands.last().unwrap().to_lowercase().trim() == "-p" {
        "path"
    } else if !commands.is_empty() && (commands.last().unwrap().to_lowercase().trim() != "-p" && commands.last().unwrap().to_lowercase().trim() != "-u") {
        panic!("You have to provide either the '-u' or '-p' flag.");
    } else {
        eprintln!("Could not determine if it is a path or url");
        std::process::exit(1);
    }
}