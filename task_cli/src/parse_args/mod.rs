use hypercortex_engine::{Id, Mutation, Period, PrimitiveParsingError, Prop, Query, Sign, Tag};

mod command;
use command::Command;

pub struct ParsedArgs {
    command: Option<Command>,
    mutations: Option<Vec<Mutation>>,
    queries: Option<Vec<Query>>,
}

impl ParsedArgs {
    fn partition_args(
        raw_args: &Vec<String>,
    ) -> (Option<Vec<String>>, Option<Command>, Option<Vec<String>>) {
        for (i, string) in raw_args.iter().enumerate() {
            if let Some(command) = Command::from_string(string) {
                let mut query_strings = raw_args.clone();
                let mutation_strings = query_strings.split_off(i + 1);
                query_strings.split_off(i);

                return (
                    if query_strings.len() > 0 {
                        Some(query_strings)
                    } else {
                        None
                    },
                    Some(command),
                    if mutation_strings.len() > 0 {
                        Some(mutation_strings)
                    } else {
                        None
                    },
                );
            }
        }

        return (Some(raw_args.clone()), None, None);
    }

    fn parse_query_strings(strings: &Option<Vec<String>>) -> Option<Vec<Query>> {
        match strings {
            None => None,
            Some(strings) => Some(
                strings
                    .iter()
                    .map(|string| Query::from_string(string.clone()))
                    .collect(),
            ),
        }
    }

    fn parse_mutation_strings(strings: &Option<Vec<String>>) -> Option<Vec<Mutation>> {
        match strings {
            None => None,
            Some(strings) => {
                let result = strings
                    .iter()
                    .map(|string| Mutation::from_string(string.clone()))
                    .collect();

                match result {
                    Err(e) => panic!(e),
                    Ok(mutations) => Some(mutations),
                }
            }
        }
    }

    pub fn new(raw_args: Vec<String>) -> Self {
        let (query_strings, command, mutation_strings) = ParsedArgs::partition_args(&raw_args);

        Self {
            command,
            mutations: ParsedArgs::parse_mutation_strings(&mutation_strings),
            queries: ParsedArgs::parse_query_strings(&query_strings),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_locate_command_at_start() {
        let parsed = ParsedArgs::new(vec![
            String::from("add"),
            String::from("_3_"),
            String::from("_4_"),
        ]);

        assert_eq!(parsed.queries, None);
        assert_eq!(parsed.command, Some(Command::Add));
    }

    #[test]
    fn can_locate_command_in_middle() {
        let parsed = ParsedArgs::new(vec![
            String::from("_1_"),
            String::from("_2_"),
            String::from("delete"),
            String::from("_3_"),
            String::from("_4_"),
        ]);

        assert_eq!(parsed.command, Some(Command::Delete));
    }

    #[test]
    fn can_locate_command_at_end() {
        let parsed = ParsedArgs::new(vec![
            String::from("_1_"),
            String::from("_2_"),
            String::from("modify"),
        ]);

        assert_eq!(parsed.command, Some(Command::Modify));
        assert_eq!(parsed.mutations, None);
    }

    #[test]
    fn can_parse_with_no_command() {
        let parsed = ParsedArgs::new(vec![String::from("_1_"), String::from("_2_")]);

        assert_eq!(parsed.command, None);
    }

    #[test]
    fn can_parse_queries() {
        let parsed = ParsedArgs::new(vec![
            String::from("+foo"),
            String::from("-bar"),
            String::from("123baz"),
            String::from("qux456"),
        ]);

        assert_eq!(
            parsed.queries,
            Some(vec![
                Query::Tag(Tag::new(String::from("foo"), Sign::Plus)),
                Query::Tag(Tag::new(String::from("bar"), Sign::Minus)),
                Query::Id(Id::new(String::from("123baz"))),
                Query::Id(Id::new(String::from("qux456"))),
            ])
        )
    }

    #[test]
    fn can_parse_mutations() {
        let parsed = ParsedArgs::new(vec![
            String::from("add"),
            String::from("+foo"),
            String::from("-bar"),
            String::from("this"),
            String::from("is"),
            String::from("description"),
            String::from("due:now"),
            String::from("wait:2019-02-03"),
            String::from("sleep:1d"),
        ]);

        assert_eq!(
            parsed.mutations,
            Some(vec![
                Mutation::Tag(Tag::new(String::from("foo"), Sign::Plus)),
                Mutation::Tag(Tag::new(String::from("bar"), Sign::Minus)),
                Mutation::PlainText(String::from("this")),
                Mutation::PlainText(String::from("is")),
                Mutation::PlainText(String::from("description")),
            ])
        )
    }
}
