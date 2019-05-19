use crate::engine::{Mutation, Query};

mod command {
    #[derive(Debug)]
    pub enum Command {
        Add,
        Delete,
        Done,
        Modify,
        Snooze,
    }

    pub fn parse_as_command(token: &str) -> Option<Command> {
        match token {
            "add" => Some(Command::Add),
            "delete" => Some(Command::Delete),
            "done" => Some(Command::Done),
            "modify" => Some(Command::Modify),
            "snooze" => Some(Command::Snooze),
            _ => None,
        }
    }
}

enum Arg {
    Command(command::Command),
    Query(Query),
    Mutation(Mutation),
}

pub fn parse_cli_args<'a>(args: impl Iterator<Item = &'a String>) -> () {
    let mut query_tokens = vec![];
    let mut mutation_tokens = vec![];
    let mut command: Option<command::Command> = None;

    for arg in args {
        if command.is_none() {
            if let Some(c) = command::parse_as_command(&arg) {
                command = Some(c);
            } else {
                query_tokens.push(arg);
            }
        } else {
            mutation_tokens.push(arg);
        }
    }

    println!("({:?}, {:?}, {:?})", query_tokens, command, mutation_tokens);
}
