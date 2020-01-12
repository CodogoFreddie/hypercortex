# CLI Intro

#### Table of Contents

<!-- vim-markdown-toc GFM -->

* [Setup](#setup)
   * [Installing](#installing)
   * [First Run](#first-run)
* [Usage](#usage)
   * [Example](#example)
   * [Syntax](#syntax)
      * [Query](#query)
      * [Command](#command)
         * [Empty Command](#empty-command)
         * [Add](#add)
         * [Modify](#modify)
         * [Shortcuts](#shortcuts)
      * [Mutation](#mutation)

<!-- vim-markdown-toc -->

## Setup

### Installing

The Cli is written in [rust][rust]. It can be installed using [cargo][cargo], rust's package manager with the command

```bash
$ cargo install hypertask
```

> Cargo can be installed (along with the rest of the rust toolchain) from [rustup][rustup]

Hypertask can then be run with the `task` command

### First Run

When you first run `task`, it will create a config file for you with default config at your system's default config path:

- **Linux**: `$XDG_CONFIG_HOME` (`$HOME/.config/hypertask-cli/client.toml`)
- **OSX**: `$HOME/Library/Application Support/hypertask-cli/client.toml`
- **Windows**: `%APPDATA%` (`C:\Users\%USERNAME%\AppData\Roaming\hypertask-cli\client.toml`)

## Usage

### Example

To verify that hypertask is installed and working correctly, run the following command:

```bash
$ task add +test description goes here due:now
```

Which should output something like the following:

```
Id                Score    Description            Tags   Due               Recur
zk3a5zeyfydmha44  10.0000  description goes here  +test  2019-12-28 15:34
```

What all this means will be be explained in the following sections

### Syntax

```bash
$ task +tag zk3a5zeyfydmha44 modify new description +newTag wait:1h
       --------------------- ------ -------------------------------
               query        command            mutation
```

#### Query

The query selects which tasks a command will run on. The query can be composed of

- A task id (`zk3a5zeyfydmha44`): selects a single task with the coresponding id.
- A positive tag (`+tag`): selects all tasks which have a given tag.
- A negative tag (`-tag`): selects all tasks which don't have a given tag.

Any task that matches any of the fields in a query will be selected, have the effects of a given command applied to them, then displayed in a table

#### Command

The command selects what type of operation you want to perform:

##### Empty Command

```bash
$ task +tag zk3a5zeyfydmha44
```

Just renders the selected tasks, does not modify them in any way. If no query is given (i.e. you just run `$ task`) then all tasks are displayed

##### Add

```bash
$ task add +tag brand new description
```

Creates a new task, this is the only command that takes no query parameters, any input query will be ignored. More details on how to set the properties of the newly created task are available [here](#mutation)

##### Modify

```bash
$ task zk3a5zeyfydmha44 modify +newTag updated description
```

Updates any tasks that match the query with new properties. **All** the matched tasks will have their properties updated to the same value. More details on how to set the properties of the updated created task are available [here](#mutation)

##### Shortcuts

There are several commands that are shortcuts for common modifications:

- `task zk3a5zeyfydmha44 done` === `task zk3a5zeyfydmha44 modify done:now`
- `task zk3a5zeyfydmha44 snooze` === `task zk3a5zeyfydmha44 modify snooze:1h`

#### Mutation

[rust]: https://www.rust-lang.org/
[rustup]: https://rustup.rs/
[cargo]: https://doc.rust-lang.org/cargo/
