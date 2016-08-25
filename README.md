# ricem [![Build Status](https://travis-ci.org/zakkor/ricem.svg?branch=master)](https://travis-ci.org/zakkor/ricem)
A lightweight, automatic dotfile manager.

Here it is in action: [video link](https://u.teknik.io/4tcfD.ogv)


## What it does
It manages your dotfiles, wherever they are.

It comes with a bunch of pre-made templates for every distro, so you don't need to specify the path to a config file manually. (But you can do that if you have a file that isn't in a template).

## Disclaimer
This is pre-release software that was tested only by me. It may wipe your whole system or literally kill your cat.

Look over the source code if you're unsure, and be careful.

## Usage
### Creating and uploading a new theme:

`ricem new my-theme`

##### Specify some files to track:

`ricem track i3 .Xresources emacs ~/Pictures/wp.png`

In this case, the i3 template will track the template group `i3` which contains `i3wm` -(file `config` in `~/.i3/config`) and `i3status` (file `.i3status.conf` in `~/`), the emacs template will track `init.el` in `~/.emacs.d/`, and so on.

##### To copy the tracked files from their locations in the system into your ricem theme folder:

`ricem sync`

All done!

##### Upload your newly made theme to a Github repository:

`ricem upload git@github.com:username/repo`

Please note that the Github url <b>needs</b> to be in SSH form (like above), and you <b>need</b> to have a SSH key registered to your PC active on your Github account.

### Downloading a Github repository created by ricem (like above):

`ricem download https://github.com/username/ricem_repo`

This merges whatever themes are in that repo with your own, but it doesn't overwrite your themes in case they have the same name.

### Downloading a Github repository <b>NOT</b> created by ricem (ANY random repo with dotfiles in it)

`ricem import https://github.com/username/random_dotfiles_repo`

For close enough file matches, this will give you a peek of the file and prompt you with a y/n as to which template you think the file belongs to, and add them to the currently selected theme. You can them apply them directly with `ricem apply`.

##### *On Arch Linux only (for now)*: Installing dependencies specified by the files tracked by the currently selected theme:

`ricem installdeps`

For example, if the currently selected theme contains the template `i3`, then this command will install "i3" and "i3status" using pacman, if needed.

##### Applying the theme to the system

`ricem apply my-theme`

or

`ricem apply`, to apply the currently selected theme.

### Other commands:

`ricem list` to list all your themes

`ricem select <theme name>` to select a theme

`ricem edit <template name>` to open a file in your editor (specified by $VISUAL). Example: `ricem edit rc.xml` will open `~/.config/openbox/rc.xml`

`ricem update` will download the latest version of `.conf` that contains the latest templates directly from this github repo.

## Obtaining
#### Dependencies:
- git
- wget
- GNU/Linux system: (need bash, cp, rm, mkdir, ...)

#### Installing
- Download `ricem` from [the latest release](https://github.com/zakkor/ricem/releases).
- Make it executable (`chmod +x ricem`)
- Copy `ricem` to `/usr/local/bin` (or whatever place you want that's in your path).

#### Or compile it yourself:
- clone this repo
- `cargo build --release`

## Planned features
- [x] Creating new themes
- [x] Selecting themes
- [x] File tracking from templates
- [x] OS-specific templates
- [x] Automatically detect GNU/Linux distro
- [x] Syncing theme
- [x] Applying theme
- [x] File tracking from manual input
- [ ] Applying only specific files
- [ ] Syncing only specific files
- [ ] Showing which files are applied / unapplied / modified since last application.
- [ ] <b> MORE TEMPLATES! </b>
