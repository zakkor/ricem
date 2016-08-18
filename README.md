# ricem
A lightweight, automatic dotfile manager

## What it does
It manages your dotfiles, wherever they are.

It comes with a bunch of pre-made templates for every distro, so you don't need to specify the path to a config file manually. (But you can do that if you have a file that isn't in a template).

## Usage
Create a new theme:

`ricem new my-theme`

Specify some files to track:

`ricem track i3 .Xresources emacs ~/Pictures/wp.png`

To copy the tracked files into your theme folder:

`ricem sync`

All done! Now any time you want to reapply this theme, (let's say you messed up your `~/.i3/config`), it's as simple as:

`ricem apply my-theme`

This copies the files you `ricem sync`ed into their original paths.

You can also upload your `~/.ricem/` directory (which contains the config files and metadata) to Github.

Anytime you wish to get your configs (or someone elses configs) from Github:

`ricem download https://github.com/username/ricem_repo`

This merges whatever themes are in that repo with your own, but it doesn't overwrite your themes in case they have the same name.
