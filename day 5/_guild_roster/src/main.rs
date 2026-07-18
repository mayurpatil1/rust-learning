// Day 5 — Iterators & Closures
// A standalone project: analyze a guild roster.
// Almost every line here has a Java Streams equivalent — noted in comments.

#[derive(Debug)]
struct Adventurer {
    name: String,
    class: String,
    level: u32,
    gold: u32,
}

fn main () {
    let mut roster = vec![
        Adventurer { name: String::from("Aria"),  class: String::from("Mage"),   level: 12, gold: 340 },
        Adventurer { name: String::from("Borin"), class: String::from("Warrior"),level: 8,  gold: 120 },
        Adventurer { name: String::from("Cael"),  class: String::from("Rogue"),  level: 15, gold: 780 },
        Adventurer { name: String::from("Dain"),  class: String::from("Warrior"),level: 5,  gold: 50  },
        Adventurer { name: String::from("Elyn"),  class: String::from("Mage"),   level: 20, gold: 990 },

    ];


    let is_high_level = |a : &Adventurer| a.level >= 10;
    let describe = |a: &Adventurer| format!("{} the level-{} {}", a.name, a.level, a.class);

    let raise = 100;
    let with_raise = |g :u32| g+raise;
    println!("A purse of 50 gold after a raise: {}\n",  with_raise(50));


    let names : Vec<String> = roster
    .iter()
    .map(|a| a.name.clone())
    .collect();

    println!("Guild members: {}", names.join(", "));

    let vecterans : Vec<&Adventurer> = roster
    .iter()
    .filter(|a| is_high_level(a))
    .collect();

    println!("\nVeterans (level 10+):");

    for a in &vecterans {
        println!("  - {}", describe(a));
    }

    let total_gold : u32 = roster.iter().map(|a| a.gold).sum();
    let average_gold = total_gold as f64 / roster.len() as f64;

    println!("\nTreasury total: {} gold", total_gold);
    println!("Average per member: {:.1} gold", average_gold);


    let target = "Cael";
    let level = roster
    .iter()
    .find(|a| a.name == target)
    .map(|a| a.level);

    match level{
         Some(l) => println!("\n{} is level {}.", target, l),
         None => println!("\nNo adventurer named {}.", target),
    }


    println!("\nRoster:");
    for (index, a) in roster.iter().enumerate() {
        println!("  {}. {}", index + 1, a.name); // +1 because enumerate starts at 0
    }

    for a in roster.iter_mut() {
        a.gold += 100;
    }

    let new_total: u32 = roster.iter().map(|a| a.gold).sum();
    println!("\nAfter a 100-gold bonus each, treasury is now {} gold.", new_total);


    let all_names : Vec<String> = roster.into_iter().map(|a| a.name).collect();
    println!("\nFinal roll call (roster consumed): {}", all_names.join(", "));
}