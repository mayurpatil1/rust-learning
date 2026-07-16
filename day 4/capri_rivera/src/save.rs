use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::player::Player;
use crate::world::Room;


#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub player: Player,
    pub rooms: HashMap<String, Room>,
}

pub fn save_game(state: &GameState) {
    match serde_json::to_string_pretty(state) {
        Ok(json) => {
            match fs::write("savegame.json", json) {
                Ok(_) => println!("Game saved."),
                Err(e) => println!("Could not write save file: {}", e),
            }
        }
        Err(e) => println!("Could not save game: {}", e),
    }
}

pub fn load_game() -> Option<GameState> {
    let json = match fs::read_to_string("savegame.json") {
        Ok(contents) => contents,
        Err(_) => {
            println!("No save file found.");
            return None;
        }
    };

    match serde_json::from_str(&json) {
        Ok(player) => Some(player),
        Err(e) => {
            println!("Save file is corrupted: {}", e);
            None
        }
    }
}