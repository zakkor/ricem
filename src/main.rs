use std::fs::File;
use std::io::*;


const VERSION: f32 = 0.0;

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
struct Command {
    name: &'static str,
    short_name: &'static str
}
 */


struct ConfigFile {
    name: String,
    path: String
}


struct Theme {
    name: String,
    tracking: Vec<ConfigFile>,
}

impl Theme {
    fn new(name: String) -> Self {
        Theme {
            name: name,
            tracking: vec![]
        }
    }
}
    
enum Help {
    Default,
    New
}

fn print_help(command: Help) {
    let usage = "USAGE:\n\tricem <command> [command-specific-args]\n";
    let help = "\thelp, h\n\t\tprints this help message\n";
    let version = "\tversion, v\n\t\tprints program version\n";
    let new = "\tnew, n\t[theme_name]\n\t\tcreates a new empty theme named [theme_name]\n";
    match command {
        Help::Default => {
            println!("{}", usage);
            println!("COMMANDS:");
            println!("{}", help);
            println!("{}", version);
            println!("{}", new);
        },
        Help::New => {
            println!("{}", new);
        },
    }

}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let mut ricem_dir = std::env::home_dir().unwrap();
    ricem_dir.push(".ricem");
    ricem_dir.as_path();

    let conf_path = ricem_dir.join(".conf");
    

    match std::fs::read_dir(&ricem_dir) {
        Err(_) => {
            // if ricem dir is not found create it
            std::fs::create_dir(&ricem_dir);
        },
        Ok(_) => {
            
        },
    }

    let mut themes: Vec<Theme> = vec![];
    let mut selected_theme = String::new();
    
    // print the dirs in it
    for maybe_path in std::fs::read_dir(&ricem_dir).unwrap() {
        let path = maybe_path.unwrap();
        
        if path.path().is_dir() {
            themes.push(Theme::new(path.file_name().into_string().unwrap()));
            println!("> {:?}", path.path().display());
        }
    }

    // try to find config file
    match File::open(&conf_path) {
        Ok(mut file) => {
            // parse the configs and apply them
            let mut conf_contents = String::new();
            file.read_to_string(&mut conf_contents);
            
            let contents_vec: Vec<&str> = conf_contents.split('\n').collect();
            for line in contents_vec {
                let mut split_line = line.split_whitespace();
                while let Some(word) = split_line.next() {                 
                    if word == "selected" {
                        selected_theme = split_line.next().unwrap().to_string();
                        break;
                    }
                }
            }
        },
        Err(_) => {
            // create config file
            let new_file = File::create(&conf_path).unwrap();
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
                
                //&themes.iter().find(|&t| t.name == args[2]).unwrap().name
                //
                match std::fs::create_dir(ricem_dir.join(args[2].clone())) {
                    Err(why) => println!("! {:?}", why.kind()),
                    Ok(_) => {
                        println!("Created new theme named '{}'.", args[2]);
                    },
                }

                themes.push(Theme::new(args[2].clone()));

                selected_theme = args[2].clone();

                let mut conf_file = std::fs::OpenOptions::new().write(true).open(conf_path).unwrap();
                conf_file.set_len(0);
                conf_file.write_all(("selected ".to_string() + &selected_theme).as_str().as_bytes());
//                conf_file.sync_all();

            },
            "status" | "s" => {
                if selected_theme.is_empty() {
                    println!("No theme currently selected");
                } else {
                    println!("Currently selected theme is {}", selected_theme);
                }
            },
            _ => {}
        }
    }
}
