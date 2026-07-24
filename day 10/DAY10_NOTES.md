# Day 10 — Generics

> Learning Rust by building small projects. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Project:** a standalone demo — a generic `largest` function and a generic `Container<T>` struct.

**Goal today:** write code once that works for many types, and see the two places Rust diverges from Java generics.

---

## 1. The basics — same as Java

`<T>` declares a type parameter; `T` is a placeholder filled in at each call site.

- `fn largest<T>(list: &[T]) -> &T` ≈ `<T> T largest(List<T> list)`

Generics have been in use since Day 2 without being written: `Vec<String>`, `Option<Player>`, `HashMap<String, Room>` are all generic types.

---

## 2. Difference 1: trait bounds are **mandatory**

A bare `<T>` won't allow comparison — the compiler refuses, because **`T` could be any type, and not every type supports `>`**.

The fix is a **trait bound**: `fn largest<T: PartialOrd>(list: &[T]) -> &T`

Read `T: PartialOrd` as _"T must implement the `PartialOrd` trait"_ — the trait providing `>`, `<`, etc.

**The key idea:** **traits are the vocabulary of generics.** A bound says "I don't care what type this is, as long as it can do X" — and X is always a trait. This is what Days 8–9 were building toward.

Java has this (`<T extends Comparable<T>>`) but treats it as optional. Rust makes it **required**: a generic function can never assume a capability it didn't declare.

### Bound syntax

| Form                           | Meaning                                           |
| ------------------------------ | ------------------------------------------------- |
| `<T: PartialOrd>`              | one bound                                         |
| `<T: PartialOrd + Display>`    | **multiple** bounds — `+` means "and"             |
| `where T: Display, U: Display` | same meaning, easier to read when bounds get long |

---

## 3. Difference 2: no type erasure — **monomorphization**

- **Java:** erases generics at runtime. `List<String>` and `List<Integer>` are both just `List` once compiled.
- **Rust:** at compile time, generates a **separate concrete copy** of the generic code for every type actually used.

Calling `largest` with `i32`, `char`, and `&str` puts **three specialized functions** in the binary.

**Consequence: generics cost nothing at runtime.** No boxing, no casting, no reflection limits — as fast as hand-writing each version. The tradeoff is slightly larger binaries and longer compile times.

---

## 4. Generic structs

`struct Container<T>` ≈ Java's `class Box<T>` — one definition, any type.

**The impl block needs its own `<T>`:** `impl<T> Container<T>` reads as _"for any T, `Container<T>` has these methods."_

---

## 5. The best part: **tiered / conditional methods**

Methods can be granted based on what the contained type can do. Three impl blocks layered on one struct:

| Impl block                      | Who gets the methods                            |
| ------------------------------- | ----------------------------------------------- |
| `impl<T> Container<T>`          | **all** containers → `add`, `count`, `first`    |
| `impl<T: Display> Container<T>` | only containers whose contents print → `show()` |
| `impl Container<i32>`           | only containers of `i32` → `total()`            |

The third form is a **concrete, non-generic impl block**: no `<T>` after `impl`, and the type spelled out fully. Inside it the compiler _knows_ the items are `i32`, so `.iter().sum()` works with no bound needed — the type is completely pinned down.

**Proof it's real:** `scores.total()` compiles; `tools.total()` (a `Container<String>`) fails with _"no method named `total` found for struct `Container<String>`"_. The method genuinely **isn't there** for that type — not a runtime check, not a cast that blows up. It doesn't exist, and the compiler knows.

**Why Java can't do this cleanly:** a Java generic class has **one** set of methods, period. Restricting to numbers means bounding the whole class (`class Container<T extends Number>`), which then forbids string containers entirely. Rust layers capabilities instead — all containers get the basics, printable ones get more, integer ones get more still.

---

## 6. The error worth seeing (Part A)

Removing `: PartialOrd` from `largest` produces an error about the binary operation `>` not applying to `T` — **and the compiler suggests adding the bound**.

**The lesson:** you cannot use a capability you didn't declare. The bound isn't ceremony; it's the promise that makes the operation legal.

---

## Takeaways

- `<T>` declares a type parameter — same idea as Java generics.
- **Trait bounds are mandatory**: `<T: SomeTrait>` promises what `T` can do. Traits are the vocabulary of generics.
- `+` combines bounds; `where` clauses keep long bound lists readable.
- **No type erasure.** Rust monomorphizes — a concrete copy per type used — so generics are **zero-cost** at runtime.
- Generic structs need `<T>` on both the struct **and** the impl block.
- **Methods can be granted conditionally** by writing multiple impl blocks: for all `T`, for bounded `T`, or for one concrete type. Java has no clean equivalent.
- A method that doesn't apply to a type simply **does not exist** on it — enforced at compile time.
