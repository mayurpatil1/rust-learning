# Day 7 — The `Display` Trait (Consolidation + Week 1 Close)

> Learning Rust by building small projects. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Project:** a standalone temperature log analyzer (a `Vec<Reading>` run through stats).

**Goal today:** cement methods and iterators, and implement the first real trait — `Display` — as a gentle on-ramp to Week 2.

---

## 1. `Display` — Rust's `toString()`

Two ways a type can print:

| Trait     | Format string | How you get it       | Purpose                      |
| --------- | ------------- | -------------------- | ---------------------------- |
| `Debug`   | `{:?}`        | `#[derive(Debug)]`   | quick, developer-facing view |
| `Display` | `{}`          | **you implement it** | clean, user-facing view      |

**You cannot `derive` `Display`** — Rust can't guess how _you_ want a type to read to a human. You write it yourself. This is the direct equivalent of overriding Java's `toString()`: same question ("how does this present itself?"), done as a trait implementation instead of a method override.

**The shape:**

```rust
impl fmt::Display for Reading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:.1}°C", self.day, self.celsius)
    }
}
```

- Read `impl Trait for Type` as: "here is an implementation _of_ the `Display` trait _for_ my `Reading` type."
- `write!` is `println!`'s cousin — same formatting syntax, but it writes into the formatter `f` instead of the screen.
- Once this exists, `println!("{}", reading)` uses _your_ format automatically, everywhere.

**This `impl Trait for Type` shape is the whole idea of Week 2's traits** — teaching a type a behavior that other code (like `println!`) already knows how to use.

---

## 2. Debug vs Display — the mental split

- `Debug` / `{:?}` → **derived**, automatic, for you the developer. ("Show me the fields.")
- `Display` / `{}` → **implemented**, deliberate, for the user. ("Show me the nice version.")

Same split as Java's auto-generated vs. hand-written `toString()`.

---

## 3. Proving the pattern: `Display` on a second type

Implementing `Display` a **second** time, on an unrelated struct (e.g. `City`), makes it stick: the `impl fmt::Display for ___` shape is **identical** regardless of the type. A trait implementation is a repeatable pattern, not a special case — which is exactly why traits scale across many types.

---

## 4. Reinforced iterator/method skills

Nothing new, just Day 5–6 skills doing real work:

- **Associated function** `Reading::new(...)` as a factory; **`&self` methods** `fahrenheit()` and `is_freezing()` for read-only calculations.
- A `bool`-returning method (`is_freezing`) reads like plain English inside a filter: `.filter(|r| r.is_freezing())` — nicer than an inline condition when the check is meaningful or reused.
- **Average:** `.map(|r| r.celsius).sum()` then divide by `len()`.
- **Filter + collect:** names of days above a threshold.
- **Filter + count:** how many days meet a condition.
- Inside a filter closure, `r` is a reference and `r.is_freezing()` "just works" — Rust supplies the `&self` automatically; no manual `&`.

### One float quirk: `max_by`

Finding the maximum of `f64` values needs a comparison closure:

```rust
readings.iter().max_by(|a, b| a.celsius.partial_cmp(&b.celsius).unwrap())
```

Because `f64` can be `NaN`, floats have no _total_ ordering, so Rust won't let you take a naive `.max()` — you compare with `partial_cmp`. Integers wouldn't need this. Just know: max/min over floats takes an extra step.

---

## Takeaways

- `Display` (`{}`) is Rust's `toString()` — **you implement it**; it can't be derived because only you know the human-facing format.
- `Debug` (`{:?}`) is the derived, developer-facing counterpart. Derive `Debug`, implement `Display`.
- `impl Trait for Type` — "an implementation of this trait for this type" — is the core shape of all traits (Week 2 preview).
- `write!` writes into a formatter; same syntax as `println!`.
- The trait impl pattern is **identical across types** — proven by doing `Display` twice.
- `bool`-returning `&self` methods make iterator filters read like English.
- Max/min over floats (`f64`) needs `max_by` + `partial_cmp` because floats lack total ordering.

---

## Week 1 complete

Seven days in, the toolkit so far: variables & mutability, structs, enums, pattern matching, ownership & borrowing, references (`&` / `&mut`), modules & visibility, error handling (`Option` / `Result`), serialization (`serde`), iterators & closures, methods (`impl`, `self`), `if let`, and the first trait (`Display`). Plus a repo of seven committed daily projects and a playable dungeon game.
