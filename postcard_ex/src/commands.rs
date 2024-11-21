use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Commands {
    SetPosition(Position),
    QueryPosition,
    SetTime(u64),
    Unknown, // If something fails or user error, the Unknown variant will be used.
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}
