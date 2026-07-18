# Day 5 — Iterators & Closures

> Learning Rust by building small projects. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Project:** a standalone guild roster analyzer (a `Vec<Adventurer>` run through a series of iterator queries).

**Goal today:** learn closures and iterators — essentially Java Streams and lambdas with different syntax, plus one genuinely new idea (ownership-aware iteration).

**The easy day:** almost every line has a direct Java Streams equivalent.

---

## 1. Closures — Java lambdas with pipes

A closure is an anonymous function. Pipes replace Java's arrows.

- `|x| x + 1` ≈ Java `x -> x + 1`
- `|a, b| a + b` ≈ Java `(a, b) -> a + b`

Closures **capture** variables from the surrounding scope, like a Java lambda closing over an effectively-final local. (Rust's capture rules are richer — by borrow _or_ by move — but that's a later concern.) A closure can be stored in a variable and reused across multiple iterator calls.

---

## 2. Iterators ≈ Streams

The pattern matches Java almost one-to-one:

| Java Stream            | Rust Iterator        |
| ---------------------- | -------------------- |
| `list.stream()`        | `vec.iter()`         |
| `.map(x -> ...)`       | `.map(\|x\| ...)`    |
| `.filter(x -> ...)`    | `.filter(\|x\| ...)` |
| `.collect(toList())`   | `.collect()`         |
| `.findFirst()`         | `.find(...)`         |
| `.anyMatch(...)`       | `.any(...)`          |
| `.allMatch(...)`       | `.all(...)`          |
| `.count()`             | `.count()`           |
| `.mapToInt(...).sum()` | `.map(...).sum()`    |

**Laziness (same as Streams):** `.map()` and `.filter()` are **adapters** — they build a recipe and do nothing on their own. Only a **consumer** (`.collect()`, `.find()`, `.count()`, `.sum()`, a `for` loop) actually runs the chain. An unconsumed chain triggers a dead-code warning.

**One quirk:** `.collect()` needs to know the target type, so it's annotated (`let names: Vec<String> = ...`). Java puts that same information in `Collectors.toList()`; Rust puts it in the type.

---

## 3. The genuinely new part: `iter()` vs `iter_mut()` vs `into_iter()`

Java Streams have one flavor (always references). Rust has **three**, chosen by **what you intend to do with the elements** — pure ownership.

| Method         | Element type | Effect on the collection                      |
| -------------- | ------------ | --------------------------------------------- |
| `.iter()`      | `&T`         | borrows it; collection survives, read-only    |
| `.iter_mut()`  | `&mut T`     | borrows mutably; modify elements **in place** |
| `.into_iter()` | `T` (owned)  | **consumes** it; collection is gone afterward |

- `.iter_mut()` is how you change every element in place (e.g. give every member a gold bonus). No Java Stream equivalent.
- `.into_iter()` **takes** owned values out — so no `.clone()` is needed, but the collection can't be used afterward. This is `.remove()`'s philosophy (Day 3) applied to a whole collection.

Day 2 and Day 5 shake hands here: iteration is just ownership again.

---

## 4. `.map()` on an `Option` is _not_ the iterator `.map()`

`roster.iter().find(|a| a.name == target).map(|a| a.level)`

- `.find(...)` returns `Option<&Adventurer>`.
- `.map(|a| a.level)` here is **`Option::map`** — Java's `Optional.map`. It transforms **what's inside** the Option: `Option<&Adventurer>` → `Option<u32>`. If it's `None`, it stays `None`.

Same word, same concept ("transform"), different type. Rust reuses `map` consistently across iterators, `Option`, and `Result` — learn it once, it works everywhere.

---

## 5. Macros and the `!` (the `vec!` question)

The `!` marks a **macro**, not a function call. A macro is **code that writes code at compile time** — the compiler _expands_ it into real instructions before building.

- `vec![a, b, c]` expands into roughly "make an empty Vec, push a, push b, push c."
- **Why a macro and not a function?** Normal Rust functions take a _fixed_ number of arguments of _known_ types. `vec![]`, `vec![1]`, and `vec![1,2,3,4]` all differ in count — only a macro can accept variable-length input, because it works on _code_ before counts and types are locked in.

Macros met so far, all "code that writes code":

- `println!` / `format!` — accept any number of format arguments.
- `vec!` — variable-length list construction.
- `#[derive(Debug)]` / `#[derive(Serialize)]` — a _derive_ macro that generates a whole trait implementation.

**Bracket convention:** brackets are interchangeable to the compiler, but the community uses `vec![...]` (square, list-like) and `println!(...)` (parens). Writing your own macros is advanced; for now the skill is just _reading_ the `!` as "this expands into more code."

---

## The error to recognize (Part B challenge)

Using `.into_iter()` too early consumes the collection, so any later `.iter()` on it fails:

```
error[E0382]: borrow of moved value: `roster`
```

Same `E0382` move error from Day 4's `GameState` fight — new costume. `.into_iter()` **took** the Vec; you can't use what you've given away. The recognition ("oh, it's _this_ again") is the point.

---

## Takeaways

- Closures are anonymous functions: `|x| ...` ≈ Java `x -> ...`; they capture surrounding variables.
- Iterators mirror Streams: `iter → map/filter → collect/find/any/sum`, and they're **lazy** — adapters build, consumers run.
- `.collect()` needs a **target type**, usually via annotation.
- **Three iteration flavors, chosen by ownership:** `.iter()` (borrow `&T`), `.iter_mut()` (borrow `&mut T`, modify in place), `.into_iter()` (consume, take owned `T`).
- `.map()` works on iterators **and** on `Option`/`Result` — same idea, different type.
- `enumerate()` gives `(index, value)` pairs; index starts at **0**.
- **`!` = macro = code that writes code** at compile time; macros allow variable-length arguments that functions can't.
- `.into_iter()` consuming a collection early causes `E0382` — the Day 4 move error, again.
