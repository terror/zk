## `zk`

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
    dir        Zettelkasten storage location
    find       Find Zettelkasten notes by tag
    link       Link two existing Zettelkasten notes
    new        Create a new Zettelkasten note
    open       Open an existing Zettelkasten note
    preview    Preview an existing Zettelkasten note in the terminal
    remove     Remove an existing Zettelkasten note
    search     Fuzzy search Zettelkasten notes
    help       Prints this message or the help of the given subcommand(s)
```
