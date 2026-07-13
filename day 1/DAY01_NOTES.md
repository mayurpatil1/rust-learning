# Day 1 — Structs, Enums, `let`/`mut`, and `match`

> Learning Rust by building a text adventure. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Goal today:** stop hardcoding the game and turn the world into real data.

---

## 1. `let` vs `let mut` — immutable by default

Rust flips Java's default. In Java everything is reassignable unless you add `final`. In Rust bindings are locked unless you add `mut`.

```rust
let health = 100;      // like Java `final int` — cannot change
let mut score = 0;     // opt in to mutability
score = 10;            // OK
// health = 90;        // COMPILE ERROR
```

| Java | Rust |
|------|------|
| `int x = 5;` (reassignable) | `let mut x = 5;` |
| `final int x = 5;` (locked) | `let x = 5;` |

**Why:** with no garbage collector, the compiler needs to know exactly what can change to guarantee memory safety.

---

## 2. `struct` — a fields-only class

A `struct` is like a Java class with only fields. Methods live in a separate `impl` block (covered later). No `new` keyword and no default constructor — you name every field when creating one.

```rust
struct Player {
    name: String,
    health: u32,   // unsigned 32-bit int — Rust makes size & signedness explicit
}

let hero = Player {
    name: String::from("Aria"),
    health: 100,
};
```

---

## 3. `enum` — stronger than Java's enum

Starts like a Java enum (fixed set of named values) but each variant can later carry data — closer to Java sealed classes / records. Today we use the simple form.

```rust
enum Direction {
    North,
    South,
    East,
    West,
}
```

---

## 4. `match` — `switch` with superpowers

- The compiler forces you to handle **every** variant (exhaustiveness checking).
- It's an **expression** — it returns a value you can assign.

```rust
let msg = match heading {
    Direction::North => "You move north.",
    Direction::South => "You move south.",
    Direction::East  => "You go east.",
    Direction::West  => "You go west.",
};
```

Use `_` as a catch-all (like Java's `default`) — but only when you truly mean "everything else," because it disables the safety below.

---

## Key lesson: exhaustiveness checking

Removing the `Direction::West` arm produced:

```
error[E0004]: non-exhaustive patterns: `Direction::West` not covered
```

**Why this matters vs Java:** a Java `switch` with a missing enum case compiles fine and silently does nothing — the bug ships and surfaces in production. Rust proves at **compile time** that every variant is handled, because an enum is a *closed* set. Add a new variant later and every `match` that doesn't handle it fails to compile, handing you an exact to-do list. The compiler works *for* you.

---

## Bonus insight: `Copy` vs move (preview of Day 2)

Printing `hero.gold` (a `u32`) multiple times works fine because small number types are **`Copy`** — they're cheaply duplicated, original untouched. A `String` would instead **move** and could "disappear." That contrast is what Day 2 (ownership) is all about.

---

## Formatting note

`println!` uses positional `{}` placeholders — like Java's `String.format`, but Rust picks the display logic from the type, so no `%s` / `%d`.

```rust
println!("You are {}, with {} HP and {} Gold", hero.name, hero.health, hero.gold);
```

`#[derive(Debug)]` above a type enables `{:?}` to print the whole struct — an auto-generated `toString()`-like view. (Mechanics covered on Day 9.)

---


## Takeaways

- Bindings are **immutable by default**; add `mut` to opt in.
- `struct` = fields; behavior lives in `impl` blocks, not inside the struct.
- Rust **enums can hold data** and are a closed set.
- `match` is exhaustive and returns a value; reach for `_` sparingly.
- Reading compiler errors as **information** is the core skill — the exhaustiveness error caught a real bug class for free.


