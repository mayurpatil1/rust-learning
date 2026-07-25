# Day 11 — Collections & `String` vs `&str`

> Learning Rust by building small projects. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Project:** a standalone tour of `Vec`, `HashMap`, `HashSet`, and the `String` / `&str` distinction.

**Goal today:** understand _why_ the collections used since Day 2 work the way they do — especially the owned-vs-borrowed string split.

---

## 1. `String` vs `&str` — the big one

**`String` is owned. `&str` is borrowed. That's the whole thing.**

- `String` owns heap memory, can grow, and cleans up after itself.
- `&str` ("string slice") is a **view into** string data someone else owns — can't grow, frees nothing.

```rust
let owned: String = String::from("hello"); // owns its data
let literal: &str = "hello";               // a view into the binary
let view: &str = &owned[0..2];             // "he" — a window into `owned`
```

### Two rules that resolve most confusion

**Rule 1 — take `&str` in parameters; return `String` when producing new data.**

- `fn shout(text: &str) -> String` — only _looks_ at the input (so `&str`), but _creates_ new text that needs an owner (so returns `String`).
- This is why `take_item(..., item: &str)` accepted both a literal and a `&String` — a `String` can always be borrowed as `&str`.

**Rule 2 — converting:**

- `&str` → `String`: `.to_string()` or `String::from(...)` — **allocates**.
- `String` → `&str`: `&my_string` or `.as_str()` — **free**, just a view.

**Why Rust splits them (Java has one `String`):** Java hides the allocation — every string is a heap object, cheap only because the GC manages it. Rust makes the distinction visible so a _view_ can be passed at zero cost, allocating only when ownership is genuinely needed.

---

## 2. `Vec<T>` — Java's `ArrayList`

Growable, ordered, indexed, duplicates allowed. Key methods:

- `.push(x)`, `.len()`, `.contains(&x)`, `.sort()` (in place).
- `.first()` / `.get(i)` return **`Option`** — safe. Indexing `v[99]` **panics**; `.get(99)` returns `None`. Prefer `.get()` (the Day 2 lesson again).
- `.remove(i)` deletes and **returns the owned value**.

---

## 3. `HashMap<K, V>` — Java's `HashMap`

Key-value pairs, unordered.

- `.insert(k, v)`, `.get(k)` → `Option<&V>`.
- Iterating yields `(key, value)` pairs.

### The method worth learning: `.entry().or_insert()`

```rust
*inventory.entry(key).or_insert(0) += 50;
```

"Get this key, or insert a default, then modify it." ≈ Java's `computeIfAbsent` / `getOrDefault`, but cleaner — one line replaces a get-check-insert dance.

- The `*` **dereferences**: `or_insert` returns a `&mut V`, and `*` reaches through it to modify the value.

---

## 4. `HashSet<T>` — unique values

No duplicates; inserting an existing value is silently ignored. Great for membership tests, deduplication, and set math.

- Deduplicate a Vec: `let unique: HashSet<i32> = rolls.iter().copied().collect();`
- Set operations with no clean Java equivalent: `.intersection()`, `.union()`, `.difference()`.

---

## 5. The word-frequency counter (Part A) — and the bug worth remembering

The canonical use of `entry()`:

```rust
let s = String::from("the quick brown fox the lazy dog the end");
let mut counts: HashMap<&str, u32> = HashMap::new();
for word in s.split_whitespace() {          // iterate words DIRECTLY
    *counts.entry(word).or_insert(0) += 1;
}
```

**The bug I hit:** collecting `split_whitespace()` into a `String` first (`let s1: String = ... .collect()`) **glues all words together** with no spaces, and then `.chars()` walks it letter by letter — producing a _letter_ counter, not a _word_ counter.

**The fix:** iterate `split_whitespace()` **directly** in the `for` loop — it already yields words one at a time; no intermediate collect, no `chars()`.

**The `&str` payoff:** keying on `&str` (not `String`) means each `word` is a **borrow** into `s`. Since `s` outlives the loop, no `String` is allocated per word — the Day 11 lesson doing real work.

---

## 6. Strings are UTF-8 bytes (Part B)

Slicing a `String` by index works on **bytes**, not characters, and must land on a **char boundary**.

- `&"adventurer"[0..3]` is fine — all ASCII, 1 byte per char.
- `&"héllo"[0..3]` **panics** — `é` is 2 bytes, so index 3 lands _inside_ a character.

Java hides this using UTF-16 internally; Rust exposes that text is UTF-8 bytes. Worth seeing once so it never surprises you. (To iterate actual characters safely, use `.chars()`.)

---

## Takeaways

- **`String` = owned, `&str` = borrowed** — the whole distinction is ownership.
- Take `&str` in parameters; return `String` when creating new data. `&str`→`String` allocates; `String`→`&str` is free.
- `Vec` ≈ `ArrayList`; prefer `.get()` (returns `Option`) over indexing (panics).
- `HashMap` ≈ `HashMap`; **`.entry().or_insert()`** is the workhorse — `*` dereferences the `&mut` it returns.
- `HashSet` = unique values; ideal for dedup and set operations.
- Word-count = iterate `split_whitespace()` **directly** and key on `&str` to avoid per-word allocation.
- Strings are **UTF-8 bytes**; slice indices must fall on char boundaries, or it panics.
