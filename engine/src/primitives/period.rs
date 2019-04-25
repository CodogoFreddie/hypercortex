#[derive(Debug, Eq, PartialEq)]
pub enum Period {
    Hour(usize),
    Day(usize),
    Week(usize),
    Month(usize),
    Year(usize),
}
