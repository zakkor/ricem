#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Help {
    New,
    Select,
    Track,
    Delete,
    Sync,
    Apply,
    Status,
    Download,
    Upload,
    List,
    Edit,
    Installdeps,
    Import,
    Update,
    
    Default,
}

pub fn print_help(command: Help) {
    let usage = "USAGE:\n\tricem <command> [command-specific-args]\n";
    let help = "\thelp, h\n\t\tprints this help message\n";
    let version = "\tversion, v\n\t\tprints program version\n";
    
    let help_commands = btreemap!{
        Help::New => "\tnew, n   [theme_name]\n\t\tcreates a new empty theme named [theme_name]\n",
        Help::Select => "\tselect, s   [theme_name]\n\t\tselects the theme named [theme_name]\n",
        Help::Track => "\ttrack, t   [template1] [template2] ... [templateN]\n\t\tstarts tracking the template named [templateX]\n",
        Help::Delete => "\tdelete, del   [theme_name]\n\t\tdeletes the theme named [theme_name]\n",
        Help::Status => "\tstatus, st   \n\t\tprints the currently selected theme and shows which file are tracked by it\n",
        Help::Sync => "\tsync, y   \n\t\tcopies the files tracked by the current theme into ~/.ricem/<THEME_NAME>, and does two git commits, before and after the copy, so there is no risk of losing your files\n",
        Help::Apply => "\tapply, a   [theme_name] <- optional\n\t\tif [theme_name] is specified, it copies the files tracked by that theme that you `sync`ed into their actual paths in the system\n\t\tif you didn't specify [theme_name], then it does all the above except for the selected theme instead\n",
        Help::Download => "\tdownload, dl   [link_to_github_repo]\n\t\tdownloads the repo THAT HAS TO BE CREATED BY RICEM and adds all the themes in it to your own \n",
        Help::Upload => "\tupload, ul   [SSH_url_to_empty_github_repo]\n\t\tuploads your theme folder to the url. you only need to specify the link the first time you run this command.\n",
        Help::List => "\tlist, l   \n\t\tprints all your themes\n",
        Help::Edit => "\tedit, e   [template_name]\n\t\topens the file tracked by [template_name] in the editor defined by the enviroment variable '$VISUAL'\n\t\tadd this variable to your .bashrc or .bash_profile if you don't have it\n",
        Help::Installdeps => "\tinstalldeps   \n\t\t_ONLY ON ARCH LINUX_ installs only the needed dependencies using pacman\n",
        Help::Import => "\timport, im   [link_to_github_repo]\n\t\timport a github repo containing dotfiles and adds whatever matches it finds to the currently selected theme\n\t\tfor close enough fuzzy matches it finds, it will show a peek of the file and prompt the user with a y/n to confirm which template the file belongs to\n",
        Help::Update => "\tupdate, upd   \n\t\tgrabs the latest version of the templates config file from github\n",
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
