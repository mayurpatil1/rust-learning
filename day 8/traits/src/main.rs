// Day 8 — Traits (Rust's interfaces)
// Several creatures share behavior via a trait — NO inheritance, no `extends`.

// 1. DEFINE the trait: a contract of methods a type must provide.
//    Like Java's `interface Describable { String describe(); }`
trait Describable {
    // Required method — signature only, no body. Each type must supply one.
    fn describe(&self) -> String;

    // DEFAULT method — has a body. Types get it free unless they override it.
    // Note it calls describe(), which every implementor is guaranteed to have.
    fn greet(&self) -> String {
        format!("You see: {}", self.describe())
    }

    fn danger_level(&self) -> u32;
}

struct Goblin {
    health: u32,
}

struct Dragon {
    name: String,
    hoard: u32,
}

struct Chest {
    locked: bool,
}

struct Skeleton { bones: u32 }

// 2. IMPLEMENT the trait for each type — same `impl Trait for Type` shape as Display.
//    Each type fulfills the contract its OWN way. No shared parent class.

impl Describable for Goblin {
    fn describe(&self) -> String {
        format!("a snarling goblin ({} HP)", self.health)
    }
    fn danger_level(&self) -> u32 {
        3
    }
}

impl Describable for Dragon {
    fn describe(&self) -> String {
        format!("{}, a dragon guarding {} gold", self.name, self.hoard)
    }

    // Dragon OVERRIDES the default greet with something grander.
    fn greet(&self) -> String {
        format!("The ground trembles. {} regards you.", self.name)
    }

    fn danger_level(&self) -> u32 {
        3
    }
}

impl Describable for Chest {
    fn describe(&self) -> String {
        if self.locked {
            String::from("a heavy locked chest")
        } else {
            String::from("an open chest, empty inside")
        }
    }

    fn danger_level(&self) -> u32 {
        3
    }
    // Chest does NOT override greet — it uses the default.
}

impl Describable for Skeleton {
    fn describe(&self) -> String {
          format!("a skeleton of goblin has bones ({} )", self.bones)
    }
    fn danger_level(&self) -> u32 {
        10
    }
}

// 3. TRAIT BOUND: this function accepts ANY type that is Describable.
//    Like Java's `void announce(Describable thing)`.
//    It doesn't know or care whether it's a Goblin, Dragon, or Chest.
fn announce(thing: &impl Describable) {
    println!("{}", thing.greet());
}


fn is_threat(thing: &impl Describable) -> bool {
    thing.danger_level() > 5
}

fn main() {
    let goblin = Goblin { health: 12 };
    let dragon = Dragon { name: String::from("Vermithrax"), hoard: 5000 };
    let chest = Chest { locked: true };
    let skeleton = Skeleton {bones: 23};

    // Each type describes itself its own way:
    println!("- {}", goblin.describe());
    println!("- {}", dragon.describe());
    println!("- {}", chest.describe());
    println!("- {}", skeleton.describe());

    println!();

    // greet(): Goblin & Chest use the DEFAULT, Dragon uses its OVERRIDE.
    announce(&goblin); // default greet
    announce(&dragon); // overridden greet
    announce(&chest);  // default greet
    announce(&skeleton);  

    println!();
    println!("Is the dragon a threat? {}", is_threat(&dragon));
    println!("Is the chest a threat? {}", is_threat(&chest));
    println!("Is the skeleton a threat? {}", is_threat(&skeleton));
}