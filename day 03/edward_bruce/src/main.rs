use std::collections::HashMap;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Room {
    name: String,
    description: String,
    exits: HashMap<String, String>,
    items: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    name: String,
    health: u32,
    current_room: String,
    inventory: Vec<String>,
}

fn main() {
    let mut rooms = build_world(); 

    let mut hero = Player {
        name: String::from("Aria"),
        health: 100,
        current_room: String::from("entrance"),
        inventory: Vec::new(), 
    };

    println!("=== The Tiny Dungeon ===");
   println!("Commands: look, go <direction>, take <item>, drop <item>, inventory, save, load, quit\n");
    look(&hero, &rooms);

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim().to_lowercase();
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.as_slice() {
            ["quit"] | ["exit"] => {
                println!("Farewell, {}!", hero.name);
                break;
            }
            ["look"] => look(&hero, &rooms),
            ["go", direction] => move_player(&mut hero, &rooms, direction),
            ["inventory"] | ["inv"] => show_inventory(&hero),
            ["take"] => println!("Take what?"),
            ["take", item_words @ ..] => {
                let item = item_words.join(" ");
                take_item(&mut hero, &mut rooms, &item);
            }
            ["drop"] => println!("Drop what?"),
            ["drop", item_words @ ..] => {
                let item = item_words.join(" ");
                drop_item(&mut hero, &mut rooms, &item);
            }
            ["save"] => save_game(&hero),
            ["load"] => match load_game() {
                Some(loaded) => {
                    hero = loaded; 
                    println!("Game loaded. Welcome back, {}!", hero.name);
                    look(&hero, &rooms);
                }
                None => println!("Could not load a game."),
            },
            [] => continue,
            _ => println!("I don't understand that command."),
        }
    }
}

fn build_world() -> HashMap<String, Room> {
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

fn look(player: &Player, rooms: &HashMap<String, Room>) {
    match rooms.get(&player.current_room) {
        Some(room) => {
            println!("You are in the {}.", room.name);
            println!("{}", room.description);
            if !room.items.is_empty() {
                println!("You see: {}", room.items.join(", ")); 
            }
        }
        None => println!("You are lost in the void..."),
    }
}

fn move_player(player: &mut Player, rooms: &HashMap<String, Room>, direction: &str) {
    let current = match rooms.get(&player.current_room) {
        Some(room) => room,
        None => {
            println!("You are nowhere. Cannot move.");
            return;
        }
    };

    match current.exits.get(direction) {
        Some(next_room_key) => {
            player.current_room = next_room_key.clone();
            println!("You move {}.", direction);
            look(player, rooms);
        }
        None => println!("You can't go {} from here.", direction),
    }
}

fn show_inventory(player: &Player) {
    if player.inventory.is_empty() {
        println!("Your inventory is empty.");
    } else {
        println!("You are carrying: {}", player.inventory.join(", "));
    }
}

fn take_item(player: &mut Player, rooms: &mut HashMap<String, Room>, item: &str) {
    let room = match rooms.get_mut(&player.current_room) {
        Some(r) => r,
        None => return,
    };
    match room.items.iter().position(|i| i == item) {
        Some(index) => {
            let taken = room.items.remove(index); 
            player.inventory.push(taken);        
            println!("You take the {}.", item);
        }
        None => println!("There is no {} here.", item),
    }
}

fn drop_item(player: &mut Player, rooms: &mut HashMap<String, Room>, item: &str) {
    let room = match rooms.get_mut(&player.current_room) {
        Some(r) => r,
        None => return,
    };
    match player.inventory.iter().position(|i| i == item) {
        Some(index) => {
            let dropped = player.inventory.remove(index);
            room.items.push(dropped);
            println!("You drop the {}.", item);
        }
        None => println!("You aren't carrying a {}.", item),
    }
}


fn save_game(player: &Player) {
    match serde_json::to_string_pretty(player) {
        Ok(json) => {
            match fs::write("savegame.json", json) {
                Ok(_) => println!("Game saved."),
                Err(e) => println!("Could not write save file: {}", e),
            }
        }
        Err(e) => println!("Could not save game: {}", e),
    }
}

fn load_game() -> Option<Player> {
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