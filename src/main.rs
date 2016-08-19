#[macro_use] extern crate json;
#[macro_use] extern crate maplit;

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
use json::JsonValue;
use std::path::{Path, PathBuf};


const VERSION: f32 = 0.2;

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
            std::fs::create_dir(&ricem_dir);
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

                match select_theme(args[2].clone(), &themes, &json_util) {
                    Some(name) => selected_theme = name,
                    None => {}
                }
            },
            "status" => {
                if selected_theme.is_empty() || selected_theme == "none" {
                    println!("No theme currently selected");
                } else {
                    println!("Currently selected theme is '{}'", selected_theme);
                    let json_obj = json_util.read();
                    
                    println!("Current theme is tracking the following files:");
                    for (key, val) in json_obj["themes"][&selected_theme].entries() {
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

                if let Some(name) = select_theme(args[2].clone(), &themes, &json_util) {
                    selected_theme = name;
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
                            let mut json_obj = json_util.read();
                            
                            let template = json_obj["templates"][&args[i]].clone();

                            let (file, location) = 
                                if !template.is_null() {
                                    let dist = detect_distro();
                                    (template[dist][0].clone(), template[dist][1].clone())
                                }
                            else {
                                (Path::new(&args[i])
                                 .file_name()
                                 .unwrap()
                                 .to_str()
                                 .unwrap()
                                 .into()
                                 ,
                                 (String::from(
                                     Path::new(&args[i])
                                         .parent()
                                         .unwrap()
                                         .to_str()
                                         .unwrap()) + "/").into())
                            };
                            
                            json_obj["themes"][&selected_theme][&args[i]][0] = file;
                            json_obj["themes"][&selected_theme][&args[i]][1] = location;

                            json_util.write(&json_obj);
                        },
                        None => {}
                    }
                }
            },
            "sync" | "y" => {
                // do a git commit with the pre-replace files
                exec_shell(&(String::from("cd ~/.ricem && git add . && git commit -m \"Files from before syncing theme named '")
                           + &selected_theme + "'\""));
                
                let mut json_obj = json_util.read();

                for (key, val) in json_obj["themes"][&selected_theme].entries() {
                    let mut theme_path = ricem_dir.join(&selected_theme);
                    theme_path.push(val[0].as_str().unwrap());
                    println!("Synced {:?}", theme_path);

                    let track_buf = JsonUtil::json_path_to_pathbuf(&val[0], &val[1]);

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
                
                let mut json_obj = json_util.read();

                for (key, val) in json_obj["themes"][&selected_theme].entries() {
                    let mut theme_path = ricem_dir.join(&selected_theme);
                    theme_path.push(val[0].as_str().unwrap());
                    println!("Applied '{:?}'.", theme_path);

                    let mut track_buf = JsonUtil::json_path_to_pathbuf(&val[0], &val[1]);

                    std::fs::copy(theme_path, track_buf).unwrap();
                }
            },
            "delete" | "del" => {
                if args.len() < 3 {
                    println!("Error: need to provide theme to delete.");
                    print_help(Help::Delete);
                    return;
                }
                
                let mut json_obj = json_util.read();
                
                match themes.iter().find(|&t| t.name == args[2]) {
                    Some(_) => {
                        json_obj["themes"].remove(&args[2]);
                    },
                    None => {
                        println!("Error: that theme does not exist.");
                        print_help(Help::Delete);
                        return;
                    }
                }

                json_util.write(&json_obj);
                
                // delete theme dir recursively
                std::fs::remove_dir_all(ricem_dir.join(&args[2]));

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
                

                // merge temp/.conf with ~/.ricem/.conf
                let mut temp_conf_path = ricem_dir.join("temp").join(".conf");

                let temp_json_obj = JsonUtil::new(&temp_conf_path).read();
                
                let mut new_obj = object!{};

                // copy stuff from temp/.conf to a new empty object
                for (key, val) in temp_json_obj["themes"].entries() {
                    new_obj[key] = val.clone();
                }

                let mut json_obj = json_util.read();

                // add themes from the new object that we read from temp/.conf to our json_obj
                for (key, val) in new_obj.entries() {
                    if json_obj["themes"][key].is_null() {
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
            _ => {
                println!("Error: Unknown command.");
                print_help(Help::Default);
            }
        }
    }
}
