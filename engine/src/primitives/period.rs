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
}

#[cfg(test)]
mod test {
    use super::*;

    mod from_string {
        use super::*;

        macro_rules! test_from_string {
            ($($name:ident: $input:expr, $output:expr,)*) => {
                $(
                    #[test]
                    fn $name() {
                        assert_eq!(Period::from_string($input), $output);
                    }
                )*
            }
        }

        test_from_string! {
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
}
