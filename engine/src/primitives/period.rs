use super::parsing_error::{PrimitiveParsingError, PrimitiveParsingResult};

#[derive(Debug, Eq, PartialEq)]
pub enum Period {
    Hour(usize),
    Day(usize),
    Week(usize),
    Month(usize),
    Year(usize),
}

impl Period {
    pub fn from_string(string: &str) -> PrimitiveParsingResult<Self> {
        let mut i: usize = 1;
        let mut n: Option<usize> = None;

        while let Ok(parsed) = string[..i].parse::<usize>() {
            println!("{}, {}", parsed, i);

            i = i + 1;
            n = Some(parsed);
        }

        match (n, &string[i - 1..i]) {
            (Some(n), "h") => Ok(Period::Hour(n)),
            (Some(n), "d") => Ok(Period::Day(n)),
            (Some(n), "w") => Ok(Period::Week(n)),
            (Some(n), "m") => Ok(Period::Month(n)),
            (Some(n), "y") => Ok(Period::Year(n)),
            _ => Err(PrimitiveParsingError::MalformedPeriod(String::from(string))),
        }

        //Err(PrimitiveParsingError::MalformedPeriod(String::from(string)))
    }
}
