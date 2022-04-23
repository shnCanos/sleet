use core::panic;
use std::process::Command;
use std::fs::File;
use std::env;
use std::io::Write;
use std::io::Read;

extern crate dirs;

const INSTALLER : &str = "pacman";
const INSTALL_ARG : &str = "-S";
const SEARCH_ARG : &str = "-Ss";
const LIST : &str = "list.shnaw";
const REMOVE_ARG : &str = "-Rsn";

enum AddRemove {
    Add,
    Remove
}

impl AddRemove {
    fn change_list(&self, args: &mut Vec<String>) {
        let path = format!("{}/.config/sleet/{}", dirs::home_dir().unwrap().to_str().unwrap(), LIST);

        match &self {
        
            AddRemove::Add => {
                let mut list = 
                    std::fs::OpenOptions::new()
                    .append(true)
                    .open(&path)
                    .unwrap();
                
                let args = args.clone();
                
                if let Err(e) = write!(list, "\n{}", args[1..].join("\n")) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            },
    
            AddRemove::Remove => {
                let mut file = get_file_info();
    
                file.retain(
                    |x| {
                        let mut trt = true;
                        args.iter().for_each(|y| if x == y {trt = false});
                        
                        trt
                    }
                );
                
                std::fs::write(&path, file.join("\n")).expect("Unable to write file");
            }
        }
    }
}

fn main() {
    let mut command_line_args: Vec<String> = env::args().collect();
    command_line_args.remove(0);
    
    if command_line_args.is_empty() {
        println!("No arguments were given!\n\nYou can use these:\nsearch\ninstall\nrecover\nremove\n\nAborting...");
        std::process::exit(1);
    }

    let list_dir : String = format!("{}/.config/sleet/", dirs::home_dir().unwrap().to_str().unwrap());
    
    let path = format!("{}{}", list_dir, LIST);

    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir(list_dir); // I am lazy
        File::create(&path).expect(&format!("Failed to create config file in {}", &path)).write_all(b"# This is a comment.\n\n# You can type your apps manually\n# But be sure to separate them with newlines.\n\n# Note: Comments only work when the \"#\" is the first char.\n# \" # hello!\" wouldn't be seen as a comment\n# Because the first char is a space.\n# Be careful with that.\n\n\n\n# Automatically added").unwrap();
    }

    match &command_line_args.remove(0) as &str {
        "search" => search(&mut command_line_args),
        "install" => install(&mut command_line_args, true),
        "recover" => recover(),
        "remove" => remove(&mut command_line_args),
        other => {println!("\"{}\" is Not an argument!\n\nYou can use these:\nsearch\ninstall\nrecover\nremove\n\nAborting...", other); std::process::exit(1);}
    };


}

/// Query the package manager
fn search(args: &mut Vec<String>) {
    args.insert(0, SEARCH_ARG.to_string());

    run_command(INSTALLER, args)
}

/// Installs a program
fn install(args: &mut Vec<String>, add: bool) {
    args.insert(0, INSTALL_ARG.to_string());

    if add {
        AddRemove::Add.change_list(args);
    }
    
    run_command(INSTALLER, args);
}

/// Removes a program
fn remove(args: &mut Vec<String>) {
    args.insert(0, REMOVE_ARG.to_string());

    AddRemove::Remove.change_list(args);

    run_command(INSTALLER, args);
}

/// Runs a command
fn run_command(command: &str, args: &mut Vec<String>) {
    println!("Running: {} {}", command, args.join(" "));
    
    args.insert(0, command.to_string());

    Command::new("sudo")
    .args(args)
    .status()
    .unwrap();
}

/// Reeinstals the whole system
fn recover() {
    
    let mut args : Vec<String> = get_file_info().iter().filter(|x| x != &"" && x.to_string().as_bytes()[0] != 35).map(|x| x.to_string()).collect::<Vec<String>>();

    install(&mut args, false);
}

/// Gets the path information
fn get_file_info() -> Vec<String> {
    let path = format!("{}/.config/sleet/{}", dirs::home_dir().unwrap().to_str().unwrap(), LIST);
    
    let mut file = 
    match File::open(&path) {
        Ok(s) => s,
        Err(e) => panic!("Failed to open {}: {}", &path, e)
    };
    let mut args = String::new();
    file.read_to_string(&mut args).unwrap();
    
    args.split("\n").map(|x| x.to_string()).collect::<Vec<String>>()

}
