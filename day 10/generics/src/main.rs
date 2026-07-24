// Day 10 — Generics
// Write code once, use it with many types. Traits are the vocabulary of bounds.

use std::fmt::Display;

// ---------------------------------------------------------------
// 1. A generic function with ONE trait bound.
//    <T: PartialOrd> = "T must support comparison (>, <)".
//    Without the bound, `item > largest` wouldn't compile —
//    the compiler can't assume every type is comparable.
// ---------------------------------------------------------------
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// ---------------------------------------------------------------
// 2. MULTIPLE bounds with +. T must be both comparable AND printable.
// ---------------------------------------------------------------
fn announce_largest<T: PartialOrd + Display>(list: &[T]) {
    println!("The largest is: {}", largest(list));
}

// ---------------------------------------------------------------
// 3. Same thing with a `where` clause — identical meaning,
//    but easier to read when bounds get long.
// ---------------------------------------------------------------
fn describe_pair<T, U>(a: T, b: U) -> String
where
    T: Display,
    U: Display,
{
    format!("{} and {}", a, b)
}

// ---------------------------------------------------------------
// 4. A GENERIC STRUCT. Like Java's `class Box<T>`.
//    One definition works for any type.
// ---------------------------------------------------------------
#[derive(Debug)]
struct Container<T> {
    items: Vec<T>,
    label: String,
}

// The impl block needs <T> too: "for any T, Container<T> has these methods."
impl<T> Container<T> {
    fn new(label: &str) -> Container<T> {
        Container {
            items: Vec::new(),
            label: String::from(label),
        }
    }

    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn count(&self) -> usize {
        self.items.len()
    }

    fn first(&self) -> Option<&T> {
        self.items.first()
    }
}

// ---------------------------------------------------------------
// 5. A method available ONLY when T meets a bound.
//    Container<T> always has add/count, but only Display-able
//    containers get show(). Java can't express this cleanly.
// ---------------------------------------------------------------
impl<T: Display> Container<T> {
    fn show(&self) {
        println!("{} contains:", self.label);
        for item in &self.items {
            println!("  - {}", item);
        }
    }
}

fn main() {
    // ---- Generic function with different types ----
    let numbers = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];
    let words = vec!["pear", "apple", "zebra"];

    println!("Largest number: {}", largest(&numbers));
    println!("Largest char:   {}", largest(&chars));
    println!("Largest word:   {}", largest(&words));

    // Monomorphization: the compiler generated THREE concrete versions
    // of `largest` above — one for i32, one for char, one for &str.
    // No boxing, no casting, zero runtime cost.

    println!();
    announce_largest(&numbers);
    announce_largest(&words);

    // ---- Two different type parameters at once ----
    println!("\n{}", describe_pair(42, "apples"));
    println!("{}", describe_pair(3.5, 'x'));

    // ---- Generic struct, used with two different types ----
    let mut tools: Container<String> = Container::new("Toolbox");
    tools.add(String::from("hammer"));
    tools.add(String::from("wrench"));

    let mut scores: Container<i32> = Container::new("Scores");
    scores.add(95);
    scores.add(72);
    scores.add(88);

    println!();
    tools.show();
    scores.show();

    println!("\nToolbox has {} items.", tools.count());
    println!("First score: {:?}", scores.first());
}