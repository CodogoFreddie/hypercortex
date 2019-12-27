# CLI Intro

## Installing

The Cli is written in [rust][rust]. It can be installed using [cargo][cargo], rust's package manager with the command

```bash
$ cargo install hypertask
```

> Cargo can be installed (along with the rest of the rust toolchain) from [rustup][rustup]

Hypertask can then be run with the `task` command

## First Run

When you first run `task`, it will create a config file for you with default config at your system's default config path:

- **Linux**: `$XDG_CONFIG_HOME` (`$HOME/.config/hypertask-cli/client.toml`)
- **OSX**: `$HOME/Library/Application Support/hypertask-cli/client.toml`
- **Windows**: `%APPDATA%` (`C:\Users\%USERNAME%\AppData\Roaming\hypertask-cli\client.toml`)

## Syntax

### Query

### Mutation

### Command

[rust]: https://www.rust-lang.org/
[rustup]: https://rustup.rs/
[cargo]: https://doc.rust-lang.org/cargo/
