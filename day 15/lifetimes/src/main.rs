// Day 15 — Lifetimes
// Goal: READ lifetimes with understanding. A lifetime is a NAME the compiler
// uses to track "how long is this reference valid?" — a label, not a lever.
// It only enforces the rule you've known since Day 3's E0716:
// a reference can never outlive what it points to.

// ---------------------------------------------------------------
// 1. NO lifetime needed — the compiler infers it (elision).
//    Returns a reference derived from ONE input, so the relationship
//    is obvious: the output borrows from `text`.
// ---------------------------------------------------------------
fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(index) => &text[..index],
        None => text,
    }
}

// ---------------------------------------------------------------
// 2. Lifetime REQUIRED — returns a reference from ONE OF TWO inputs.
//    The compiler can't tell which, so YOU describe the relationship:
//    all three references share lifetime 'a → the returned reference
//    is valid only as long as BOTH inputs are.
//    You are not changing lifespans; you're stating the relationship.
// ---------------------------------------------------------------
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

// ---------------------------------------------------------------
// 3. A struct that HOLDS a reference needs a lifetime.
//    It says: "an Excerpt cannot outlive the text it borrows from."
//    Without this, the struct could hold a dangling reference.
// ---------------------------------------------------------------
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn announce(&self) -> &str {
        self.part
    }
}

fn main() {
    // Case 1 — no annotation anywhere, just works (elision).
    let sentence = String::from("the quick brown fox");
    println!("First word: {}", first_word(&sentence));

    // Case 2 — the annotated function. Both inputs live through the call,
    // so the returned reference is valid here.
    let a = String::from("short");
    let b = String::from("a much longer string");
    let result = longest(&a, &b);
    println!("Longest: {}", result);

    // Case 3 — a struct borrowing from `sentence`.
    // `sentence` lives longer than `excerpt`, so this is sound.
    let first = first_word(&sentence);
    let excerpt = Excerpt { part: first };
    println!("Excerpt: {}", excerpt.announce());

    // ---------------------------------------------------------------
    // WHY THE RULE EXISTS — uncomment to see the compiler stop a dangling ref:
    //
    // let result;
    // {
    //     let temporary = String::from("I will not live long");
    //     result = longest(&sentence, &temporary); // result may borrow `temporary`
    // } // `temporary` is dropped HERE
    // println!("{}", result); // ERROR: `temporary` does not live long enough
    //
    // The lifetime 'a is what lets the compiler PROVE this is unsafe.
    // ---------------------------------------------------------------
}