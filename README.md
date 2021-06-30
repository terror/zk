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
    dir       Output the Zettelkasten directory path
    search    Fuzzy search notes
    find      Find notes by tag
    new       Create a new note
    open      Open a note
    rm        Remove a note
    link      Link two notes
    rmlink    Remove a link between two notes
    tag       Add a tag to a note
    rmtag     Remove a tag from a note
    help      Prints this message or the help of the given subcommand(s)
```
