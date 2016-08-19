#[derive(PartialEq, Eq, Hash)]
pub enum Help {
    New,
    Select,
    Track,
    Delete,
    
    Default,
}

pub fn print_help(command: Help) {
    let usage = "USAGE:\n\tricem <command> [command-specific-args]\n";
    let help = "\thelp, h\n\t\tprints this help message\n";
    let version = "\tversion, v\n\t\tprints program version\n";
    
    let help_commands = hashmap!{
        Help::New => "\tnew, n   [theme_name]\n\t\tcreates a new empty theme named [theme_name]\n",
        Help::Select => "\tselect, e   [theme_name]\n\t\tselects the theme named [theme_name]\n",
        Help::Track => "\ttrack, t   [template1] [template2] ... [templateN]\n\t\tstarts tracking the template named [templateX]\n",
        Help::Delete => "\tdelete, del   [theme_name]\n\t\tdeletes the theme named [theme_name]\n",
    };
    
    match command {
        Help::Default => {
            println!("{}", usage);
            println!("COMMANDS:");
            println!("{}", help);
            println!("{}", version);
            
            for (_, val) in help_commands {
                println!("{}", val);
            }
        },
        _ => {
            println!("{}", help_commands[&command]);
        }
    }
}
