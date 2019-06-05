#[derive(Debug)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Debug)]
pub struct Tag {
    pub sign: Sign,
    pub name: String,
}
