# Day 12 — Enums with Data & Exhaustive Matching

> Learning Rust by building small projects. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Project:** a standalone command parser — text turned into a data-carrying `Command` enum, then handled by `match`.

**Goal today:** the feature where Rust enums decisively beat Java's — variants that carry data.

---

## 1. Variants can carry data

Java enums are a fixed set of named constants — `Direction.NORTH`, nothing more. Rust enums start there but go further: **each variant can carry its own data, and different variants can carry different shapes.**

```rust
enum Command {
    Quit,                                    // no data
    Look,                                    // no data
    Go(String),                              // one String
    Take(String),                            // one String
    Attack { target: String, power: u32 },   // named fields, struct-style
}
```

A `Command` is **one of** these shapes. Impossible in Java without a class hierarchy (abstract `Command` + a subclass per variant). Rust says it in a few lines.

**Closest Java equivalent:** sealed classes/interfaces (Java 17+) — a fixed, known set of types. Rust enums are that, but lighter and with pattern matching built for them.

---

## 2. `match` destructures the data

Each arm can **pull the data out** in the same step it identifies the variant:

```rust
match command {
    Command::Quit => println!("Goodbye!"),
    Command::Go(direction) => println!("Going {}", direction),   // binds the String
    Command::Attack { target, power } => {                       // binds both fields
        println!("Hitting {} for {} damage", target, power);
    }
}
```

`Command::Go(direction)` detects the variant **and** extracts its inner value, naming it `direction`. That's **destructuring**.

**Ignoring parts:** `Command::Cast { spell, .. }` grabs `spell` and ignores the rest with `..`.

---

## 3. Exhaustiveness + data = a compiler checklist

The `run` function handled every variant with **no `_` catch-all** — on purpose.

Adding a new variant (`Drop`) without handling it produced:

```
error[E0004]: non-exhaustive patterns: `Command::Drop(_)` not covered
```

**This is the feature, not an annoyance.** When the enum grows, every `match` that doesn't handle the new variant fails to compile, handing over an exact to-do list. Day 1's exhaustiveness check + data-carrying variants = you can never forget to handle a case when the type expands. Java's `switch` over an enum would silently compile and do nothing.

---

## 4. Match guards — `match` gains `if`

An arm can add a condition beyond the shape:

```rust
Command::Cast { spell, mana } if mana >= 5 => println!("You cast {}!", spell),
Command::Cast { spell, .. }              => println!("Not enough mana to cast {}.", spell),
```

`if mana >= 5` is a **match guard** — the arm only fires if the pattern matches *and* the condition holds.

**Order matters:** the **guarded** arm must come first. If the unguarded `Cast { spell, .. }` were on top, it would swallow every cast, and the compiler would flag the guarded arm as unreachable.

---

## 5. The revelation: `Option` and `Result` ARE enums with data

Everything used since Day 3 is exactly this pattern:

- `enum Option<T> { Some(T), None }` — `Some(player)` is a variant carrying data, just like `Go(direction)`.
- `enum Result<T, E> { Ok(T), Err(E) }` — same idea.

Every `match` on `Some(room) => ...` was **destructuring an enum**. Today just names the machinery already in daily use. Modeling a custom outcome (`RollResult { Critical, Normal(u32), Fumble }`) is the same tool the standard library uses for `Option`/`Result`.

---

## 6. This is how the dungeon *should* parse commands

The dungeon matched `parts.as_slice()` against string patterns — it worked, but modeling commands as an enum is cleaner and type-safe. The parser's whole job becomes **`text -> Command`**, and everything downstream works with structured data instead of raw `&str` slices.

---

## Takeaways

- Rust enum **variants carry data** — tuple-style `Go(String)` or struct-style `Attack { target, power }` — and different variants can differ in shape.
- `match` **destructures**: it identifies the variant and binds its inner data in one step; `..` ignores the rest.
- **Exhaustiveness + data** makes `match` a compiler-enforced checklist — a new variant breaks every unhandled `match` with `E0004`.
- **Match guards** (`... if condition =>`) add a runtime condition to a pattern; put the guarded arm **first**.
- **`Option` and `Result` are just enums with data** — every `Some(x) =>` match has been destructuring an enum all along.
- Modeling input as an enum (`text -> Command`) makes downstream code type-safe instead of string-based.
