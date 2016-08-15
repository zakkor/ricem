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

fn print_help() {
    println!("USAGE:\n\tricem <command> [command-specific-args]\n");
    println!("COMMANDS:\n\thelp, -h, --help\n\t\tprints this help message\n");
    println!("\tversion, -v, -V, --version\n\t\tprints program version\n");
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() <= 1 {
        println!("Error: no arguments provided.");
        print_help();
    } else {
        match args[1].as_str() {
            "help" | "-h" | "--help" => print_help(),
            "version" | "-v" | "--version" | "-V" => {
                println!("ricem version {} running on {} GNU/Linux.", VERSION, detect_distro())
            },
            _ => {}
        }
    }

}
