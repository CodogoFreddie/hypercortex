mod command;

use chrono::prelude::*;
use command::Command;
use hypercortex_engine::prelude::*;

type ParsingOutput<Output> = Option<Result<Output,()>>;

fn parse_as_tag(input: &str) -> ParsingOutput<Tag> {
    match input.chars().nth(0) {
        Some('+') => Some(Ok(Tag::new(&input[1..], Sign::Plus))),
        Some('-') => Some(Ok(Tag::new(&input[1..], Sign::Minus))),
        _ => None
    }
}

mod parse_as_prop {
    use super::*;

    fn parse_as_prop(input: &str) -> ParsingOutput<Prop> {
        let splited: Vec<&str> = input.split(':').collect();

        match (splited.get(0), splited.get(1)) {
            (Some("recur"), payload) => parse_as_recur(payload),

            _ => None,
        }
    }
}
