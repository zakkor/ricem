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

struct Theme<'a> {
    name: &'a String,
    tracking: Vec<ConfigFile>,
}

impl<'a> Theme<'a> {
    fn new(name: &'a String) -> Self {
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
    let mut themes: Vec<Theme> = vec![];
    let mut selected_theme = 0;

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

                themes.push(Theme::new(&args[2]));
                if selected_theme > 0 {
                    selected_theme += 1;
                }

                match std::fs::create_dir(themes[selected_theme].name) {
                    Err(why) => println!("! {:?}", why.kind()),
                    Ok(_) => {
                        println!("Created new theme named '{}'.", args[2]);
                    },
                }
                
            },
            "status" | "s" => {
                println!("Currently selected theme is {}", themes[selected_theme].name);
            },
            _ => {}
        }
    }

}
