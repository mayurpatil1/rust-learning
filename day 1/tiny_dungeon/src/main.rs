#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Room {
    name : String,
    description:String,
}
#[derive(Debug)]
struct Player {
    name  : String,
    health : u32,
    gold:u32,
}

fn main() {

    let entrance = Room {
        name : String::from("Entrance hall"),
        description: String :: from("A damp stone room. An exit leads north.")
    };

    let mut hero = Player {
        name: String::from("Aria"),
        health: 100,
        gold: 21,
    };

    let heading = Direction::East;

    println!("You are {}, with {} HP and {} Gold", hero.name, hero.health, hero.gold);
    println!("You stand in the {}.", entrance.name);
    println!("{}", entrance.description);

    hero.health -= 20;
    println!("Ouch! HP now: {}", hero.health);

    let movement = match heading {
        Direction::North => "You move north into darkness.",
        Direction::South => "You head back south.",
        Direction::East  => "You go east.",
        Direction::West  => "You go west.",
    };

      println!("{}", movement);

    println!("Debug view -> {:?}", entrance);
    println!("Debug view -> {:?}", hero);
}