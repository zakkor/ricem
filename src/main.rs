#![feature(question_mark)]
#![feature(step_by)]
#[macro_use]

extern crate json;

use std::fs::File;
use std::io::*;
use std::collections::HashMap;


const VERSION: f32 = 0.2;

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
    let mut data = String::new();

    // open file for reading data into json object
    {
        let mut file = std::fs::OpenOptions::new().read(true).open(conf_path).unwrap();
        file.read_to_string(&mut data);
        // file goes out of scope
    }

    let mut obj = json::parse(&data).unwrap();

    let mut return_val = None;

    if name == "none" {
        obj["selected"] = "none".into();
        return_val = Some(name.to_string());
    }

    match themes.iter().find(|&t| t.name == name) {
        Some(_) => {            
            // change selected theme in json obj
            obj["selected"] = name.clone().into();
            
            println!("Selected theme '{}'.", name);
            return_val = Some(name);
            
        },
        None => {
            if name != "none" {
                println!("Error: theme '{}' does not exist.", name);
                print_help(Help::Select);
                return_val = None;
            }
        }
    }

    
    //open file for writing data
    if return_val != None {    
        let mut file = std::fs::OpenOptions::new().write(true).open(conf_path).unwrap();
        file.set_len(0);
        file.write_fmt(format_args!("{:#}", obj));
        //file goes out of scope
    }

    return_val
}

enum Help {
    Default,
    New,
    Select,
    Track,
    Delete,
}

