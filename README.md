## zk

<img align="right" width="225" height="120" src="./assets/zk.png">

![Build and Test](https://github.com/terror/zk/actions/workflows/rust.yml/badge.svg)

A zettelkasten command line interface

## Installation

You can install `zk` using cargo:
```bash
$ cargo install zk
```

## Usage

```
zk 0.0.1

USAGE:
    zk <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    dir                    Output the Zettelkasten directory path
    explore <name>         Explore a notes links recursively
    find    <tag>          Find notes by tag
    link    <left> <right> Link two notes
    new     <name>         Create a new note
    open    <name>         Open a note
    rm      <name>         Remove a note
    rmlink  <left> <right> Remove a link between two notes
    rmtag   <name> <tag>   Remove a tag from a note
    search                 Fuzzy search notes
    tag     <name> <tag>   Add a tag to a note
    help                   Prints this message or the help of the given subcommand(s)
```
