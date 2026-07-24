use std::collections::HashMap;
use crate::player::Player;
use crate::world::Room;

pub fn look(player: &Player, rooms: &HashMap<String, Room>) {
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

pub fn move_player(player: &mut Player, rooms: &HashMap<String, Room>, direction: &str) {
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

pub fn show_inventory(player: &Player) {
    if player.inventory.is_empty() {
        println!("Your inventory is empty.");
    } else {
        println!("You are carrying: {}", player.inventory.join(", "));
    }
}

pub fn take_item(player: &mut Player, rooms: &mut HashMap<String, Room>, item: &str) {
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

pub fn drop_item(player: &mut Player, rooms: &mut HashMap<String, Room>, item: &str) {
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