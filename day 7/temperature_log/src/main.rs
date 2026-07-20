// Day 7 — Consolidation + a first trait (Display)
// A temperature log analyzer. Reinforces methods & iterators,
// and introduces Display: Rust's equivalent of Java's toString().

use std::fmt;

#[derive(Debug)]
struct Reading {
    day: String,
    celsius: f64,
}
#[derive(Debug)]
struct City {name: String }



impl Reading {
    // Associated function / factory (no self) — called as Reading::new(...)
    fn new(day: &str, celsius: f64) -> Reading {
        Reading {
            day: String::from(day),
            celsius,
        }
    }

    // &self method: read-only. Converts to Fahrenheit.
    fn fahrenheit(&self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }

    // &self, read-only. Returns true if this reading is at or below freezing.
    fn is_freezing(&self) -> bool {
        self.celsius <= 0.0
    }
}

// Implementing the Display trait = teaching Reading how to print with {}.
// This is Rust's version of overriding toString(). You CANNOT derive it —
// you decide how it should read to a human.
impl fmt::Display for Reading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:.1}°C ({:.1}°F)", self.day, self.celsius, self.fahrenheit())
    }
}

impl City {
    fn new(name: &str) -> City {
       City {
       name: String::from(name),
       }
    }
}

impl fmt::Display for City {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.name)
    }
}

fn main() {
    let readings = vec![
        Reading::new("Mon", 21.5),
        Reading::new("Tue", 24.0),
        Reading::new("Wed", 19.2),
        Reading::new("Thu", 26.8),
        Reading::new("Fri", 22.1),
        Reading::new("Sat", -3.0), 
    ];

    let city = vec![
        City::new("pune"),
        City::new("Mumbai"),
        City::new("Nagpur"),
    ];

     println!("This week's City:");
    for c in &city {
        println!(" {}", c);
    }

    // Because Reading implements Display, {} works directly. No :?, no clone.
    println!("This week's readings:");
    for r in &readings {
        println!("  {}", r); // uses our Display impl
    }

    // ---- Iterator queries (Day 5 skills) ----

    // Average temperature: map to the numbers, sum, divide.
    let total: f64 = readings.iter().map(|r| r.celsius).sum();
    let average = total / readings.len() as f64;
    println!("\nAverage: {:.1}°C", average);

    // Warmest day: max_by comparing the f64 values.
    // (f64 has no total ordering, so we compare with partial_cmp.)
    let warmest = readings
        .iter()
        .max_by(|a, b| a.celsius.partial_cmp(&b.celsius).unwrap());
    if let Some(r) = warmest {
        println!("Warmest: {}", r); // Display again
    }

    // Days above 22°C: filter, then collect their names.
    let hot_days: Vec<&str> = readings
        .iter()
        .filter(|r| r.celsius > 22.0)
        .map(|r| r.day.as_str())
        .collect();
    println!("Days above 22°C: {}", hot_days.join(", "));

    // How many days were below 20°C? filter + count.
    let cold_count = readings.iter().filter(|r| r.celsius < 20.0).count();
    println!("Days below 20°C: {}", cold_count);

    let freezing_count = readings.iter().filter(|r| r.is_freezing()).count();
    println!("Freezing days: {}", freezing_count);
}