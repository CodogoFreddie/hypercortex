use serde::{Deserialize, Serialize};
use time::Duration;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Period {
    Hour(usize),
    Day(usize),
    Week(usize),
    Month(usize),
    Year(usize),
}

impl Period {
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
