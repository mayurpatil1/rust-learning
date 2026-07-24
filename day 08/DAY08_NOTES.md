# Day 8 — Traits (Rust's Interfaces)

> Learning Rust by building small projects. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Project:** a standalone set of dungeon creatures (Goblin, Dragon, Chest, Skeleton) sharing behavior through a trait.

**Goal today:** define and implement custom traits — and retire the Java `extends` instinct for good.

**Week 2 opens here.** Yesterday's `Display` impl was the on-ramp; today the same shape gets used for traits _I_ define.

---

## 1. What a trait is

A trait is Rust's version of a Java **interface**: a named set of methods a type promises to provide.

- `trait Describable { fn describe(&self) -> String; }` ≈ `interface Describable { String describe(); }`
- A method signature with **no body** is a _required_ method — a promise each implementor must keep.
- Types fulfill it with the same `impl Trait for Type` shape used for `Display` on Day 7.

Each type implements the contract **its own way** — the trait says _what_, the impl says _how_.

---

## 2. Default methods

A trait method **with** a body is a default. Implementors get it for free unless they override it — like a Java `default` method on an interface.

- A default method can call the trait's **required** methods, since every implementor is guaranteed to have them. (`greet()` calls `describe()`.)
- Overriding is optional and per-type: in this project, `Dragon` overrode `greet` for something grander, while `Goblin`, `Chest`, and `Skeleton` used the default.

This is how a trait provides shared behavior **without** a base class.

---

## 3. The big shift: no inheritance

**Rust has no inheritance. There is no `extends`.**

- **Java instinct:** `abstract class Creature`, then `class Goblin extends Creature` — a hierarchy passing down state and behavior.
- **Rust model:** a type _implements_ the behaviors it needs. Capabilities are **composed**, not inherited. A type can implement many traits, but no parent class hands anything down.

The proof in this project: `Goblin`, `Dragon`, `Chest`, and `Skeleton` share **no parent, no common fields, nothing** — yet all four are `Describable`. A chest and a dragon have nothing in common except that both can describe themselves, and **that is enough**. Java would likely have forced them under some artificial `abstract class Entity`.

> **Stop asking "what is this a subclass of?" Start asking "what can this type do?"**

---

## 4. Trait bounds — writing code against the contract

Once types share a trait, functions can accept _anything_ that implements it:

```rust
fn announce(thing: &impl Describable) { ... }
```

- `&impl Describable` = "a reference to any type implementing `Describable`."
- ≈ Java's `void announce(Describable thing)` — programming to the interface.
- The function doesn't know or care which concrete type it got; it only knows the contract.

**The payoff:** a function like `is_threat(thing: &impl Describable) -> bool` mentions **no type names at all**. It works on every current implementor _and_ on types written next month. Behavior is defined once against a contract — not duplicated per class, not inherited down a hierarchy.

**Felt directly:** adding a brand-new `Skeleton` type required **zero changes** to `announce`. It just dropped in.

---

## 5. The error: `E0046` — the contract enforced

Adding one new required method (`danger_level`) to the trait broke **every** implementor at once:

```
error[E0046]: not all trait items implemented, missing: `danger_level`
```

— reported once per implementing type, with the error pointing at **both** the trait's requirement line **and** each incomplete `impl` block. Promise and broken promise, paired.

**What this guarantees:** a type cannot claim to implement a trait while missing a required method. One line changed in the contract produces a complete, exact to-do list of everything that must be updated — no runtime `AbstractMethodError`, no silently unimplemented behavior.

That property is what makes traits **safe to evolve** in a large codebase.

---

## Takeaways

- A **trait** is a contract of methods ≈ a Java interface; `impl Trait for Type` fulfills it.
- Method signature with **no body** = required; **with** a body = default (free unless overridden).
- Default methods may call required methods — shared behavior with **no base class**.
- **Rust has no inheritance.** Types compose capabilities instead of descending from a parent. Unrelated types can share a trait.
- `&impl Trait` as a parameter = accept any implementor — programming to the interface.
- Code written against a trait works for types that **don't exist yet** (new `Skeleton` needed no changes to `announce`).
- `E0046` enforces the contract completely: change the trait, and the compiler lists every implementor that must be updated.
