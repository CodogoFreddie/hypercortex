use super::parsing_error::{PrimitiveParsingError, PrimitiveParsingResult};
use time::Duration;

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
        if string.len() == 0 {
            return Err(PrimitiveParsingError::MalformedPeriod(String::from(string)));
        }

        let mut i: usize = 1;
        let mut n: Option<usize> = None;

        while let Ok(parsed) = string[..i].parse::<usize>() {
            i = i + 1;
            n = Some(parsed);

            if i > string.len() {
                return Err(PrimitiveParsingError::MalformedPeriod(String::from(string)));
            }
        }

        match (n, &string[i - 1..i]) {
            (Some(n), "h") => Ok(Period::Hour(n)),
            (Some(n), "d") => Ok(Period::Day(n)),
            (Some(n), "w") => Ok(Period::Week(n)),
            (Some(n), "m") => Ok(Period::Month(n)),
            (Some(n), "y") => Ok(Period::Year(n)),
            _ => Err(PrimitiveParsingError::MalformedPeriod(String::from(string))),
        }
    }

    pub fn to_duration(&self) -> Duration {
        match self {
            Period::Hour(n) => Duration::hours(n.clone() as i64),
            Period::Day(n) => Duration::days(n.clone() as i64),
            Period::Week(n) => Duration::weeks(n.clone() as i64),
            Period::Month(n) => {
                Duration::seconds((/*number of seconds in the average month*/2631600 * n) as i64)
            }
            Period::Year(n) => {
                Duration::seconds((/*number of seconds in the average year*/31557600 * n) as i64)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod from_string {
        use super::*;

        macro_rules! test_from_params {
            ($($name:ident: $input:expr, $output:expr,)*) => {
                $(
                    #[test]
                    fn $name() {
                        assert_eq!(Period::from_string($input), $output);
                    }
                )*
            }
        }

        test_from_params! {
            empty:          "",      Err(PrimitiveParsingError::MalformedPeriod(String::from(""))),
            just_numbers:   "123",   Err(PrimitiveParsingError::MalformedPeriod(String::from("123"))),
            just_letter:    "h",     Err(PrimitiveParsingError::MalformedPeriod(String::from("h"))),
            just_letters:   "abc",   Err(PrimitiveParsingError::MalformedPeriod(String::from("abc"))),
            mad_shit:       "+#@(",  Err(PrimitiveParsingError::MalformedPeriod(String::from("+#@("))),

            one_hour:       "1h",    Ok(Period::Hour(1)),
            two_hours:      "2h",    Ok(Period::Hour(2)),
            thirty_hours:   "30h",   Ok(Period::Hour(30)),
            one_day:        "1d",    Ok(Period::Day(1)),
            two_days:       "2d",    Ok(Period::Day(2)),
            thirty_days:    "30d",   Ok(Period::Day(30)),
            one_week:       "1w",    Ok(Period::Week(1)),
            two_weeks:      "2w",    Ok(Period::Week(2)),
            thirty_weeks:   "30w",   Ok(Period::Week(30)),
            one_month:      "1m",    Ok(Period::Month(1)),
            two_months:     "2m",    Ok(Period::Month(2)),
            thirty_months:  "30m",   Ok(Period::Month(30)),
            one_year:       "1y",    Ok(Period::Year(1)),
            two_years:      "2y",    Ok(Period::Year(2)),
            thirty_years:   "30y",   Ok(Period::Year(30)),
        }
    }

    mod to_duration {
        use super::*;

        macro_rules! test_from_params {
            ($($name:ident: $input:expr, $output:expr,)*) => {
                $(
                    #[test]
                    fn $name() {
                        assert_eq!($input.to_duration(), $output);
                    }
                )*
            }
        }

        test_from_params! {
            one_hour:       Period::Hour(1),    Duration::hours(1),
            two_hours:      Period::Hour(2),    Duration::hours(2),
            thirty_hours:   Period::Hour(30),   Duration::hours(30),
            one_day:        Period::Day(1),     Duration::days(1),
            two_days:       Period::Day(2),     Duration::days(2),
            thirty_days:    Period::Day(30),    Duration::days(30),
            one_week:       Period::Week(1),    Duration::weeks(1),
            two_weeks:      Period::Week(2),    Duration::weeks(2),
            thirty_weeks:   Period::Week(30),   Duration::weeks(30),
            one_month:      Period::Month(1),   Duration::seconds(2631600),
            two_months:     Period::Month(2),   Duration::seconds(5263200),
            thirty_months:  Period::Month(30),  Duration::seconds(78948000),
            one_year:       Period::Year(1),    Duration::seconds(31557600),
            two_years:      Period::Year(2),    Duration::seconds(63115200),
            thirty_years:   Period::Year(30),   Duration::seconds(946728000),
        }
    }
}
