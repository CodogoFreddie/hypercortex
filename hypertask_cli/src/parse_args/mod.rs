use chrono::prelude::*;
use hypertask_engine::prelude::*;
use regex::Regex;
use time::Duration;

#[derive(Debug)]
pub enum CliCommand {
    Add,
    Delete,
    Done,
    Modify,
    Snooze,
}

pub fn parse_as_command(token: &str) -> Option<CliCommand> {
    match token {
        "add" => Some(CliCommand::Add),
        "delete" => Some(CliCommand::Delete),
        "done" => Some(CliCommand::Done),
        "modify" => Some(CliCommand::Modify),
        "snooze" => Some(CliCommand::Snooze),
        _ => None,
    }
}

fn partition_args<'a>(
    args: impl Iterator<Item = &'a String>,
) -> (Vec<&'a String>, Option<CliCommand>, Vec<&'a String>) {
    let mut query_tokens = vec![];
    let mut mutation_tokens = vec![];

    let mut command: Option<CliCommand> = None;

    for arg in args {
        if command.is_none() {
            if let Some(c) = parse_as_command(&arg) {
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
    if let Some(tag) = parse_as_tag(token) {
        return Ok(Query::Tag(tag));
    };

    if let Some(id) = parse_as_id(token) {
        return Ok(Query::Id(id));
    };

    Err(format!("`{}` is not a valid query parameter", token))
}

fn parse_weekday(weekday: Weekday) -> DateTime<Utc> {
    let now_week = Utc::now().iso_week();
    let d = Utc
        .isoywd(now_week.year(), now_week.week(), weekday)
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
    parse_weekday(Weekday::Sun)
        .with_second(59)
        .unwrap()
        .with_minute(59)
        .unwrap()
        .with_hour(23)
        .unwrap()
}

fn end_of_month() -> DateTime<Utc> {
    let d = end_of_day();
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
    static ref DATE_SHORTCUT_REGEX: Regex = Regex::new(r"(\d+)([dwmy])").unwrap();
}

fn is_relative_date_shortcut(token: &str) -> bool {
    DATE_SHORTCUT_REGEX.is_match(token)
}

fn parse_relative_date_shortcut(token: &str) -> DateTime<Utc> {
    let caps = DATE_SHORTCUT_REGEX.captures(token).unwrap();
    let number = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let unit = caps.get(2).unwrap().as_str();

    let increment = match (number, unit) {
        (n, "d") => Duration::days(n),
        (n, "w") => Duration::weeks(n),
        (n, "m") => Duration::seconds(n * 60 * 60 * 24 * 365 / 12),
        (n, "y") => Duration::days(n * 365),
        (_, u) => panic!("{} is not a valid unit", u),
    };

    println!("{:?}, {:?}", number, unit);

    Utc::now() + increment
}

pub fn parse_as_date_time(token: &str) -> Result<DateTime<Utc>, String> {
    match token {
        "now" => Ok(Utc::now()),
        "mon" => Ok(parse_weekday(Weekday::Mon)),
        "tue" => Ok(parse_weekday(Weekday::Tue)),
        "wed" => Ok(parse_weekday(Weekday::Wed)),
        "thu" => Ok(parse_weekday(Weekday::Thu)),
        "fri" => Ok(parse_weekday(Weekday::Fri)),
        "sat" => Ok(parse_weekday(Weekday::Sat)),
        "sun" => Ok(parse_weekday(Weekday::Sun)),
        "eod" => Ok(end_of_day()),
        "eow" => Ok(end_of_week()),
        "eom" => Ok(end_of_month()),
        "eoy" => Ok(end_of_year()),
        x if is_relative_date_shortcut(&x) => Ok(parse_relative_date_shortcut(&x)),
        _ => Err(format!("`{}` is a malformed DateTime value", token)),
    }
}

fn parse_as_recur(token: &str) -> Result<Recur, String> {
    let caps = DATE_SHORTCUT_REGEX
        .captures(token)
        .ok_or_else(|| format!("{} is not a valid recurence format", token))?;

    let number = caps
        .get(1)
        .ok_or_else(|| format!("{} is not a valid recurence format", token))?
        .as_str()
        .parse::<i64>()
        .unwrap();
    let unit = caps
        .get(2)
        .ok_or_else(|| format!("{} is not a valid recurence format", token))?
        .as_str();

    let recur = match (number, unit) {
        (n, "d") => Recur::Day(n),
        (n, "w") => Recur::Week(n),
        (n, "m") => Recur::Month(n),
        (n, "y") => Recur::Year(n),
        (_, u) => panic!("{} is not a valid unit", u),
    };

    Ok(recur)
}

pub fn parse_as_prop(token: &str) -> Option<Result<Prop, String>> {
    let colon_index = match token.chars().position(|c| c == ':') {
        Some(i) => i,
        None => return None,
    };

    Some(match (&token[..colon_index], &token[colon_index + 1..]) {
        ("due", "") => Ok(Prop::Due(None)),
        ("due", value) => {
            let value = match parse_as_date_time(&value) {
                Ok(x) => x,
                Err(msg) => return Some(Err(msg)),
            };
            Ok(Prop::Due(Some(value)))
        }

        ("wait", "") => Ok(Prop::Wait(None)),
        ("wait", value) => {
            let value = match parse_as_date_time(&value) {
                Ok(x) => x,
                Err(msg) => return Some(Err(msg)),
            };
            Ok(Prop::Wait(Some(value)))
        }

        ("snooze", "") => Ok(Prop::Snooze(None)),
        ("snooze", value) => {
            let value = match parse_as_date_time(&value) {
                Ok(x) => x,
                Err(msg) => return Some(Err(msg)),
            };
            Ok(Prop::Snooze(Some(value)))
        }

        ("recur", "") => Ok(Prop::Recur(None)),
        ("recur", value) => {
            let value = match parse_as_recur(&value) {
                Ok(x) => x,
                Err(msg) => return Some(Err(msg)),
            };
            Ok(Prop::Recur(Some(value)))
        }
        _ => Err(format!("`{}` is a malformed prop parameter", token)),
    })
}

pub fn parse_as_mutation(token: &str) -> Result<Mutation, String> {
    if let Some(tag) = parse_as_tag(token) {
        return Ok(Mutation::SetTag(tag));
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

pub fn parse_cli_args<'a>(args: impl Iterator<Item = &'a String>) -> Result<Command, String> {
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

    match command {
        Some(CliCommand::Add) => Ok(Command::Create(parsed_mutations_with_merged_description)),
        Some(CliCommand::Delete) => Ok(Command::Delete(parsed_queries)),
        Some(CliCommand::Done) => Ok(Command::Update(
            parsed_queries,
            vec![Mutation::SetProp(Prop::Done(Utc::now()))],
        )),
        Some(CliCommand::Snooze) => Ok(Command::Update(
            parsed_queries,
            vec![Mutation::SetProp(Prop::Snooze(Some(
                Utc::now() + Duration::hours(1),
            )))],
        )),
        Some(CliCommand::Modify) => Ok(Command::Update(
            parsed_queries,
            parsed_mutations_with_merged_description,
        )),
        None => Ok(Command::Read(parsed_queries)),
    }
}
