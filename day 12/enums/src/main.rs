#[derive(Debug)]
enum Command {
    Quit,
    Look,
    Go(String),
    Take(String),
    Drop(String),
    Attack { target: String, power :u32},
    Cast { spell: String, mana: u32 }
}

fn parse(input: &str) -> Command {
    let parts : Vec<&str> = input.split_whitespace().collect();
    match parts.as_slice() {
        ["quit"] => Command::Quit,
        ["look"] => Command::Look,
        ["go",dir] => Command::Go(dir.to_string()),
        ["take",item @ ..] => Command::Take(item.join(" ")),
        ["drop", who] => Command::Drop(who.to_string()),
        ["attack",target] => Command::Attack {
            target: target.to_string(),
            power:10,
        },
        ["cast",spell] => Command::Cast {
            spell:spell.to_string(),
            mana:10,
        },
        _ => Command::Look,
    }
}

fn run(command:Command) {
    match command {
        Command::Quit => println!("Farewell, adventurer!"),
        Command::Look => println!("You survey the room."),
        Command::Go(direction) => println!("You head {}.", direction),
        Command::Take(item) => println!("You pick up the {}.", item),
        Command::Drop(who) => println!("You drop the {}.", who),
        Command::Attack { target, power } => {
            println!("You strike the {} for {} damage!", target, power);
        }
        Command::Cast{spell, mana} if mana >= 5 => {
              println!("You cast {}!", spell);
        }
        Command::Cast{spell,..} => {
              println!("Not enough mana to cast {}.", spell);
        }
    }
}

enum RollResult {
    Critical,
    Normal(u32),
    Fumble,
}


fn describe_roll(roll:u32) -> RollResult {
    match roll {
        20 => RollResult::Critical,
        1 => RollResult::Fumble,
        n => RollResult::Normal(n),
    }
}

fn main () {
    let inputs = ["look", "go north", "take rusty key", "attack goblin", "quit","drop me", "cast fireball"];

    println!("--- Command demo ---");

    for input in inputs {
        let command = parse(input);
        print!("[{:>16}] -> ", input);
        run(command)
    }


     println!("\n--- Roll demo ---");
     for roll in [20,13,1,8] {
        match describe_roll(roll) {
            RollResult::Critical => println!("Rolled {}: CRITICAL HIT!", roll),
            RollResult::Fumble => println!("Rolled {}: fumble...", roll),
            RollResult::Normal(value) => println!("Rolled {}: a normal {}", roll, value),
        }
     }
}