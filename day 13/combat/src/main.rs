// Days 13-14 — Consolidation: a combat system + unit tests
// Combines methods, traits, data-carrying enums, and iterators,
// then adds Rust's built-in testing.

// A data-carrying enum (Day 12) for what happens when an attack lands.
#[derive(Debug, PartialEq)]
enum AttackResult {
    Hit(u32),   // damage dealt
    Killed,     // target died
    Missed,     // no damage
}

// A trait (Day 8): anything that can take part in combat.
trait Combatant {
    fn name(&self) -> &str;
    fn health(&self) -> u32;
    fn is_alive(&self) -> bool {
        self.health() > 0 // default method
    }
}

#[derive(Debug)]
struct Enemy {
    name: String,
    health: u32,
    power: u32,
}

impl Enemy {
    fn new(name: &str, health: u32, power: u32) -> Enemy {
        Enemy {
            name: String::from(name),
            health,
            power,
        }
    }

    // Take damage; return a structured result describing what happened.
    fn take_damage(&mut self, amount: u32) -> AttackResult {
        if amount == 0 {
            return AttackResult::Missed;
        }
        // saturating_sub: subtract but never go below 0 (no underflow panic)
        self.health = self.health.saturating_sub(amount);
        if self.health == 0 {
            AttackResult::Killed
        } else {
            AttackResult::Hit(amount)
        }
    }

    fn heal(&mut self, amount: u32) {
        self.health += amount;
    }
}

impl Combatant for Enemy {
    fn name(&self) -> &str {
        &self.name
    }
    fn health(&self) -> u32 {
        self.health
    }
}

fn main() {
    let mut goblin = Enemy::new("Goblin", 30, 5);
    println!("A {} appears with {} HP!", goblin.name(), goblin.health());

    // A sequence of attacks, handled by destructuring the enum result.
    let attacks = [10, 0, 12, 15];
    for damage in attacks {
        let result = goblin.take_damage(damage);
        match result {
            AttackResult::Hit(dealt) => {
                println!("Hit for {}! {} has {} HP left.", dealt, goblin.name(), goblin.health());
            }
            AttackResult::Missed => println!("You missed!"),
            AttackResult::Killed => println!("You slew the {}!", goblin.name()),
        }
        if !goblin.is_alive() {
            break;
        }
    }
}

// ===================================================================
// TESTS — compiled only during `cargo test`, zero cost to the real build.
// ===================================================================
#[cfg(test)]
mod tests {
    use super::*; // bring in Enemy, AttackResult, etc.

    #[test]
    fn new_enemy_has_full_health() {
        let e = Enemy::new("orc", 40, 8);
        assert_eq!(e.health(), 40);
        assert_eq!(e.name(), "orc");
    }

    #[test]
    fn taking_damage_reduces_health() {
        let mut e = Enemy::new("orc", 40, 8);
        let result = e.take_damage(10);
        assert_eq!(result, AttackResult::Hit(10));
        assert_eq!(e.health(), 30);
    }

    #[test]
    fn lethal_damage_kills() {
        let mut e = Enemy::new("rat", 5, 1);
        let result = e.take_damage(10); // more than its health
        assert_eq!(result, AttackResult::Killed);
        assert_eq!(e.health(), 0);      // saturating_sub kept it at 0, no underflow
        assert!(!e.is_alive());
    }

    #[test]
    fn zero_damage_misses() {
        let mut e = Enemy::new("orc", 40, 8);
        assert_eq!(e.take_damage(0), AttackResult::Missed);
        assert_eq!(e.health(), 40); // unchanged
    }

     #[test]
    fn damage_accumulates() {
        let mut e = Enemy::new("troll", 50, 10);
        e.take_damage(15);   
        e.take_damage(20);  
        assert_eq!(e.health(), 15);
    }

    #[test]
    fn healing_restores_health() {
        let mut e = Enemy::new("orc", 40, 8);
        e.take_damage(25);       
        assert_eq!(e.health(), 15);
        e.heal(10);             
        assert_eq!(e.health(), 25);
    }
}