default:
  just --list

alias r := run
alias f := fmt

ci: build test clippy fmt-check

build:
  cargo build

test:
  cargo test -- --test-threads=1

clippy:
  cargo clippy --all-targets --all-features

fmt-check:
  cargo fmt --all -- --check
  @echo formatting check done

run subcommand *args:
  cargo run {{subcommand}} {{args}}

fmt:
  cargo fmt

check:
  cargo check --all-features --all-targets

watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"

usage:
  cargo run -- --help | pbcopy

install:
  cargo install --path .
