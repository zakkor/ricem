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
- Create a new theme:

`ricem new my-theme`

- Specify some files to track:

`ricem track i3 .Xresources emacs ~/Pictures/wp.png`

In this case, the i3 template will track `config` in `~/.i3/`, the emacs template will track `init.el` in `~/.emacs.d/`, and so on.

- To copy the tracked files into your theme folder:

`ricem sync`

All done!

- Any time you want to reapply this theme, (let's say you messed up your `~/.i3/config`, or you want to switch between themes), it's as simple as:

`ricem apply my-theme`

This copies the files you `ricem sync`ed into their original paths.

Note that in some cases (for example `/etc/i3status.conf`) you need to provide root to be able to apply stuff.

`sudo ricem apply my-theme`

Anytime you wish to get your configs (or someone elses configs) from Github:

`ricem download https://github.com/username/ricem_repo`

This merges whatever themes are in that repo with your own, but it doesn't overwrite your themes in case they have the same name.

Try it with mine: `ricem dl https://github.com/zakkor/ricem-themes`

It only contains two themes that contain an .Xresources each.

- To upload your theme folder to a Github repository:

`ricem upload git@github.com:username/repo`

Please note that the Github url <b>needs</b> to be in SSH form (like above), and you <b>need</b> to have a SSH key registered to your PC active on your Github account.

It will probably not upload if the repository has things that aren't in your theme folder. --- _maybe should add a `--force` flag for this?_

## Installing
- Grab `ricem` and `.conf` from [the latest release](https://github.com/zakkor/ricem/releases).
- Copy `ricem` to `/usr/local/bin` (or whatever place you want that's in your path).
- Make a directory called `.ricem` in your home. (`mkdir ~/.ricem`)
- Copy `.conf` to `~/.ricem` (`cp .conf ~/.ricem`)

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
