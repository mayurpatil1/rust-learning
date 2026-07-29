# Days 13–14 — Combat System + Unit Testing (Consolidation)

> Learning Rust by building small projects. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Project:** a standalone combat system (an `Enemy`, attacks, health), then unit tests for it.

**Goal:** combine Week 2's tools in one program, and learn Rust's **built-in** testing.

---

## 1. The consolidation — nothing new, everything together

The combat code reused the whole toolkit:

- **Data-carrying enum** (Day 12): `AttackResult { Hit(u32), Killed, Missed }` — the outcome of an attack, matched by destructuring.
- **Trait with default method** (Day 8): `Combatant` with `name()`, `health()`, and a default `is_alive()`.
- **Struct + methods** (Day 6): `Enemy` with `new` (associated function) and `take_damage` (`&mut self`).
- **Iterators** (Day 5): looping a sequence of attacks.

Following all of it comfortably is the real signal that Week 2 has landed.

### One small new method: `saturating_sub`

`self.health.saturating_sub(amount)` subtracts but **stops at 0** instead of underflowing.

- Plain `30u32 - 40` **panics** in debug builds — unsigned integers can't go negative.
- `saturating_sub` is the safe way to subtract health (or any `u32` that shouldn't go below zero). A common source of beginner panics.

---

## 2. Testing is built into the language

Java needs JUnit (a separate framework). In Rust, testing is **part of the language and `cargo`** — no dependency, no setup.

- `#[test]` — an attribute (like `#[derive(...)]`) marking a function as a test.
- `cargo test` — finds and runs every test function.
- A test **passes if it runs without panicking**; a failed assertion panics, and the runner catches and reports it.

### Assertion macros

| Macro              | Fails when                        |
| ------------------ | --------------------------------- |
| `assert_eq!(a, b)` | `a != b` (≈ JUnit `assertEquals`) |
| `assert_ne!(a, b)` | `a == b`                          |
| `assert!(cond)`    | `cond` is `false`                 |

---

## 3. The test module pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn my_test() { ... }
}
```

- `#[cfg(test)]` — compile this module **only** during `cargo test`; tests add **zero weight** to the real build.
- `use super::*` — pull in the code under test from the parent module (the file above).
- Tests live in the **same file** as the code they test — no separate test directory needed for unit tests.

---

## 4. Writing good tests (from the challenges)

- **Test a scenario, assert an outcome.** `damage_accumulates` hits an enemy twice and checks cumulative health — catching any bug where damage overwrites instead of subtracting.
- **Assert intermediate state.** `healing_restores_health` checks health after the damage _and_ after the heal, so a failure immediately reveals which method was wrong.
- **Testing side effects is valid.** A test can ignore a method's return value and check the mutation it caused (health dropping) — Rust doesn't complain about an unused return value from a method call.
- **The dev loop:** write method → write its test → `cargo test` → watch it pass. With testing built in, there's no framework to wire up.

---

## 5. Reading a failing test (Part B)

Writing a deliberately failing test (`assert_eq!(fresh.health(), 999)`) shows how failures report:

- The runner names **which test** failed.
- It prints **expected vs actual** (`left` vs `right` for `assert_eq!`).
- Overall summary line: `test result: FAILED. N passed; M failed`.

Seeing a failure is as important as seeing a pass — that expected-vs-actual output is exactly what gets read when something breaks for real.

---

## Takeaways

- The combat system was pure consolidation: enum + trait + methods + iterators, no new concepts.
- `saturating_sub` subtracts without underflowing — unsigned `a - b` panics if it would go negative.
- **Testing is built in:** `#[test]` marks a test, `cargo test` runs them, assertion macros (`assert_eq!`, `assert!`, `assert_ne!`) do the checking.
- Tests live in a `#[cfg(test)] mod tests { use super::*; ... }` block in the same file — compiled only when testing.
- Good tests: one scenario per test, assert intermediate state, and test side effects as well as return values.
- Failing tests report the test name plus expected-vs-actual — learn to read that output.

---
