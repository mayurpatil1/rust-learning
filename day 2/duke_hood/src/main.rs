use std::collections::HashMap;

#[derive(Debug)]
struct Room {
    name: String,
    description: String,
    exits: HashMap<String,String>
}

#[derive(Debug)]
struct Player {
    name: String,
    health: u32,
    current_room: String,
}

fn main() {

    let mut rooms: HashMap<String, Room> = HashMap::new();

    let mut entrance_exits = HashMap::new();
    entrance_exits.insert(String::from("north"), String::from("hall"));

    rooms.insert(
        String::from("entrance"),
        Room {
            name: String::from("Entrance Hall"),
            description: String::from("A damp stone room. An exit leads north."),
            exits: entrance_exits,
        },
    );


    let mut hall_exits = HashMap::new();
    hall_exits.insert(String::from("south"), String::from("entrance"));
    hall_exits.insert(String::from("down"), String::from("cellar"));
      rooms.insert(                                  
        String::from("hall"),
        Room {
            name: String::from("Great Hall"),
            description: String::from("A vast hall wrapped in cobwebs. Exits lead south and down."),
            exits: hall_exits,
        },
    );

    let mut cellar_exits = HashMap::new();
    cellar_exits.insert(String::from("up"), String::from("hall")); 
    rooms.insert(
        String::from("cellar"),
        Room {
            name: String::from("Dusty Cellar"),
            description: String::from("A cramped cellar smelling of old wine. Stairs lead up."),
            exits: cellar_exits,
        },
    ); 


    let mut hero = Player {
        name: String::from("Aria"),
        health: 100,
        current_room: String::from("entrance"),
    };

    let current = rooms.get(&hero.current_room).unwrap();
    println!("\nYou are now in the {}.", current.name);
    println!("{}", current.description);

    move_player(&mut hero, &rooms, "north");
    print_location(&hero, &rooms);

  
    move_player(&mut hero, &rooms, "down");
    print_location(&hero, &rooms);

  
    move_player(&mut hero, &rooms, "up");
    print_location(&hero, &rooms);

   
    move_player(&mut hero, &rooms, "south");
    print_location(&hero, &rooms);
   

}

  fn move_player(player:&mut Player, rooms:&HashMap<String,Room>, direction:&str) {
        let current = rooms.get(&player.current_room).unwrap();
        match current.exits.get(direction) {
            Some(next_room_key) => {
                player.current_room = next_room_key.clone();
                println!("You move {}.", direction);
            } 
            None => {
                   println!("You can't go {} from here.", direction);
            }
        }
    }


fn print_location(player: &Player, rooms: &HashMap<String, Room>) {
    let current = rooms.get(&player.current_room).unwrap();
    println!("\nYou are in the {}.", current.name);
    println!("{}", current.description);
}    