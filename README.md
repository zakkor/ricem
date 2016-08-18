# ricem
A lightweight, automatic dotfile manager

## What it does
It manages your dotfiles, wherever they are.

It comes with a bunch of pre-made templates for every distro, so you don't need to specify the path to a config file manually. (But you can do that if you have a file that isn't in a template).

## Disclaimer
This is pre-release software that was tested only by me. It may wipe your whole system or literally kill your cat.

Look over the source code <b>(but beware, it's not very pretty)</b> if you're unsure, and be careful.

## Usage
Create a new theme:

`ricem new my-theme`

Specify some files to track:

`ricem track i3 .Xresources emacs ~/Pictures/wp.png`

In this case, the i3 template will track `config` in `~/.i3/`, the emacs template will track `init.el` in `~/.emacs.d/`, and so on.

To copy the tracked files into your theme folder:

`ricem sync`

All done! Now any time you want to reapply this theme, (let's say you messed up your `~/.i3/config`, or you want to switch between themes), it's as simple as:

`ricem apply my-theme`

This copies the files you `ricem sync`ed into their original paths.

Note that in some cases (for example `/etc/i3status.conf`) you need to provide root to be able to apply stuff.

`sudo ricem apply my-theme`

You can also upload your `~/.ricem/` directory (which contains the config files and metadata) to Github.

Anytime you wish to get your configs (or someone elses configs) from Github:

`ricem download https://github.com/username/ricem_repo`

This merges whatever themes are in that repo with your own, but it doesn't overwrite your themes in case they have the same name.

## Installing
- Grab `ricem` and `.conf` from [the release page](https://github.com/zakkor/ricem/releases/tag/v0.2-alpha).
- Copy `ricem` to `/usr/local/bin` (or whatever place you want that's in your path).
- Make a directory called `.ricem` in your home. (`mkdir ~/.ricem`)
- Copy `.conf` to `~/.ricem` (`cp .conf ~/.ricem`)

## Things that are not implemented yet
Very few file templates are currently available. <b>(in progress)</b>
