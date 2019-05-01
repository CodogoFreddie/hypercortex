mod command;

use chrono::prelude::*;
use command::Command;
use hypercortex_engine::*;

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

    fn parse_mutation_strings(
        get_now: &GetNow,
        strings: &Vec<String>,
    ) -> PrimitiveParsingResult<Vec<Mutation>> {
        let mut mutations: Vec<Mutation> = strings
            .iter()
            .map(|s| Mutation::from_string(get_now, &s[..]))
            .collect::<PrimitiveParsingResult<Vec<Mutation>>>()?;

        let mut not_plains = Vec::new();
        let mut description: Option<String> = None;

        while let Some(mutation) = mutations.pop() {
            if let Mutation::PlainText(plain_text) = mutation {
                description = Some(match description {
                    None => plain_text.clone(),
                    Some(tail) => format!("{} {}", plain_text, tail),
                });
            } else {
                not_plains.push(mutation)
            }
        }

        not_plains.reverse();

        if let Some(description_string) = description {
            not_plains.push(Mutation::Prop(Prop::Description(Some(description_string))))
        }

        Ok(not_plains)
    }

    pub fn new(get_now: &GetNow, raw_args: Vec<String>) -> PrimitiveParsingResult<Self> {
        let (query_strings, command, mutation_strings) = ParsedArgs::partition_args(&raw_args);

        let queries = match query_strings {
            None => None,
            Some(qs) => Some(ParsedArgs::parse_query_strings(&qs)?),
        };

        let mutations = match mutation_strings {
            None => None,
            Some(qs) => Some(ParsedArgs::parse_mutation_strings(get_now, &qs)?),
        };

        Ok(Self {
            command,
            mutations,
            queries,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn mock_get_now() -> DateTime<Utc> {
        Utc.ymd(2014, 7, 8).and_hms(9, 10, 11)
    }

    #[test]
    fn can_locate_command_at_start() {
        let parsed = ParsedArgs::new(
            &mock_get_now,
            vec![
                String::from("add"),
                String::from("_3_"),
                String::from("_4_"),
            ],
        )
        .unwrap();

        assert_eq!(parsed.queries, None);
        assert_eq!(parsed.command, Some(Command::Add));
    }

    #[test]
    fn can_locate_command_in_middle() {
        let parsed = ParsedArgs::new(
            &mock_get_now,
            vec![
                String::from("_1_"),
                String::from("_2_"),
                String::from("delete"),
                String::from("_3_"),
                String::from("_4_"),
            ],
        )
        .unwrap();

        assert_eq!(parsed.command, Some(Command::Delete));
    }

    #[test]
    fn can_locate_command_at_end() {
        let parsed = ParsedArgs::new(
            &mock_get_now,
            vec![
                String::from("_1_"),
                String::from("_2_"),
                String::from("modify"),
            ],
        )
        .unwrap();

        assert_eq!(parsed.command, Some(Command::Modify));
        assert_eq!(parsed.mutations, None);
    }

    #[test]
    fn can_parse_with_no_command() {
        let parsed = ParsedArgs::new(
            &mock_get_now,
            vec![String::from("_1_"), String::from("_2_")],
        )
        .unwrap();

        assert_eq!(parsed.command, None);
    }

    #[test]
    fn can_parse_queries() {
        let parsed = ParsedArgs::new(
            &mock_get_now,
            vec![
                String::from("+foo"),
                String::from("-bar"),
                String::from("123baz"),
                String::from("qux456"),
            ],
        )
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

    #[test]
    fn can_parse_mutations() {
        let parsed_mutations = ParsedArgs::new(
            &mock_get_now,
            vec![
                "add",
                "+foo",
                "-bar",
                "due:1w",
                "wait:2018-02-03",
                "snooze:",
                "recur:3d",
                "archived:true",
                "this",
                "is",
                "the",
                "description",
            ]
            .iter()
            .map(|&s| String::from(s))
            .collect(),
        )
        .unwrap()
        .mutations
        .unwrap();

        let expected = vec![
            Mutation::Tag(Tag::new("foo", Sign::Plus)),
            Mutation::Tag(Tag::new("bar", Sign::Minus)),
            Mutation::Prop(Prop::Due(Some(AbstractDate::Definite(
                Utc.ymd(2014, 7, 15).and_hms(9, 10, 11),
            )))),
            Mutation::Prop(Prop::Wait(Some(AbstractDate::Definite(
                Utc.ymd(2018, 2, 3).and_hms(0, 0, 0),
            )))),
            Mutation::Prop(Prop::Snooze(None)),
            Mutation::Prop(Prop::Recur(Some(Period::Day(3)))),
            Mutation::Prop(Prop::Archived(Some(true))),
            Mutation::Prop(Prop::Description(Some(String::from(
                "this is the description",
            )))),
        ];

        expected
            .iter()
            .zip(parsed_mutations.iter())
            .for_each(|(expected, produced)| {
                assert_eq!(expected, produced);
            });

        assert_eq!(expected.len(), parsed_mutations.len());
    }
}
