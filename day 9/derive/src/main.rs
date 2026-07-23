// Day 9 — Derive macros and the common traits
// `derive` = the compiler auto-writes a trait implementation for you.

use std::fmt;

// ---------------------------------------------------------------
// 1. Debug — enables {:?}. The one used since Day 1.
// 2. Clone — enables .clone(), an explicit copy.
// 3. PartialEq — enables == and !=, comparing field by field.
// ---------------------------------------------------------------
#[derive(Debug, PartialEq)]
struct Item {
    name: String,
    value: u32,
}

// ---------------------------------------------------------------
// Copy: assignment DUPLICATES instead of MOVES.
// Only possible because every field here is itself Copy (u32, u32).
// A struct containing a String could NEVER be Copy — String owns heap memory.
// Copy always requires Clone alongside it.
// ---------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: u32,
    y: u32,
}

// ---------------------------------------------------------------
// Default: gives Type::default() with every field at its zero value.
// (0 for numbers, "" for String, false for bool, empty Vec, etc.)
// ---------------------------------------------------------------
#[derive(Debug, Default)]
struct Settings {
    difficulty: u32,
    player_name: String,
    sound_on: bool,
}

// Display CANNOT be derived — only you know the human-facing format.
// This is the hand-written one from Day 7, for contrast.
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (worth {} gold)", self.name, self.value)
    }
}

fn main() {
    // ---- Debug: {:?} prints the whole struct ----
    let sword = Item { name: String::from("sword"), value: 150 };
    println!("Debug view:   {:?}", sword);
    println!("Display view: {}", sword); // hand-written impl

    // ---- Clone: an explicit copy. Both remain usable. ----
    let backup = sword.clone();
    println!("\nOriginal: {:?}", sword);   // still valid — clone() didn't move it
    println!("Backup:   {:?}", backup);

    // ---- PartialEq: == compares field by field ----
    let same = Item { name: String::from("sword"), value: 150 };
    let different = Item { name: String::from("shield"), value: 80 };
    println!("\nsword == same?      {}", sword == same);
    println!("sword == different? {}", sword == different);

    // ---- Copy: assignment duplicates, no move, no .clone() needed ----
    let start = Position { x: 0, y: 0 };
    let end = start; // COPIED, not moved
    println!("\nstart is still usable: {:?}", start); // works! Copy types don't move
    println!("end:                   {:?}", end);
    println!("start == end?          {}", start == end);

    // Contrast: Item is NOT Copy (it holds a String), so this would MOVE:
    //   let moved = sword;
    //   println!("{:?}", sword); // ERROR: value borrowed after move
    // That's the Day 2 rule, now with a name: Copy vs non-Copy.

    // ---- Default: every field at its zero value ----
    let defaults = Settings::default();
    println!("\nDefault settings: {:?}", defaults);

    // Override just some fields, take the rest from default (..):
    let custom = Settings {
        difficulty: 3,
        ..Default::default() // "fill the remaining fields with defaults"
    };
    println!("Custom settings:  {:?}", custom);
}