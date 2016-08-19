use std::process::{Output, Command};


/// Returns a `str` containing the name of the Linux distribution the user is currently running.
/// Returns "Default" if it cannot detect succesfully.

pub fn detect_distro() -> &'static str {
    let release_info = exec_shell("cat /etc/*-release").stdout;
    let release_info = String::from_utf8_lossy(&release_info);

    let linux_distros = vec!["Arch", "Ubuntu", "Debian", "OpenSUSE", "Fedora", "Gentoo", "Kubuntu", "Lubuntu"];

    let mut distro_name = "Default";
    
    for name in linux_distros {
        if release_info.contains(name) {
            distro_name = name; 
        }
    }

    if distro_name == "Default" {
        println!("Could not detect your distro, assuming 'Default' (you won't get any distro-specific templates).");
    }
    
    distro_name
}


pub fn exec_shell(arg: &str) -> Output {
    Command::new("sh")
        .arg("-c")
        .arg(arg)
        .output()
        .expect("failed to execute process")
}
