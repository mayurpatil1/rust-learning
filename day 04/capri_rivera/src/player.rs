use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub health: u32,
    pub current_room: String,
    pub inventory: Vec<String>,
}