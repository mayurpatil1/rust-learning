use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub exits: HashMap<String, String>,
    pub items: Vec<String>,
}

pub fn build_world() -> HashMap<String, Room> {
    let mut rooms = HashMap::new();

    let mut entrance_exits = HashMap::new();
    entrance_exits.insert(String::from("north"), String::from("hall"));
    rooms.insert(String::from("entrance"), Room {
        name: String::from("Entrance Hall"),
        description: String::from("A damp stone room. An exit leads north."),
        exits: entrance_exits,
        items: vec![String::from("torch")], 
    });

    let mut hall_exits = HashMap::new();
    hall_exits.insert(String::from("south"), String::from("entrance"));
    hall_exits.insert(String::from("down"), String::from("cellar"));
    rooms.insert(String::from("hall"), Room {
        name: String::from("Great Hall"),
        description: String::from("A vast hall wrapped in cobwebs. Exits lead south and down."),
        exits: hall_exits,
        items: vec![], 
    });

    let mut cellar_exits = HashMap::new();
    cellar_exits.insert(String::from("up"), String::from("hall"));
    rooms.insert(String::from("cellar"), Room {
        name: String::from("Dusty Cellar"),
        description: String::from("A cramped cellar smelling of old wine. Stairs lead up."),
        exits: cellar_exits,
        items: vec![String::from("rusty key")],
    });

    rooms
}