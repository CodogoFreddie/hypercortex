use hypercortex_engine::{
    Id, Mutation, Period, PrimitiveParsingError, PrimitiveParsingResult, Prop, Query, Sign, Tag,
};

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

    fn parse_query_strings(strings: &Vec<String>) -> PrimitiveParsingResult<Vec<Query>> {
        strings.iter().map(|s| Query::from_string(&s[..])).collect()
    }

    fn parse_mutation_strings(strings: &Option<Vec<String>>) -> Option<Vec<Mutation>> {
        match strings {
            None => None,
            Some(_) => Some(Vec::new()),
        }
    }

    pub fn new(raw_args: Vec<String>) -> PrimitiveParsingResult<Self> {
        let (query_strings, command, mutation_strings) = ParsedArgs::partition_args(&raw_args);

        let queries = match query_strings {
            None => None,
            Some(qs) => Some(ParsedArgs::parse_query_strings(&qs)?),
        };

        Ok(Self {
            command,
            mutations: ParsedArgs::parse_mutation_strings(&mutation_strings),
            queries,
        })
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
        ])
        .unwrap();

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
        ])
        .unwrap();

        assert_eq!(parsed.command, Some(Command::Delete));
    }

    #[test]
    fn can_locate_command_at_end() {
        let parsed = ParsedArgs::new(vec![
            String::from("_1_"),
            String::from("_2_"),
            String::from("modify"),
        ])
        .unwrap();

        assert_eq!(parsed.command, Some(Command::Modify));
        assert_eq!(parsed.mutations, None);
    }

    #[test]
    fn can_parse_with_no_command() {
        let parsed = ParsedArgs::new(vec![String::from("_1_"), String::from("_2_")]).unwrap();

        assert_eq!(parsed.command, None);
    }

    #[test]
    fn can_parse_queries() {
        let parsed = ParsedArgs::new(vec![
            String::from("+foo"),
            String::from("-bar"),
            String::from("123baz"),
            String::from("qux456"),
        ])
        .unwrap();

        assert_eq!(
            parsed.queries,
            Some(vec![
                Query::Tag(Tag::new("foo", Sign::Plus)),
                Query::Tag(Tag::new("bar", Sign::Minus)),
                Query::Id(Id::new("123baz")),
                Query::Id(Id::new("qux456")),
            ])
        )
    }
}
