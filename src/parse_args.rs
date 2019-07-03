use crate::engine::{Mutation, Query};
use crate::id::{Id, NUMBER_OF_CHARS_IN_FULL_ID};
use crate::prop::Prop;
use crate::tag::{Sign, Tag};
use chrono::prelude::*;
use regex::Regex;
use time::Duration;

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

fn parse_weekday(weekday: &Weekday) -> DateTime<Utc> {
    let now_week = Utc::now().iso_week();
    let mut d = Utc
        .isoywd(now_week.year(), now_week.week(), *weekday)
        .and_hms(0, 0, 0);

    if d < Utc::now() {
        d + Duration::weeks(1)
    } else {
        d
    }
}

fn end_of_day() -> DateTime<Utc> {
    Utc::now()
        .with_nanosecond(0)
        .unwrap()
        .with_second(59)
        .unwrap()
        .with_minute(59)
        .unwrap()
        .with_hour(23)
        .unwrap()
}

fn end_of_week() -> DateTime<Utc> {
    parse_weekday(&Weekday::Sun)
        .with_second(59)
        .unwrap()
        .with_minute(59)
        .unwrap()
        .with_hour(23)
        .unwrap()
}

fn end_of_month() -> DateTime<Utc> {
    let mut d = end_of_day();
    let mut count_up = Some(d);
    while let Some(new_count_up) = count_up.unwrap().with_day(1 + count_up.unwrap().day()) {
        count_up = Some(new_count_up);
    }

    count_up.unwrap()
}

fn end_of_year() -> DateTime<Utc> {
    end_of_month().with_month(12).unwrap()
}

lazy_static! {
    static ref IS_RELATIVE_DATE_SHORTCUT_REGEX: Regex = Regex::new(r"(\d+)([dwmy])").unwrap();
}
fn is_relative_date_shortcut(token: &str) -> bool {
    IS_RELATIVE_DATE_SHORTCUT_REGEX.is_match(token)
}

fn parse_relative_date_shortcut(token: &str) -> DateTime<Utc> {
    let caps = IS_RELATIVE_DATE_SHORTCUT_REGEX.captures(token).unwrap();
    let number = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let unit = caps.get(2).unwrap().as_str();

    println!("{:?}, {:?}", number, unit); 

    Utc::now()
}

pub fn parse_as_date_time(token: &str) -> Result<DateTime<Utc>, String> {
    match token {
        "now" => Ok(Utc::now()),
        "mon" => Ok(parse_weekday(&Weekday::Mon)),
        "tue" => Ok(parse_weekday(&Weekday::Tue)),
        "wed" => Ok(parse_weekday(&Weekday::Wed)),
        "thu" => Ok(parse_weekday(&Weekday::Thu)),
        "fri" => Ok(parse_weekday(&Weekday::Fri)),
        "sat" => Ok(parse_weekday(&Weekday::Sat)),
        "sun" => Ok(parse_weekday(&Weekday::Sun)),
        "eod" => Ok(end_of_day()),
        "eow" => Ok(end_of_week()),
        "eom" => Ok(end_of_month()),
        "eoy" => Ok(end_of_year()),
        x if is_relative_date_shortcut(&x) => Ok(parse_relative_date_shortcut(&x)),
        _ => Err(format!("`{}` is a malformed DateTime value", token)),
    }
}

pub fn parse_as_prop(token: &str) -> Option<Result<Prop, String>> {
    let colon_index = match token.chars().position(|c| c == ':') {
        Some(i) => i,
        None => return None,
    };

    Some(match (&token[..colon_index], &token[colon_index + 1..]) {
        ("due", value) => {
            let value = match parse_as_date_time(&value) {
                Ok(x) => x,
                Err(msg) => return Some(Err(msg)),
            };
            Ok(Prop::Due(value))
        }
        _ => Err(format!("`{}` is a malformed prop parameter", token)),
    })
}

pub fn parse_as_mutation(token: &str) -> Result<Mutation, String> {
    match parse_as_tag(token) {
        Some(tag) => return Ok(Mutation::SetTag(tag)),
        _ => {}
    };

    match parse_as_prop(token) {
        Some(Ok(prop)) => return Ok(Mutation::SetProp(prop)),
        Some(Err(msg)) => return Err(msg),
        _ => {}
    }

    Ok(Mutation::SetProp(Prop::Description(token.to_string())))
}

fn merge_description_mutations(mut mutations: Vec<Mutation>) -> Vec<Mutation> {
    let mut output: Vec<Mutation> = vec![];
    let mut description: Option<String> = None;

    while let Some(m) = mutations.pop() {
        if let Mutation::SetProp(Prop::Description(d)) = m {
            description = match description {
                None => Some(d.to_string()),
                Some(ds) => Some(format!("{} {}", d, ds)),
            }
        } else {
            output.push(m);
        }
    }

    if let Some(d) = description {
        output.push(Mutation::SetProp(Prop::Description(d)))
    }

    output
}

pub fn parse_cli_args<'a>(args: impl Iterator<Item = &'a String>) -> Result<(), String> {
    let (query_tokens, command, mutation_tokens) = partition_args(args);

    let parsed_queries: Vec<Query> = (query_tokens
        .iter()
        .map(|q| parse_as_query(q))
        .collect::<Result<Vec<Query>, String>>())?;

    let parsed_mutations: Vec<Mutation> = mutation_tokens
        .iter()
        .map(|m| parse_as_mutation(m))
        .collect::<Result<Vec<Mutation>, String>>()?;

    let parsed_mutations_with_merged_description = merge_description_mutations(parsed_mutations);

    println!(
        "{:#?}",
        (
            parsed_queries,
            command,
            parsed_mutations_with_merged_description
        )
    );

    Ok(())
}
