# Day 18 — Idiomatic Error Handling

> Learning Rust by building small projects. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Project:** a standalone demo of `?`, a custom error enum, `thiserror`, and `anyhow`.

**Goal today:** go from `.unwrap()`-and-crash to how real Rust projects structure errors. Pulls together `Result` (Day 3), data-carrying enums (Day 12), and traits (Day 8).

---

## What was wrong with `.unwrap()` everywhere

`.unwrap()` / `.expect()` **crash** on failure. Real code needs errors that:

1. carry meaningful **information**,
2. can be different **types** in the same function, and
3. **propagate** cleanly without a pyramid of `match` statements.

Today's three tools solve exactly those.

---

## 1. The `?` operator — the workhorse

`?` replaces the whole match-on-`Result`-and-early-return dance with one character:

```rust
let number = text.parse::<i32>()?;
```

- **If `Ok`:** unwrap the value and continue.
- **If `Err`:** return that error from the whole function immediately.

**The one rule:** `?` only works in a function that itself returns `Result` (or `Option`) — the error needs somewhere to return **to**. Using `?` in `main` (returns `()`) won't compile unless `main` is changed to return `Result`.

The single most-used error feature in Rust. Compare to Day 3, where the same thing took nested `match` blocks.

---

## 2. Custom error enums — errors as data

When a function fails in several distinct ways, model them as an enum (Day 12 variants doing real work):

```rust
enum GameError {
    RoomNotFound(String),
    Locked(String),
    NotEnoughGold { needed: u32, have: u32 },
}
```

- Each variant is a **specific failure with specific details**.
- The caller can `match` and **branch per variant** — pull out `NotEnoughGold { needed, have }` and react precisely, or fall through to a generic arm.
- Far better than a string message: the caller can **inspect and branch** on what went wrong.

**Java analogy:** this is Rust's answer to an exception hierarchy — but as a **value you return**, not a thing you throw.

**To be a "real" error**, the type implements `Display` (its human-readable message). The standard `Error` trait wants this too.

---

## 3. `thiserror` — kill the boilerplate (for libraries)

Writing the `Display`/`Error` impls by hand is boilerplate. `thiserror` derives it from annotations:

```rust
#[derive(Debug, Error)]
enum GameError {
    #[error("no room called '{0}'")]
    RoomNotFound(String),
    #[error("need {needed} gold, only have {have}")]
    NotEnoughGold { needed: u32, have: u32 },
}
```

- **Part A win:** replacing the hand-written `impl Display` with these `#[error(...)]` attributes works **identically** — `thiserror` generated the exact `Display` code, from the annotations.
- `{0}` = first tuple field; `{needed}` = named field.

---

## 4. `anyhow` — just make it work (for applications)

When precise error _types_ don't matter and you only want "it failed, with context":

```rust
fn parse_config(input: &str) -> anyhow::Result<i32> {
    let value: i32 = input.parse()?;  // parse's error auto-converts to anyhow::Error
    if value < 0 {
        anyhow::bail!("must be non-negative, got {}", value);
    }
    Ok(value)
}
```

- `anyhow::Result<T>` accepts **any** error via `?` — different error sources, one clean signature.
- `anyhow::bail!("...")` creates an error from a message and returns early.

---

## 5. The rule of thumb

- **Libraries → `thiserror`.** Callers need **precise** error types to match on.
- **Applications → `anyhow`.** You just want propagation to work; types don't matter.

---

## Takeaways

- `.unwrap()` crashes; idiomatic errors carry info, vary in type, and propagate.
- **`?`** = "unwrap `Ok`, or return the `Err` now" — only usable in a function returning `Result`/`Option`.
- **Custom error enums** make errors _data_ the caller can inspect and branch on — Rust's return-a-value alternative to thrown exceptions.
- An error type implements **`Display`** for its message.
- **`thiserror`** derives all the error boilerplate from `#[error("...")]` annotations — for libraries.
- **`anyhow`** accepts any error via `?` for a "just works" app-level signature — for applications.
- Chained `?` (as in `make_purchases`) propagates the first failure and skips the rest; the caller decides how to handle it.
