#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Add,
    Delete,
    Done,
    Export,
    Modify,
    Share,
    Snooze,
}

impl Command {
    pub fn from_string(string: &String) -> Option<Command> {
        match string.as_str() {
            "add" => Some(Command::Add),
            "delete" => Some(Command::Delete),
            "done" => Some(Command::Done),
            "export" => Some(Command::Export),
            "modify" => Some(Command::Modify),
            "share" => Some(Command::Share),
            "snooze" => Some(Command::Snooze),
            _ => None,
        }
    }
}
