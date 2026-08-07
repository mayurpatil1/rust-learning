// Day 23 — impl Trait vs Box<dyn Trait>  (static vs dynamic dispatch)
// How to store a MIX of different types that share a trait.

trait Describable {
    fn describe(&self) -> String;
    fn danger(&self) -> u32 {
        0 // default
    }
}

struct Goblin {
    health: u32,
}
struct Dragon {
    name: String,
}
struct Chest {
    locked: bool,
}

impl Describable for Goblin {
    fn describe(&self) -> String {
        format!("a goblin ({} HP)", self.health)
    }
    fn danger(&self) -> u32 {
        3
    }
}
impl Describable for Dragon {
    fn describe(&self) -> String {
        format!("{} the dragon", self.name)
    }
    fn danger(&self) -> u32 {
        95
    }
}
impl Describable for Chest {
    fn describe(&self) -> String {
        if self.locked {
            String::from("a locked chest")
        } else {
            String::from("an open chest")
        }
    }
    // uses default danger() = 0
}

// ---------------------------------------------------------------
// STATIC dispatch: impl Trait. One concrete type per call.
// Compiler knows the exact type, generates specialized code. Zero cost.
// ---------------------------------------------------------------
fn announce(thing: &impl Describable) {
    println!("You see {} [danger {}]", thing.describe(), thing.danger());
}

// ---------------------------------------------------------------
// DYNAMIC dispatch: a function taking a trait object.
// Works on ANY Describable, resolved at runtime.
// ---------------------------------------------------------------
fn announce_dyn(thing: &dyn Describable) {
    println!("You see {} [danger {}]", thing.describe(), thing.danger());
}

fn main() {
    let goblin = Goblin { health: 12 };
    let dragon = Dragon { name: String::from("Vermithrax") };
    let chest = Chest { locked: true };

    // Static dispatch — each call uses one known type.
    println!("--- static (impl Trait) ---");
    announce(&goblin);
    announce(&dragon);
    announce(&chest);

    // ---------------------------------------------------------------
    // The payoff: a MIXED collection behind one trait.
    // Box<dyn Describable> = "a heap value that is SOME Describable."
    // Box makes every element the same SIZE (a pointer), so Vec is happy.
    // ---------------------------------------------------------------
    println!("\n--- dynamic (Box<dyn Trait>) ---");
    let dungeon: Vec<Box<dyn Describable>> = vec![
        Box::new(Goblin { health: 8 }),
        Box::new(Dragon { name: String::from("Smaug") }),
        Box::new(Chest { locked: false }),
    ];

    // Iterate the mixed list — each element dispatches to its OWN describe().
    for thing in &dungeon {
        announce_dyn(thing.as_ref()); // as_ref(): &Box<dyn T> -> &dyn T
    }

    // Total danger across the whole mixed dungeon (Day 5 iterators still work):
    let total_danger: u32 = dungeon.iter().map(|thing| thing.danger()).sum();
    println!("\nTotal dungeon danger: {}", total_danger);

    // Count the truly dangerous ones:
    let dangerous = dungeon.iter().filter(|thing| thing.danger() > 50).count();
    println!("Seriously dangerous things: {}", dangerous);
}