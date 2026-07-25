// Day 11 — Collections & String vs &str
// Vec, HashMap, HashSet — all generic, all following the same ownership rules.

use std::collections::{HashMap, HashSet};

fn main() {
    // ===============================================================
    // PART 1: String vs &str  —  owned vs borrowed
    // ===============================================================
    let owned: String = String::from("adventurer");  // OWNS heap memory
    let literal: &str = "hello";                     // a view into the binary
    let slice: &str = &owned[0..5];                  // "adven" — a window into `owned`

    println!("owned:   {}", owned);
    println!("literal: {}", literal);
    println!("slice:   {}", slice);

    // Converting between them:
    let to_owned: String = literal.to_string();      // &str -> String (ALLOCATES)
    let to_borrowed: &str = owned.as_str();          // String -> &str (free, a view)
    println!("\nconverted: {} / {}", to_owned, to_borrowed);

    // RULE: take &str as a parameter (only need to look),
    //       return String when producing new data (someone must own it).
    println!("{}", shout("quiet"));    // pass a literal — no allocation
    println!("{}", shout(&owned));     // pass a String as &str — also free

    // ===============================================================
    // PART 2: Vec<T>  —  Java's ArrayList
    // ===============================================================
    let mut potions: Vec<String> = Vec::new();
    potions.push(String::from("healing"));
    potions.push(String::from("mana"));
    potions.push(String::from("speed"));

    println!("\n--- Vec ---");
    println!("all:    {:?}", potions);
    println!("count:  {}", potions.len());
    println!("first:  {:?}", potions.first());   // Option<&String> — safe
    println!("index:  {:?}", potions.get(1));    // Option — safe
    // potions[99] would PANIC; .get(99) returns None. Prefer .get().

    println!("has mana? {}", potions.contains(&String::from("mana")));

    potions.sort();                              // in place
    println!("sorted: {:?}", potions);

    let removed = potions.remove(0);              // returns the owned value
    println!("removed '{}', left: {:?}", removed, potions);

    // ===============================================================
    // PART 3: HashMap<K, V>  —  Java's HashMap
    // ===============================================================
    let mut inventory: HashMap<String, u32> = HashMap::new();
    inventory.insert(String::from("gold"), 250);
    inventory.insert(String::from("arrows"), 12);

    println!("\n--- HashMap ---");
    println!("gold: {:?}", inventory.get("gold"));      // Option<&u32>
    println!("gems: {:?}", inventory.get("gems"));      // None — missing key

    // .entry() — the method worth learning. "Get this key, or insert a default."
    // Like Java's computeIfAbsent / getOrDefault, but cleaner.
    *inventory.entry(String::from("gold")).or_insert(0) += 50;   // 250 -> 300
    *inventory.entry(String::from("potions")).or_insert(0) += 3; // new key, starts at 0

    println!("after entry(): {:?}", inventory);

    // Iterating gives (key, value) pairs:
    println!("Contents:");
    for (item, count) in &inventory {
        println!("  {} x{}", item, count);
    }

    // ===============================================================
    // PART 4: HashSet<T>  —  unique values only
    // ===============================================================
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(String::from("entrance"));
    visited.insert(String::from("hall"));
    visited.insert(String::from("entrance"));  // duplicate — silently ignored

    println!("\n--- HashSet ---");
    println!("visited {} unique rooms", visited.len());   // 2, not 3
    println!("been to the hall? {}", visited.contains("hall"));

    // Classic use: deduplicate a Vec
    let rolls = vec![3, 7, 3, 1, 7, 9, 1];
    let unique: HashSet<i32> = rolls.iter().copied().collect();
    println!("rolls {:?} -> {} unique values", rolls, unique.len());

    // Set operations (no clean Java equivalent without extra libraries):
    let mage_spells: HashSet<&str> = ["fireball", "shield", "heal"].into_iter().collect();
    let cleric_spells: HashSet<&str> = ["heal", "bless", "shield"].into_iter().collect();

    let shared: Vec<&&str> = mage_spells.intersection(&cleric_spells).collect();
    println!("\nboth classes know: {:?}", shared);


    let s = String::from("the quick brown fox the lazy dog the end");
    let mut counts: HashMap<&str, u32> = HashMap::new();

    for word in s.split_whitespace() {          // each word, e.g. "the"
        *counts.entry(word).or_insert(0) += 1;
    }

    println!("{:?}", counts);                    // "the": 3, others: 1
}

// Takes &str (only looks at it), returns String (creates new data).
fn shout(text: &str) -> String {
    format!("{}!", text.to_uppercase())
}