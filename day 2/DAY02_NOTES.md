# Day 2 — Ownership & Borrowing

> Learning Rust by building a text adventure. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Goal today:** store the game world in a `HashMap` and move the player between rooms — which forces a first real encounter with the borrow checker.

**The big idea:** Every value in Rust has exactly one owner. The compiler tracks ownership and frees memory automatically when the owner goes out of scope — no garbage collector, no manual `free()`. This one rule is what the whole language is built on. Java hid all of this behind the GC; Rust makes it visible.

---

## 1. Ownership and moving

Assigning a value like a `String` to a new variable **moves** ownership rather than copying it. After the move, the original variable is invalid and the compiler forbids using it — there is only ever one owner at a time.

- Java: `String s2 = s1;` gives two references to the same object; the GC cleans up later.
- Rust: ownership transfers to `s2`, and `s1` can no longer be used.

When the single owner goes out of scope, Rust frees the memory automatically. This eliminates double-free and use-after-free bugs by construction.

---

## 2. Why numbers don't move: the `Copy` trait

Small, fixed-size, stack-only types (`u32`, `i32`, `bool`, `char`, etc.) implement the `Copy` trait — they are cheaply duplicated instead of moved, so the original stays valid.

Rule of thumb:

- **Copied:** numbers, `bool`, `char`
- **Moved:** `String`, structs, `Vec`, `HashMap`

This single distinction explains most early "value used after move" errors. It's also why yesterday `hero.gold` (a `u32`) could be printed repeatedly, while a `String` would have "disappeared" after the first use.

---

## 3. Borrowing with `&`

Moving a value every time you pass it to a function would be painful, so instead you **lend** a reference with `&`. This is called borrowing. The function can read the value, and when it returns you still own the original.

- `&` means: "look at this, but it's still mine."
- This is the closest thing to Java's default behavior of passing objects around — except in Rust it's explicit and visible in the type.

---

## 4. Mutable borrowing with `&mut` — and the one rule

If a borrower needs to **change** a value, you lend it mutably with `&mut`.

**The rule the borrow checker enforces — memorize this:**

> At any moment you may have **either** any number of shared `&` references (read-only) **OR** exactly one `&mut` reference (read-write) — **never both at once.**

- In Java, two threads mutating the same object is a data race found in production.
- In Rust, that situation is a **compile error**. This rule is the price you pay; "fearless concurrency" is what you buy.

Reading (`&`) never conflicts and is always allowed. A function that only reads its inputs should take `&` parameters; one that mutates takes `&mut`.

---

## Applying it: the room map

The game world became a `HashMap<String, Room>` (Rust's `HashMap` ≈ Java's `HashMap`), and the player tracks a `current_room` key. Two design facts worth remembering:

- A room **exists** when it's inserted into the `rooms` map.
- A room is **reachable** only when another room's `exits` map points to its key.

Rooms and the connections between them are two separate pieces of data — exactly how real game maps and graphs work.

The movement function borrowed the player mutably (`&mut Player`, because it changes location) and the rooms immutably (`&HashMap`, because it only reads). Since these are two different values, the borrow rule is satisfied.

---

## The errors I hit (and what they meant)

These three are the same underlying truth — **you cannot take ownership of something you only borrowed** — seen from different angles.

### 1. `expected String, found &String` (type mismatch)

`HashMap::get` returns a `&String` (a borrowed peek), but the field wanted an owned `String`. The type system keeps "a thing I own" and "a thing I'm only looking at" permanently distinct — the `&` is the visible marker of the difference. Java treats both as the same reference; Rust does not.

### 2. `E0507: cannot move out of ... behind a shared reference`

Trying to dereference (`*`) the borrowed value to force the `String` out. Illegal, because the `String` lives inside the `HashMap` and I only borrowed the map — moving it out would leave a hole. The error message even noted the value "does not implement the `Copy` trait," tying straight back to Day 1: a `u32` would have been fine, a `String` cannot be silently duplicated.

### 3. `unwrap()` on a `None` value (runtime panic)

A forgotten map insertion meant a room key had no entry. This one is different: it **compiled fine** and only failed at runtime. The type system can't know a map is missing a key. Key lesson: the difference between a **compile error** and a **runtime panic**, and what `.unwrap()` actually is — a promise that an `Option` has a value, with a crash if the promise is wrong.

---

## The fix, and why it's fine

`.clone()` makes an independent, owned copy of the borrowed value, leaving the original untouched inside the map. Rust makes you write `.clone()` **explicitly** so the cost is never hidden — no accidental deep copies. For something small like a room key, the cost is negligible; don't optimize it away yet. Later there are ways to avoid the clone (different key storage, or references with lifetimes), but reaching for `.clone()` to get unstuck is completely respectable while learning.

---

## Takeaways

- Every value has **one owner**; the owner going out of scope frees the value — no GC.
- Assignment **moves** owned types (`String`, structs, collections) and **copies** `Copy` types (numbers, `bool`, `char`).
- **Borrow** with `&` (read) or `&mut` (mutate) to use a value without taking ownership.
- The core rule: **many `&` XOR one `&mut`**, never both at once — this is what makes data races a compile error.
- You **cannot take ownership of something you only borrowed** — the type mismatch, the `E0507` move error, and the fix (`.clone()`) are all this one idea.
- **Compile error vs runtime panic** are different failure modes; `.unwrap()` turns a missing value into a crash.
- Reading compiler errors as **information** is the real skill — the fix is often in the error message itself.
