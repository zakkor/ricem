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

                match select_theme(args[2].clone(), &themes, &json_util) {
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
                // do a git commit with the pre-replace files
                exec_shell(&(String::from("cd ~/.ricem && git add . && git commit -m \"Files from before syncing theme named '")
                           + &selected_theme + "'\""));
                
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

                // do a git commit with the post-replace files
                exec_shell(&(String::from("cd ~/.ricem && git add . && git commit -m \"Files that are in theme named '")
                           + &selected_theme + "'\""));
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
                    } else {
                        track_buf = std::path::PathBuf::from(val[1].as_str().unwrap()).join(val[0].as_str().unwrap());
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

                // remove temp dir
                exec_shell("rm -rf ~/.ricem/temp");

                println!("Done!");
            },
            "upload" | "ul" => {
                // this means we specify a new remote repo
                if args.len() == 3 {
                    let replace_origin_cmd = String::from("cd ~/.ricem && git remote remove origin && git remote add origin ")
                        + &args[2]
                        + " && git push -u origin master";
                    
                    exec_shell(&replace_origin_cmd);
                }
                else {
                    // if we already have a remote then just git push
                    if exec_shell("cd ~/.ricem && git remote get-url origin").status.success() {
                        exec_shell("cd ~/.ricem && git push");
                    }
                    else {
                        println!("Error: You need to specify a remote git url");
                    }
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
