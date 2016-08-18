#![feature(question_mark)]
#![feature(step_by)]
#[macro_use]

extern crate json;

use std::fs::File;
use std::io::*;
use std::collections::HashMap;


const VERSION: f32 = 0.0;

mod theme;
use theme::*;

/// Returns a `str` containing the name of the Linux distribution the user is currently running.
/// Panics if it can't detect succesfully.
fn detect_distro() -> &'static str {
    let shell_command = std::process::Command::new("sh")
        .arg("-c")
        .arg("cat /etc/*-release")
        .output()
        .expect("failed to execute process");    

    let release_info = String::from_utf8_lossy(&shell_command.stdout);

    let linux_distros = vec!["Arch", "Ubuntu", "Debian", "OpenSUSE", "Fedora", "Gentoo", "Kubuntu", "Lubuntu"];

    let mut distro_name = "NULL";
    
    for name in linux_distros {
        if release_info.contains(name) {
            distro_name = name; 
        }
    }

    if distro_name == "NULL" {
        panic!("Could not detect your OS");
    }
    
    distro_name
}
    
enum Help {
    Default,
    New,
    Select,
    Track,
}

/*
{
    "selected": "darky",
    "i3": {
        "Arch": "~/.i3/config",
        "Ubuntu": "~/.i3/config"
    }
}
*/

fn select_theme(name: String, themes: &Vec<Theme>, conf_path: &std::path::Path) -> Option<String> {
    match themes.iter().find(|&t| t.name == name) {
        Some(_) => {
            let mut data = String::new();

            // open file for reading data into json object
            {
                let mut file = std::fs::OpenOptions::new().read(true).open(conf_path).unwrap();
                file.read_to_string(&mut data);
                // file goes out of scope
            }

            let mut obj = json::parse(&data).unwrap();
            
            // change selected theme in json obj
            obj["selected"] = name.clone().into();
            
            //open file for writing data
            {
                let mut file = std::fs::OpenOptions::new().write(true).open(conf_path).unwrap();
                file.set_len(0);
                file.write_fmt(format_args!("{:#}", obj));
                //file goes out of scope
            }
            
            println!("Selected theme '{}'.", name);
            Some(name)
        },
        None => {
            println!("Error: theme '{}' does not exist.", name);
            print_help(Help::Select);
            None
        }
    }
}

fn print_help(command: Help) {
    let usage = "USAGE:\n\tricem <command> [command-specific-args]\n";
    let help = "\thelp, h\n\t\tprints this help message\n";
    let version = "\tversion, v\n\t\tprints program version\n";
    let new = "\tnew, n   [theme_name]\n\t\tcreates a new empty theme named [theme_name]\n";
    let select = "\tselect, e   [theme_name]\n\t\tselects the theme named [theme_name]\n";
    let track = "\ttrack, t   [template1] [template2] ... [templateN]\n\t\tstarts tracking the template named [templateX]\n";
    
    match command {
        Help::Default => {
            println!("{}", usage);
            println!("COMMANDS:");
            println!("{}", help);
            println!("{}", version);
            println!("{}", new);
            println!("{}", select);
        },
        Help::New => {
            println!("{}", new);
        },
        Help::Select => {
            println!("{}", select);
        },
        Help::Track => {
            println!("{}", track);
        },
    }

}

fn load_templates(conf_path: &std::path::Path) -> HashMap<String, HashMap<String, String>> {
    let mut file = File::open(conf_path).unwrap();
    
    let mut data = String::new();
    file.read_to_string(&mut data);
    
    let mut obj = json::parse(&data).unwrap();
    obj["selected"] = "darky".into();
    println!("{}", obj["selected"]);

    let mut templates = HashMap::new();    
    
    templates
}

fn main() {

//    templates.get("i3").unwrap().get("Arch").unwrap();
    
    let mut themes: Vec<Theme> = vec![];
    let mut selected_theme = String::new();

    ////
    
    let args: Vec<_> = std::env::args().collect();
    
    let mut ricem_dir = std::env::home_dir().unwrap();
    ricem_dir.push(".ricem");
    ricem_dir.as_path();


    let conf_path = ricem_dir.join(".conf");
    //let templates = load_templates(&conf_path);

    match std::fs::read_dir(&ricem_dir) {
        Err(_) => {
            // if ricem dir is not found create it
            std::fs::create_dir(&ricem_dir);
        },
        Ok(dir) => {
            // add themes based on existing directory names
            for maybe_path in dir {
                match maybe_path {
                    Ok(path) => {
                        if path.path().is_dir() {
                            themes.push(Theme::new(path.file_name().into_string().unwrap()));
                            println!("Loaded theme {:?}", path.file_name());
                        }   
                    },
                    Err(_) => {
                        println!("Something went wrong while parsing .ricem dir");
                    }
                }

            }
        },
    }


    // try to find config file
    match File::open(&conf_path) {
        Ok(mut file) => {
            // parse the configs and apply them
            let mut data = String::new();
            file.read_to_string(&mut data);
            let mut obj = json::parse(&data).unwrap();
            selected_theme = obj["selected"].as_str().unwrap().to_string();
        },
        Err(_) => {
            let empty_json = object!{
                "selected" => "none"
            };
            {
                // create config file
                let mut new_file = File::create(&conf_path).unwrap();
                new_file.write_fmt(format_args!("{:#}", empty_json));
            }
        }
    }


    if args.len() <= 1 {
        println!("Error: no arguments provided.");
        print_help(Help::Default);
        return;
    }
    else {
        match args[1].as_str() {
            "help" | "h" => print_help(Help::Default),
            "version" | "v" => {
                println!("ricem version {} running on {} GNU/Linux.", VERSION, detect_distro())
            },
            "new" | "n" => {
                if args.len() < 3 {
                    println!("Error: need to provide a name for the new theme");
                    print_help(Help::New);
                    return;
                }

                if args[2] == "none" {
                    println!("Error: you cannot name your theme 'none'. Sorry about that.");
                    print_help(Help::New);
                    return;
                }
                
                match std::fs::create_dir(ricem_dir.join(args[2].clone())) {
                    Err(why) => println!("! {:?}", why.kind()),
                    Ok(_) => {
                        println!("Created new theme named '{}'.", args[2]);
                    },
                }

                themes.push(Theme::new(args[2].clone()));

                match select_theme(args[2].clone(), &themes, &conf_path) {
                    Some(name) => selected_theme = name,
                    None => {}
                }
            },
            "status" | "s" => {
                if selected_theme.is_empty() || selected_theme == "none" {
                    println!("No theme currently selected");
                } else {
                    println!("Currently selected theme is {}", selected_theme);
                }
            },
            "select" | "e" => {
                if args.len() < 3 {
                    println!("Error: need to provide a name for which theme to select.");
                    print_help(Help::Select);
                    return;
                }

                match select_theme(args[2].clone(), &themes, &conf_path) {
                    Some(name) => selected_theme = name,
                    None => {}
                }
            },
            "track" | "t" => {
                if args.len() < 3 {
                    println!("Error: need to provide files to track.");
                    print_help(Help::Track);
                    return;
                }

                for i in (2..args.len()).step_by(2) {
                    match themes.iter_mut().find(|ref t| t.name == selected_theme) {
                        Some(theme) => {
                            println!("tracking file {} located in {}.", args[i], args[i+1]);
                            theme.tracking.push(ConfigFile::new(args[i].clone(), args[i+1].clone()));
                        },
                        None => {}
                    }
                }
            },
            _ => {}
        }
    }
}
