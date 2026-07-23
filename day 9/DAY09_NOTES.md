# Day 9 — Derive Macros & Common Traits

> Learning Rust by building small projects. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Project:** a standalone demo of the five most common derivable traits.

**Goal today:** finally explain `#[derive(...)]` — the annotation sitting in my code since Day 1 — now that traits make sense.

---

## 1. What `derive` actually does

Yesterday's trait impls were written by hand. Some traits are so **mechanical** that the compiler can write them instead — compare two structs field by field, copy every field, print every field.

`#[derive(Debug)]` means: _"compiler, write the `Debug` implementation for this type."_ It expands into a real `impl Debug for Item { ... }` block **at compile time**, before the program builds — the same shape as the hand-written `Display` impl from Day 7.

This connects three earlier ideas:

- **Traits** (Day 8) — what's being implemented.
- **Macros / the `!` and `#[...]`** (Day 5) — code that writes code at compile time.
- **`Debug`** (Day 1) — the first one used, without knowing why.

**The key limit:** only traits that are _mechanically obvious_ can be derived. `Debug` can be ("print every field"). `Display` **cannot** — only the author knows how a type should read to a human. That's exactly why Day 7 required writing it by hand.

---

## 2. The five common derivable traits

| Trait       | Enables                                           | Java analogy                  |
| ----------- | ------------------------------------------------- | ----------------------------- |
| `Debug`     | `{:?}` formatting                                 | auto-generated `toString()`   |
| `Clone`     | `.clone()` — an explicit copy                     | `clone()` / copy constructor  |
| `Copy`      | assignment **duplicates** instead of moving       | (no equivalent — GC hides it) |
| `PartialEq` | `==` and `!=`, field by field                     | `equals()`                    |
| `Default`   | `Type::default()` — every field at its zero value | a no-arg constructor          |

**`Debug`** — developer-facing view, used since Day 1.

**`Clone`** — my own types don't get `.clone()` until it's derived. This is what the Day 4 compiler hint ("consider implementing `Clone` for this type") was asking for. `String` already implements it, which is why `next_room_key.clone()` worked back on Day 2.

**`Default`** — fills fields with zero values: `0`, `""`, `false`, empty `Vec`.

---

## 3. `Copy` — the Day 2 question, finally named

Day 2's puzzle was: _why does `hero.gold` (a `u32`) survive being used repeatedly, while a `String` disappears?_

**The answer:** `Copy` is a **trait**. Types that implement it are duplicated on assignment instead of moved. Numbers, `bool`, and `char` implement it; `String`, `Vec`, and `HashMap` do not.

It's not magic reserved for primitives — **any** type can derive `Copy`, _provided every field is itself `Copy`_.

- `Position { x: u32, y: u32 }` → can be `Copy`. `let end = start;` leaves `start` usable, no `.clone()` needed.
- `Item { name: String, value: u32 }` → **can never** be `Copy`, because `String` owns heap memory that can't be silently duplicated.

**Attempting it produces the error I hit in Part A** — the trait can't be implemented for a type containing non-`Copy` fields. That's Day 2's ownership rule restated in trait terms: moving vs. copying was never arbitrary, it's just whether a type implements `Copy`.

**Rule:** `Copy` always requires `Clone` alongside it.

---

## 4. `Clone` is opt-in (Part B)

Removing `Clone` from the derive list makes `.clone()` vanish with a "method not found" error.

**The lesson:** `.clone()` isn't built into every type — it's a **capability opted into** by deriving the trait, exactly like any trait method from Day 8. Nothing about it is special or automatic.

---

## 5. Struct update syntax: `..Default::default()`

```rust
let custom = Settings {
    difficulty: 3,
    ..Default::default()
};
```

Means: _"set these fields explicitly, fill everything else with defaults."_

Rust has **no named or optional parameters**, so this pattern fills that gap — especially useful for config-style structs where only one or two fields matter.

---

## 6. Derived vs hand-written, side by side

The same `Item` type had **`Debug` derived** and **`Display` hand-written**. Two printing traits on one struct: one the compiler wrote, one I wrote. That contrast is the whole lesson about what can and can't be derived.

---

## Takeaways

- **`derive` auto-generates a trait implementation at compile time** — it's a macro producing the same `impl Trait for Type` block you'd write by hand.
- Only **mechanically obvious** traits are derivable; `Display` isn't, because the human-facing format is a design decision.
- `Debug` → `{:?}`; `Clone` → `.clone()`; `PartialEq` → `==`; `Default` → `Type::default()`.
- **`Copy` is the trait behind Day 2's move-vs-copy mystery.** Any type can derive it _if every field is `Copy`_ — so a struct holding a `String` never can.
- `Copy` requires `Clone`.
- `.clone()` is **opt-in**, not universal — a trait capability like any other.
- `..Default::default()` fills unspecified fields — Rust's stand-in for optional/named parameters.
