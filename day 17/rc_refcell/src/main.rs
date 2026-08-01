// Day 17 — Rc and RefCell
// The escape hatches from Rust's two core rules:
//   Rc<T>      -> break "one owner"      (multiple owners)
//   RefCell<T> -> break "one writer"     (mutate via a shared reference)
//   Rc<RefCell<T>> -> both at once (≈ Java's default shared, mutable object)

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // ===============================================================
    // 1. Rc<T> — multiple owners of the same data.
    // ===============================================================
    let treasure = Rc::new(String::from("golden crown"));
    println!("owners at start: {}", Rc::strong_count(&treasure)); // 1

    let guard_a = Rc::clone(&treasure); // NOT a deep copy — just another handle
    let guard_b = Rc::clone(&treasure);
    println!("owners now:      {}", Rc::strong_count(&treasure)); // 3

    println!("A guards the {}", guard_a);
    println!("B guards the {}", guard_b);

    {
        let guard_c = Rc::clone(&treasure);
        println!("owners in block: {}", Rc::strong_count(&treasure)); // 4
    } // guard_c dropped here — count goes back down

    println!("owners after block: {}", Rc::strong_count(&treasure)); // 3

    // ===============================================================
    // 2. RefCell<T> — mutate through a shared reference.
    //    The borrow rules move from COMPILE time to RUNTIME.
    // ===============================================================
    let counter = RefCell::new(0);

    *counter.borrow_mut() += 1;  // write handle
    *counter.borrow_mut() += 1;
    *counter.borrow_mut() += 1;

    println!("\ncounter: {}", counter.borrow()); // read handle -> 3

    // ===============================================================
    // 3. Rc<RefCell<T>> — MULTIPLE owners that can all MUTATE.
    //    This is essentially Java's default: a shared, mutable object.
    // ===============================================================
    let party_gold = Rc::new(RefCell::new(100));

    // Two "party members" share ownership of the same gold pool.
    let member_1 = Rc::clone(&party_gold);
    let member_2 = Rc::clone(&party_gold);

    *member_1.borrow_mut() += 50; // member 1 finds treasure
    *member_2.borrow_mut() -= 30; // member 2 buys a potion

    // All handles see the same updated value — they share ONE pool.
    println!("\nParty gold: {}", party_gold.borrow()); // 120
    println!("Seen by member_1: {}", member_1.borrow());
    println!("Seen by member_2: {}", member_2.borrow());

    // ===============================================================
    // 4. The RUNTIME cost of RefCell — uncomment to see it PANIC:
    //
    // let cell = RefCell::new(1);
    // let a = cell.borrow_mut();
    // let b = cell.borrow_mut(); // PANIC: already mutably borrowed
    // println!("{} {}", a, b);
    //
    // With plain &mut this would be a COMPILE error. RefCell defers the
    // same check to runtime — flexibility bought with a panic risk.
    // ===============================================================

    // One shared, mutable counter.
    let counter = Rc::new(RefCell::new(0u32));

    // Two owners, each a handle to the SAME counter.
    let worker_1 = Rc::clone(&counter);
    let worker_2 = Rc::clone(&counter);

    // Each owner mutates through its handle.
    *worker_1.borrow_mut() += 5;
    *worker_2.borrow_mut() += 3;

    // All handles see the same value — there is only one counter.
    println!("Final count: {}", counter.borrow());        // 8
    println!("owners: {}", Rc::strong_count(&counter));   // 3

}