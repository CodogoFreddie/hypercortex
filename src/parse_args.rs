use crate::engine::{Mutation, Query};
use crate::id::{Id, NUMBER_OF_CHARS_IN_FULL_ID};
use crate::tag::{Sign, Tag};

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

fn partition_args<'a>(
    args: impl Iterator<Item = &'a String>,
) -> (Vec<&'a String>, Option<command::Command>, Vec<&'a String>) {
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

    (query_tokens, command, mutation_tokens)
}

pub fn parse_as_id(token: &str) -> Option<Id> {
    if token.len() > NUMBER_OF_CHARS_IN_FULL_ID {
        None
    } else {
        Some(Id(token.to_string()))
    }
}

pub fn parse_as_tag(token: &str) -> Option<Tag> {
    match (token.chars().nth(0), &token[1..]) {
        (Some('+'), name) => Some(Tag {
            sign: Sign::Plus,
            name: name.to_string(),
        }),
        (Some('-'), name) => Some(Tag {
            sign: Sign::Minus,
            name: name.to_string(),
        }),
        _ => None,
    }
}

pub fn parse_as_query(token: &str) -> Result<Query, String> {
    match parse_as_tag(token) {
        Some(tag) => return Ok(Query::Tag(tag)),
        _ => {}
    };

    match parse_as_id(token) {
        Some(id) => return Ok(Query::Id(id)),
        _ => {}
    };

    Err(format!("`{}` is not a valid query parameter", token))
}

pub fn parse_cli_args<'a>(args: impl Iterator<Item = &'a String>) -> () {
    let (query_tokens, command, mutation_tokens) = partition_args(args);

    let parsed_queries: Result<Vec<Query>, String> =
        query_tokens.iter().map(|q| parse_as_query(q)).collect();

    println!("({:?}, {:?}, {:?})", parsed_queries, command, ());
}
