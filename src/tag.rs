use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub sign: Sign,
    pub name: String,
}
