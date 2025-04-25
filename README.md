## zk

<img align="right" width="225" height="120" src="./assets/zk.png">

[![release](https://img.shields.io/github/release/terror/zk.svg?label=release&style=flat&labelColor=282c34&logo=github)](https://github.com/terror/zk/releases/latest)
[![crates.io](https://shields.io/crates/v/zk.svg)](https://crates.io/crates/zk)
[![CI](https://github.com/terror/zk/actions/workflows/ci.yaml/badge.svg)](https://github.com/terror/zk/actions/workflows/ci.yaml)
[![dependency status](https://deps.rs/repo/github/terror/zk/status.svg)](https://deps.rs/repo/github/terror/zk)

A note-taking tool based on the famous *Zettelkasten* method with support for
fuzzy searching, tags & link exploration, in Rust!

## What is a *Zettelkasten*?

A **zettelkasten**, or 'slip box' is a method of note-taking famously used by
the sociologist Niklas Luhmann. Notes essentially contain metadata such as tags
that describe key aspects of the note or links to other notes. The goal is to
enhance creativity by exploring the relationships between notes and
making new connections between seemingly unrelated ideas.

## Features

- Fast fuzzy search and link exploration powered by [skim](https://github.com/lotabout/skim)
- Works with a flat directory of markdown files
- Minimal configuration with sensible defaults

## Demo

Here is a quick demo showcasing the new, search, tag, link and explore
features.

[![asciicast](https://asciinema.org/a/4TrHLpcAv9lk0RfGngzS6ft3e.svg)](https://asciinema.org/a/4TrHLpcAv9lk0RfGngzS6ft3e)

## Installation

`zk` should run on any system, including Linux, MacOS, and the BSDs.

The easiest way to install it is by using [cargo](https://doc.rust-lang.org/cargo/index.html),
the Rust package manager:

```bash
cargo install zk-cli
```

### Pre-built binaries

Pre-built binaries for Linux, MacOS, and Windows can be found on [the releases
page](https://github.com/terror/zk/releases).

## Usage

```present cargo run -- --help
zk 0.1.0

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

## Configuration

`zk` currently looks for a configuration file called `.zk.toml` using the
[rust-xdg](https://github.com/whitequark/rust-xdg) crate.

If a configuration file is found it must have the following key-value pairs
set (default values are shown below if no configuration file is present):

```toml
# .zk.toml

# The Zettelkasten directory path.
path = '~/.zk'

# The preferred editor of choice when opening
# and editing notes.
editor = 'vim'
```

## Keybindings

The fuzzy finder `skim` supports the implementation of custom keybindings, this
section documents the custom keybindings that are implemented when using
various commands.

| Command | Keybindings                                                                      |
|---------|----------------------------------------------------------------------------------|
| explore | `<C-e>` - Edit the selected note<br/> `Enter` - Explore the selected notes links |

## Prior Art

- [`srid/neuron`](https://github.com/srid/neuron) - Future-proof note-taking and publishing based on Zettelkasten
- [`AndrewCopeland/zettelkasten`](https://github.com/AndrewCopeland/zettelkasten) - Creating notes with the zettelkasten note taking method and storing all notes on github
- [`sirupsen/zk`](https://github.com/sirupsen/zk) - Zettelkasten on the command-line 📚 🔍
