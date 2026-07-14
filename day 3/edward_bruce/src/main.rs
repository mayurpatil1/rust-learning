use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
struct Room {
    name: String,
    description: String,
    exits: HashMap<String, String>,
}

#[derive(Debug)]
struct Player {
    name: String,
    health: u32,
    current_room: String,
}

fn main() {
    let rooms = build_world();

    let mut hero = Player {
        name: String::from("Aria"),
        health: 100,
        current_room: String::from("entrance"),
    };

    println!("=== The Tiny Dungeon ===");
    println!("Commands: look, go <direction>, quit\n");
    look(&hero, &rooms);

    loop {
        print!("\n> ");
        io::stdout().flush().unwrap(); // force the prompt to appear before we read

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match parts.as_slice() {
            ["quit"] | ["exit"] => {
                println!("Farewell, {}!", hero.name);
                break;
            }
            ["look"] => look(&hero, &rooms),
            ["go", direction] => move_player(&mut hero, &rooms, direction),
            [] => continue, // empty line, just re-prompt
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
    });

    let mut hall_exits = HashMap::new();
    hall_exits.insert(String::from("south"), String::from("entrance"));
    hall_exits.insert(String::from("down"), String::from("cellar"));
    rooms.insert(String::from("hall"), Room {
        name: String::from("Great Hall"),
        description: String::from("A vast hall wrapped in cobwebs. Exits lead south and down."),
        exits: hall_exits,
    });

    let mut cellar_exits = HashMap::new();
    cellar_exits.insert(String::from("up"), String::from("hall"));
    rooms.insert(String::from("cellar"), Room {
        name: String::from("Dusty Cellar"),
        description: String::from("A cramped cellar smelling of old wine. Stairs lead up."),
        exits: cellar_exits,
    });

    rooms
}

fn look(player: &Player, rooms: &HashMap<String, Room>) {
    match rooms.get(&player.current_room) {
        Some(room) => {
            println!("You are in the {}.", room.name);
            println!("{}", room.description);
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