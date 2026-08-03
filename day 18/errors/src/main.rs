// Day 18 — Idiomatic error handling
// The ? operator, custom error enums, and how real projects structure errors.
//
// Cargo.toml needs (run these):
//   cargo add thiserror
//   cargo add anyhow

use std::fmt;

// ===================================================================
// PART 1: A custom error enum, written BY HAND.
// Each variant is a distinct failure carrying its own details (Day 12).
// ===================================================================
#[derive(Debug)]
enum GameError {
    RoomNotFound(String),
    Locked(String),
    NotEnoughGold { needed: u32, have: u32 },
}

// To be a "real" error, a type implements Display (its message).
// (The standard Error trait also wants this.)
impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::RoomNotFound(name) => write!(f, "no room called '{}'", name),
            GameError::Locked(dir) => write!(f, "the way {} is locked", dir),
            GameError::NotEnoughGold { needed, have } => {
                write!(f, "need {} gold, only have {}", needed, have)
            }
        }
    }
}

// A function returning our custom error. Note the return type: Result<_, GameError>.
fn buy(item: &str, cost: u32, gold: u32) -> Result<u32, GameError> {
    if gold < cost {
        return Err(GameError::NotEnoughGold { needed: cost, have: gold });
    }
    println!("Bought {} for {} gold.", item, cost);
    Ok(gold - cost) // remaining gold
}

// ===================================================================
// PART 2: The ? operator — propagate errors without match pyramids.
// This function calls buy() twice; if either fails, ? returns the error.
// ===================================================================
fn shopping_trip(starting_gold: u32) -> Result<u32, GameError> {
    let gold = buy("sword", 100, starting_gold)?; // ? unwraps Ok or returns Err
    let gold = buy("shield", 60, gold)?;          // uses remaining gold
    let gold = buy("potion", 25, gold)?;
    Ok(gold)
}

// ===================================================================
// PART 3: anyhow — for APPLICATIONS. Accepts ANY error type via ?.
// Great when you just want "it failed, with context," not precise types.
// ===================================================================
fn parse_config(input: &str) -> anyhow::Result<i32> {
    let value: i32 = input.parse()?; // parse's error auto-converts into anyhow::Error
    if value < 0 {
        anyhow::bail!("config value must be non-negative, got {}", value);
    }
    Ok(value)
}

fn withdraw(amount: u32, balance: u32) -> Result<u32, GameError> {
    if amount > balance {
        return Err(GameError::NotEnoughGold { needed: amount, have: balance });
    }
    Ok(balance - amount) 
}

fn make_purchases(mut balance: u32) -> Result<u32, GameError> {
    balance = withdraw(50, balance)?; 
    balance = withdraw(30, balance)?;
    balance = withdraw(100, balance)?; 
    Ok(balance)
}

fn main() {
    // ---- Custom error, handled by matching each variant ----
    println!("--- Custom errors ---");
    match shopping_trip(200) {
        Ok(remaining) => println!("Trip done, {} gold left.", remaining),
        Err(GameError::NotEnoughGold { needed, have }) => {
            println!("Couldn't afford it: needed {}, had {}.", needed, have);
        }
        Err(other) => println!("Shopping failed: {}", other),
    }

    // A trip that runs out of money partway:
    match shopping_trip(120) {
        Ok(remaining) => println!("Trip done, {} gold left.", remaining),
        Err(e) => println!("Shopping failed: {}", e), // uses Display
    }

    match make_purchases(120) {
    Ok(left) => println!("All purchases done, {} gold left.", left),
    Err(e) => println!("Purchase failed: {}", e),
}

    // ---- anyhow: different error sources, one clean signature ----
    println!("\n--- anyhow ---");
    for input in ["42", "-5", "not a number"] {
        match parse_config(input) {
            Ok(v) => println!("'{}' -> config {}", input, v),
            Err(e) => println!("'{}' -> error: {}", input, e),
        }
    }
}


//----

// use thiserror::Error;

// #[derive(Debug, Error)]
// enum GameError {
//     #[error("no room called '{0}'")]
//     RoomNotFound(String),
//     #[error("the way {0} is locked")]
//     Locked(String),
//     #[error("need {needed} gold, only have {have}")]
//     NotEnoughGold { needed: u32, have: u32 },
// }