fn print_help(command: Help) {
    let usage = "USAGE:\n\tricem <command> [command-specific-args]\n";
    let help = "\thelp, h\n\t\tprints this help message\n";
    let version = "\tversion, v\n\t\tprints program version\n";
    let new = "\tnew, n   [theme_name]\n\t\tcreates a new empty theme named [theme_name]\n";
    let select = "\tselect, e   [theme_name]\n\t\tselects the theme named [theme_name]\n";
    let track = "\ttrack, t   [template1] [template2] ... [templateN]\n\t\tstarts tracking the template named [templateX]\n";
    let delete = "\tdelete, del   [theme_name]\n\t\tdeletes the theme named [theme_name]\n";
    
    match command {
        Help::Default => {
            println!("{}", usage);
            println!("COMMANDS:");
            println!("{}", help);
            println!("{}", version);
            println!("{}", new);
            println!("{}", select);
            println!("{}", track);
            println!("{}", delete);
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
        Help::Delete => {
            println!("{}", delete);
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
//                            println!("Loaded theme {:?}", path.file_name());
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
            "status" => {
                if selected_theme.is_empty() || selected_theme == "none" {
                    println!("No theme currently selected");
                } else {
                    println!("Currently selected theme is '{}'", selected_theme);
                    let mut data = String::new();

                    // open file for reading data into json object
                    {
                        let mut file = std::fs::OpenOptions::new().read(true).open(&conf_path).unwrap();
                        file.read_to_string(&mut data);
                        // file goes out of scope
                    }

                    let mut obj = json::parse(&data).unwrap();
                    println!("Current theme is tracking the following files:");
                    for (key, val) in obj["themes"][&selected_theme].entries() {
                        println!("\ttemplate '{}' with file '{}' located in '{}'", key, val[0], val[1]);
                    }
                }
            },
            "select" | "s" => {
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

                for i in 2..args.len() {
                    match themes.iter_mut().find(|ref t| t.name == selected_theme) {
                        Some(theme) => {
                            let mut data = String::new();

                            // open file for reading data into json object
                            {
                                let mut file = std::fs::OpenOptions::new().read(true).open(&conf_path).unwrap();
                                file.read_to_string(&mut data);
                                // file goes out of scope
                            }

                            let mut obj = json::parse(&data).unwrap();

                            let mut file;
                            let mut location;

                            if !obj["templates"][&args[i]].is_null() {
                                file = obj["templates"][&args[i]][detect_distro()][0].clone();
                                location = obj["templates"][&args[i]][detect_distro()][1].clone();    
                            } else {
                                file = std::path::Path::new(&args[i]).file_name().unwrap().to_str().unwrap().into();
                                location = (String::from(std::path::Path::new(&args[i]).parent().unwrap().to_str().unwrap()) + "/").into();
                            }
                            
                            obj["themes"][&selected_theme][&args[i]][0] = file;
                            obj["themes"][&selected_theme][&args[i]][1] = location;

                            //open file for writing data
                            {
                                let mut file = std::fs::OpenOptions::new().write(true).open(&conf_path).unwrap();
                                file.set_len(0);
                                file.write_fmt(format_args!("{:#}", obj));
                                //file goes out of scope
                            }
                        },
                        None => {}
                    }
                }
            },
            "sync" | "y" => {
                let mut data = String::new();

                // open file for reading data into json object
                {
                    let mut file = std::fs::OpenOptions::new().read(true).open(&conf_path).unwrap();
                    file.read_to_string(&mut data);
                    // file goes out of scope
                }

                let mut obj = json::parse(&data).unwrap();

                for (key, val) in obj["themes"][&selected_theme].entries() {
                    //println!("{}, {}", val[0], val[1]);
                    let mut theme_path = ricem_dir.join(&selected_theme);
                    theme_path.push(val[0].as_str().unwrap());
                    println!("{:?}", theme_path);


                    let mut track_buf = std::path::PathBuf::new();
                    
                    if val[1].as_str().unwrap().chars().nth(0).unwrap() == '~' {
                        let track_string = val[1].as_str()
                            .clone()
                            .unwrap()
                            .to_string()
                            .replace("~", std::env::home_dir().unwrap().to_str().unwrap());
                        track_buf = std::path::PathBuf::from(track_string).join(val[0].as_str().unwrap());
                    } else {
                        track_buf = std::path::PathBuf::from(val[1].as_str().unwrap()).join(val[0].as_str().unwrap());
                    }

                    std::fs::copy(track_buf, theme_path).unwrap();
                }
            },
            "apply" | "a" => {
                let theme_to_apply =
                    if args.len() < 3 {
                        selected_theme.clone()
                    } else {
                        args[2].clone()
                    };
                
                let mut data = String::new();

                // open file for reading data into json object
                {
                    let mut file = std::fs::OpenOptions::new().read(true).open(&conf_path).unwrap();
                    file.read_to_string(&mut data);
                    // file goes out of scope
                }

                let mut obj = json::parse(&data).unwrap();

                for (key, val) in obj["themes"][&theme_to_apply].entries() {
                    //println!("{}, {}", val[0], val[1]);
                    let mut theme_path = ricem_dir.join(&theme_to_apply);
                    theme_path.push(val[0].as_str().unwrap());
                    println!("Applied '{:?}'.", theme_path);


                    let mut track_buf = std::path::PathBuf::new();
                    
                    if val[1].as_str().unwrap().chars().nth(0).unwrap() == '~' {
                        let track_string = val[1].as_str()
                            .clone()
                            .unwrap()
                            .to_string()
                            .replace("~", std::env::home_dir().unwrap().to_str().unwrap());
                        
                        track_buf = std::path::PathBuf::from(track_string).join(val[0].as_str().unwrap());
                    }

                    std::fs::copy(theme_path, track_buf).unwrap();
                }
            },
            "delete" | "del" => {
                if args.len() < 3 {
                    println!("Error: need to provide theme to delete.");
                    print_help(Help::Delete);
                    return;
                }
                let mut data = String::new();

                // open file for reading data into json object
                {
                    let mut file = std::fs::OpenOptions::new().read(true).open(&conf_path).unwrap();
                    file.read_to_string(&mut data);
                    // file goes out of scope
                }

                let mut obj = json::parse(&data).unwrap();

                match themes.iter().find(|&t| t.name == args[2]) {
                    Some(_) => {
                        obj["themes"].remove(&args[2]);
                    },
                    None => {
                        println!("Error: that theme does not exist.");
                        print_help(Help::Delete);
                        return;
                    }
                }

                //open file for writing data
                {
                    let mut file = std::fs::OpenOptions::new().write(true).open(&conf_path).unwrap();
                    file.set_len(0);
                    file.write_fmt(format_args!("{:#}", obj));
                    //file goes out of scope
                }

                // delete theme dir recursively
                std::fs::remove_dir_all(ricem_dir.join(&args[2]));

                // select none
                select_theme("none".to_string(), &themes, &conf_path);
            },
            "download" | "dl" => {
                if args.len() < 3 {
                    println!("Error: need to provide a link to the github repository you wish to merge.");
                    print_help(Help::Delete);
                    return;
                }

                println!("Cloning repository...");
                
                let clone_cmd = String::from("git clone ") + &args[2] + " ~/.ricem/temp && mv -v ~/.ricem/temp/* ~/.ricem/";
                
                let shell_cmd = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(clone_cmd)
                    .output()
                    .expect("failed to execute process");


                // merge temp/.conf with ~/.ricem/.conf

                let mut data = String::new();
                let mut temp_conf_path = ricem_dir.join("temp").join(".conf");

                // open file for reading data into json object
                {
                    let mut file = std::fs::OpenOptions::new().read(true).open(&temp_conf_path).unwrap();
                    file.read_to_string(&mut data);
                    // file goes out of scope
                }

                let mut obj = json::parse(&data).unwrap();
                let mut new_obj = object!{};

                for (key, val) in obj["themes"].entries() {
                    new_obj[key] = val.clone();
                }

                data.clear();

                // open file for reading data into json object
                {
                    let mut file = std::fs::OpenOptions::new().read(true).open(&conf_path).unwrap();
                    file.read_to_string(&mut data);
                    // file goes out of scope
                }
                let mut obj = json::parse(&data).unwrap();

                for (key, val) in new_obj.entries() {
                    if obj["themes"][key].is_null() {
                        obj["themes"][key] = val.clone();
                    }
                }

                //open file for writing data
                {
                    let mut file = std::fs::OpenOptions::new().write(true).open(&conf_path).unwrap();
                    file.set_len(0);
                    file.write_fmt(format_args!("{:#}", obj));
                    //file goes out of scope
                }

                let shell_cmd = std::process::Command::new("sh")
                    .arg("-c")
                    .arg("rm -rf ~/.ricem/temp")
                    .output()
                    .expect("failed to execute process");

                println!("Done!");
            },
            "upload" | "ul" => {
                if args.len() < 3 {
                    println!("Error: need to provide a link to the github repository you wish to merge.");
                    print_help(Help::Delete);
                    return;
                }
            },
            "list" | "l" => {
                println!("Themes:");
                for t in themes {
                    println!("\t{}", t.name);
                }
            },
            _ => {
                println!("Error: Unknown command.");
                print_help(Help::Default);
            }
        }
    }
}
