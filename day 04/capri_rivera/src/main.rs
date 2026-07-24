mod commands;
mod player;
mod save;
mod world;

use std::io::{self, Write};

use commands::{drop_item, look, move_player, show_inventory, take_item};
use player::Player;
use save::{load_game, save_game, GameState};
use world::build_world;

fn main() {
    // One variable owns the whole game. No more separate `hero` and `rooms`.
    let mut state = GameState {
        player: Player {
            name: String::from("Aria"),
            health: 100,
            current_room: String::from("entrance"),
            inventory: Vec::new(),
        },
        rooms: build_world(),
    };

    println!("=== The Tiny Dungeon ===");
    println!("Commands: look, go <direction>, take <item>, drop <item>, inventory, save, load, quit\n");
    look(&state.player, &state.rooms);

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
                println!("Farewell, {}!", state.player.name);
                break;
            }
            ["look"] => look(&state.player, &state.rooms),
            ["go", direction] => move_player(&mut state.player, &state.rooms, direction),
            ["inventory"] | ["inv"] => show_inventory(&state.player),
            ["take"] => println!("Take what?"),
            ["take", item_words @ ..] => {
                let item = item_words.join(" ");
                take_item(&mut state.player, &mut state.rooms, &item);
            }
            ["drop"] => println!("Drop what?"),
            ["drop", item_words @ ..] => {
                let item = item_words.join(" ");
                drop_item(&mut state.player, &mut state.rooms, &item);
            }
            // Saving now just BORROWS the state. Nothing moves.
            ["save"] => save_game(&state),
            // Loading replaces the whole state in one assignment.
            ["load"] => match load_game() {
                Some(loaded) => {
                    state = loaded;
                    println!("Game loaded. Welcome back, {}!", state.player.name);
                    look(&state.player, &state.rooms);
                }
                None => println!("Could not load a game."),
            },
            [] => continue,
            _ => println!("I don't understand that command."),
        }
    }
}