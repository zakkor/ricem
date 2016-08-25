#[macro_use] extern crate json;
#[macro_use] extern crate maplit;
extern crate edit_distance;
extern crate walkdir;
extern crate getch;

use getch::Getch;

use edit_distance::edit_distance;

use walkdir::{WalkDir, Iter, WalkDirIterator};

mod theme;
use theme::*;

mod jsonutil;
use jsonutil::JsonUtil;

mod help;
use help::*;

mod util;
use util::*;

use std::fs::File;
use std::io::*;
use std::path::Path;


const VERSION: f32 = 0.4;

fn select_theme(name: String, themes: &Vec<Theme>, json_util: &JsonUtil) -> Option<String> {
    let mut json_obj = json_util.read();

    let mut return_val = None;

    if name == "none" {
        json_obj["selected"] = "none".into();
        return_val = Some(name.to_string());
    }

    match themes.iter().find(|&t| t.name == name) {
        Some(_) => {            
            // change selected theme in json obj
            json_obj["selected"] = name.clone().into();
            
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
    
    if return_val != None {
        json_util.write(&json_obj);
    }
    
    return_val
}

fn track_template(json_obj: &mut json::JsonValue, template_name: &str, selected_theme: &String) {
    let template = json_obj["templates"][template_name].clone();

    let (file, location) = 
        if !template.is_null() {
            let mut dist = detect_distro();
            
            if template[dist].is_null() {
                dist = "Default";
            }
            
            (template[dist][0].clone(), template[dist][1].clone())
        } else {
            panic!("No such template");
        };
    
    println!("from template '{}' tracking file '{}' located in '{}'", template_name, file, location);

    if json_obj["themes"][selected_theme].is_null() {
        json_obj["themes"][selected_theme] = json::JsonValue::new_array();
    }

    json_obj["themes"][selected_theme].push(template_name).unwrap();
}

fn main() {
    let mut themes: Vec<Theme> = vec![];
    let mut selected_theme = String::new();
    let args: Vec<_> = std::env::args().collect();
    
    let mut ricem_dir = std::env::home_dir().unwrap();
    ricem_dir.push(".ricem");
    ricem_dir.as_path();




    let conf_path = ricem_dir.join(".conf");

    let json_util = JsonUtil::new(&conf_path);

    match std::fs::read_dir(&ricem_dir) {
        Err(_) => {
            // if ricem dir is not found create it
            std::fs::create_dir(&ricem_dir).expect("Could not create theme directory.");
        },
        Ok(dir) => {
            // add themes based on existing directory names
            for maybe_path in dir {
                match maybe_path {
                    Ok(path) => {
                        // ignore folder named '.git'
                        if path.path().is_dir() && path.file_name() != std::path::Path::new(".git") {
                            themes.push(Theme::new(path.file_name().into_string().unwrap()));
                        }
                    },
                    Err(_) => {
                        println!("Something went wrong while parsing .ricem dir");
                    }
                }

            }
        },
    }

    // check if there's a git repo, create it if not
    if !exec_shell("cd ~/.ricem && git status").status.success() {
        println!("Creating empty git repository in ~/.ricem");
        exec_shell("cd ~/.ricem && git init");
    }

    // try to find config file
    if let Ok(_) = File::open(&conf_path) {
        // parse the configs and apply them
        let json_obj = json_util.read();
        selected_theme = json_obj["selected"].as_str().unwrap().to_string();
    } else {
        let empty_json = object!{
            "selected" => "none"
        };

        // create config file
        let mut new_file = File::create(&conf_path).unwrap();
        new_file.write_fmt(format_args!("{:#}", empty_json)).expect("Could not create config file.");
    }

    if args.len() <= 1 {
        println!("Error: no arguments provided.");
        print_help(Help::Default);
        return;
    }
    else {
        match args[1].as_str() {
            "help" | "h" => {
                print_help(Help::Default);
            },
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

                select_theme(args[2].clone(), &themes, &json_util);
            },
            "status" | "st" => {
                if selected_theme.is_empty() || selected_theme == "none" {
                    println!("No theme currently selected");
                } else {
                    println!("Currently selected theme is '{}'", selected_theme);
                    let json_obj = json_util.read();

                    let mut dist = detect_distro();

                    println!("Current theme is tracking the following files:");
                    
                    for templ_name in json_obj["themes"][&selected_theme].members() {
                        let templ_info = json_obj["templates"][templ_name.as_str().unwrap()].clone();
                        
                        if templ_info[dist].is_null() {
                            dist = "Default";
                        }
                        
                        println!("\ttemplate '{}' with file '{}' located in '{}'", templ_name
                                 , templ_info[dist][0]
                                 , templ_info[dist][1]);
                        
                        // print package dependencies if there are any
                        if !templ_info["deps"].is_null() {
                            print!("\t\twith dependencies: ");
                            for dep in templ_info["deps"].members() {
                                print!("{} ", dep);
                            }
                            println!("");
                        }
                    }
                }
            },
            "select" | "s" => {
                if args.len() < 3 {
                    println!("Error: need to provide a name for which theme to select.");
                    print_help(Help::Select);
                    return;
                }

                select_theme(args[2].clone(), &themes, &json_util);
            },
            "track" | "t" => {
                if args.len() < 3 {
                    println!("Error: need to provide files to track.");
                    print_help(Help::Track);
                    return;
                }

                let mut json_obj = json_util.read();
                
                for i in 2..args.len() {
                    if !json_obj["groups"][&args[i]].is_null() {
                        println!("From group '{}':", &args[i]);
                        
                        for templ in json_obj["groups"][&args[i]].clone().members() {
                            print!("\t");
                            track_template(&mut json_obj, templ.as_str().unwrap(), &selected_theme);
                        }
                    }
                    else if !json_obj["themes"][&selected_theme].is_null() {
                        track_template(&mut json_obj, &args[i], &selected_theme);
                    }
                }
                
                json_util.write(&json_obj);
            },
            "sync" | "y" => {
                // do a git commit with the pre-replace files
                exec_shell(&(String::from("cd ~/.ricem && git add . && git commit -m \"Files from before syncing theme named '")
                           + &selected_theme + "'\""));
                
                let json_obj = json_util.read();
                let mut dist = detect_distro();

                for templ in json_obj["themes"][&selected_theme].members() {
                    let mut theme_path = ricem_dir.join(&selected_theme);
                    
                    if templ[dist].is_null() {
                        dist = "Default";
                    }

                    let templ_file_info = &json_obj["templates"][templ.as_str().unwrap()][dist];
                    
                    theme_path.push(templ_file_info[0].as_str().unwrap());
                    println!("Synced {:?}", theme_path);

                    let track_buf = JsonUtil::json_path_to_pathbuf(&templ_file_info[0], &templ_file_info[1]);

                    std::fs::copy(track_buf, theme_path).unwrap();
                }

                // do a git commit with the post-replace files
                exec_shell(&(String::from("cd ~/.ricem && git add . && git commit -m \"Files that are in theme named '")
                           + &selected_theme + "'\""));
            },
            "apply" | "a" => {
                let selected_theme =
                    if args.len() >= 3 {
                        args[2].clone()
                    } else {
                        selected_theme
                    };
                
                let json_obj = json_util.read();
                let mut dist = detect_distro();
                
                for templ in json_obj["themes"][&selected_theme].members() {
                    let mut theme_path = ricem_dir.join(&selected_theme);

                    if templ[dist].is_null() {
                        dist = "Default";
                    }

                    let templ_file_info = json_obj["templates"][templ.as_str().unwrap()][dist].clone();
                    
                    
                    theme_path.push(templ_file_info[0].as_str().unwrap());
                    
                    let track_buf = JsonUtil::json_path_to_pathbuf(&templ_file_info[0], &templ_file_info[1]);
                    // create directories if they don't exist
                    exec_shell(&(String::from("mkdir -p ") + &track_buf.parent().unwrap().to_str().unwrap()));
                    
                    // check if we have the required permissions...
                    if let Ok(_) = std::fs::copy(&theme_path, &track_buf) {
                        println!("Applied {:?}.", theme_path)
                    } else {
                        let cp_with_sudo_cmd = String::from("sudo cp ") + &theme_path.to_str().unwrap() + " " + &track_buf.to_str().unwrap();
                        println!("Need sudo for this command: {}", &cp_with_sudo_cmd);
                        if exec_shell(&cp_with_sudo_cmd).status.success() {
                            println!("Applied {:?}.", theme_path);
                        }
                    }
                }
            },
            "delete" | "del" => {
                if args.len() < 3 {
                    println!("Error: need to provide theme to delete.");
                    print_help(Help::Delete);
                    return;
                }
                
                let mut json_obj = json_util.read();

                if let Some(_) = themes.iter().find(|&t| t.name == args[2]) {
                    json_obj["themes"].remove(&args[2]);
                } else {
                    println!("Error: that theme does not exist.");
                    print_help(Help::Delete);
                    return;
                }

                json_util.write(&json_obj);
                
                // delete theme dir recursively
                std::fs::remove_dir_all(ricem_dir.join(&args[2])).expect("Could not remove temp dir");

                // select none
                select_theme("none".to_string(), &themes, &json_util);
            },
            "download" | "dl" => {
                if args.len() < 3 {
                    println!("Error: need to provide a link to the github repository you wish to merge.");
                    print_help(Help::Delete);
                    return;
                }

                println!("Cloning repository...");
                let clone_cmd = String::from("git clone ") + &args[2] + " ~/.ricem/temp && mv -v ~/.ricem/temp/* ~/.ricem/";
                exec_shell(&clone_cmd);
                
                let temp_conf_path = ricem_dir.join("temp").join(".conf");

                let temp_json_obj = JsonUtil::new(&temp_conf_path).read();
                let mut json_obj = json_util.read();
                
                // merge temp/.conf with ~/.ricem/.conf

                // add themes that don't conflict
                
                for (key, val) in temp_json_obj["themes"].entries() {
                    if json_obj["themes"][key].is_null() {
                        println!("Added theme '{}'", key);
                        json_obj["themes"][key] = val.clone();
                    }
                }

                json_util.write(&json_obj);

                // remove temp dir
                exec_shell("rm -rf ~/.ricem/temp");

                println!("Done!");
            },
            "upload" | "ul" => {
                println!("Uploading repo to github...");
                if exec_shell("cd ~/.ricem && git remote get-url origin").status.success() {
                    // this means we specify a new remote repo
                    if args.len() == 3 {
                        // if we already have a remote then replace it
                        let replace_origin_cmd = String::from("cd ~/.ricem && git remote remove origin && git remote add origin ")
                            + &args[2]
                            + " && git push -u origin master";
                        
                        exec_shell(&replace_origin_cmd);
                    }
                    // just push
                    else {
                        exec_shell("cd ~/.ricem && git push");
                    }
                    println!("Done!");
                }
                else if args.len() < 3 {
                    println!("Error: You need to specify a remote git url");
                }
                else {
                    let add_origin_cmd = String::from("cd ~/.ricem && git remote add origin ")
                        + &args[2]
                        + " && git push -u origin master";
                    
                    exec_shell(&add_origin_cmd);
                    println!("Done!");
                }
            },
            "list" | "l" => {
                println!("Themes:");
                for t in themes {
                    println!("\t{}", t.name);
                }
            },
            "installdeps" => {
                if detect_distro() != "Arch" {
                    println!("Error: installdeps feature only works on Arch GNU/Linux right now, sorry!");
                    print_help(Help::Installdeps);
                    return;
                }
                
                let json_obj = json_util.read();
                for templ in json_obj["themes"][&selected_theme].members() {
                    let templ_name = templ.as_str().unwrap();
                    let mut deps_to_install = String::new();
                    if !json_obj["templates"][templ_name]["deps"].is_null() {
                        for dep in json_obj["templates"][templ_name]["deps"].members() {
                            deps_to_install.push_str(dep.as_str().unwrap());
                            deps_to_install.push(' ');
                        }
                        
                        println!("Installing dependencies for file '{}'", templ_name);
                        let install_cmd = &(String::from("sudo pacman -S --needed ") + &deps_to_install);
                        exec_shell_with_output(install_cmd);
                    }
                }
            },
            "edit" | "e" => {
                if args.len() < 3 {
                    println!("Error: need to specify a template to edit");
                    print_help(Help::Edit);
                    return;
                }

                let json_obj = json_util.read();
                
                let template = json_obj["templates"][&args[2]].clone();
                
                let mut dist = detect_distro();
                
                if template[dist].is_null() {
                    dist = "Default";
                }

                let full_path = String::from(template[dist][1].as_str().unwrap()) + template[dist][0].as_str().unwrap();
                
                // open in user's editor, in background
                exec_shell(&(String::from("($VISUAL ") + &(full_path + " &> /dev/null &)")));
            },
            "import" | "im" => {
                if args.len() < 3 {
                    println!("Error: need to specify a github url to import");
                    print_help(Help::Import);
                    return;
                }
                
                let getch = Getch::new().unwrap();
                
                let extensions_ignore = ["png", "jpg"]; // add more
                
                println!("Cloning repository...");
                let clone_cmd = String::from("git clone ") + &args[2] + " ~/.ricem/temp";
                exec_shell(&clone_cmd);
                let temp_dir = ricem_dir.join("temp");

                let json_obj = json_util.read();
                let mut to_add = vec![];

                'outer: for (key, val) in json_obj["templates"].entries() {
                    let walker = WalkDir::new(&temp_dir).into_iter();
                    for entry in walker {
                        let entry = entry.unwrap();
                        
                        let templ_filename = val["Default"][0].as_str().unwrap();
                        let entry_filename = entry.path().file_name().unwrap().to_str().unwrap();
                        
                        let edit_dist = edit_distance(templ_filename, entry_filename);
                        
                        if edit_dist < 3 {
                            println!("do you think {} could be from {}?", entry_filename, key);
                            println!("here's a peek at the file:\n");
                            
                            let f = File::open(entry.path()).unwrap();
                            let reader = std::io::BufReader::new(f);

                            for line in reader.lines().take(10) {
                                println!("{}", line.unwrap());
                            }

                            println!("(y/n): ");
                            let input = getch.getch().unwrap();
                            
                            std::process::Command::new("clear").status();
                            
                            if input as char == 'y' {
                                let theme_path = ricem_dir.join(&selected_theme).join(templ_filename);
                                std::fs::copy(temp_dir.join(entry.path()).as_path(), &theme_path).expect("wops");
                                to_add.push(key);
                                continue 'outer;
                            }
                        }
                    }
                }

                let mut json_obj = json_util.read();
                
                for t in to_add {
                    track_template(&mut json_obj, t, &selected_theme);
                }

                json_util.write(&json_obj);

                // remove temp dir
                exec_shell("rm -rf ~/.ricem/temp");
                
            },
            _ => {
                println!("Error: Unknown command.");
                print_help(Help::Default);
            }
        }
    }
}
