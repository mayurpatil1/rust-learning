# Day 6 — Methods, `self`, `&`, and `if let` (Consolidation)

> Learning Rust by building small projects. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Project:** a standalone in-memory task manager (add / start / complete / remove tasks).

**Goal today:** pull Week 1 together, and add two things the dungeon skipped — **methods** (`impl` blocks) and **`if let`** — while finally understanding `self` and `&` deeply.

---

## 1. `impl` blocks — methods on your types

Until now every function was free-standing (`move_player(player, ...)`). Rust lets you attach functions **to** a type inside an `impl` block, exactly like methods in a Java class.

- `task.describe()` reads like the Java you know — the behavior lives _on_ the type.
- Same logic as a free function, just called with dot syntax on an instance.

---

## 2. `self` — the instance, with a declared access level

`self` is the instance the method was called on (≈ Java's `this`). The twist: **you declare how the method accesses that instance**, and there are four forms. This choice IS the ownership rules from Day 2, applied to the instance.

| Signature                 | Meaning                                        | Java analogy               |
| ------------------------- | ---------------------------------------------- | -------------------------- |
| `fn describe(&self)`      | **borrow to read** — caller keeps ownership    | a getter / read method     |
| `fn mark_done(&mut self)` | **borrow to modify** — changes the instance    | a setter / mutating method |
| `fn consume(self)`        | **take ownership** — caller loses the instance | (no clean equivalent)      |
| `fn new(...)` (no self)   | **associated function** — called on the _type_ | `static` method / factory  |

**The decision rule when writing a method:** does it need to _read_ → `&self`; _change_ → `&mut self`; _consume_ → `self`. That's why `describe` and `mark_done` have different signatures.

**`.` vs `::`** — the visual tell:

- `task.describe()` — dot = called **on an instance**.
- `Task::new(...)` — double-colon = called **on the type** (associated function).

`&self` calls can be made freely (reading never conflicts); a `&mut self` method, while it runs, locks out all other access to that instance.

---

## 3. `&` — one symbol, three jobs

`&` always means the same thing: **"a reference to, not the value itself."** A reference points at a value without owning it — the owner still does. It shows up in three places, but it's one idea.

**Job 1 — making a reference (at the call site).** `&x` lends `x` instead of giving it away:

- `look(&hero, &rooms)` → lends both; the caller still owns them afterward.
- Without `&`, the value would **move** and the caller would lose it (the Day 4 `GameState` error).

**Job 2 — accepting a reference (in a type/signature).** `&` in a parameter type promises "I borrow this, I don't consume it":

- `fn look(player: &Player)` → borrows a Player.
- That promise is _why_ `hero` is still usable after `look(&hero, ...)`.

Jobs 1 and 2 are the same idea from two sides — the call site creates the loan, the signature accepts it. Plug and socket.

**Job 3 — `&mut`, the mutable version of both.** Add `mut` after `&` for a loan that can also change the value:

- Call site: `move_player(&mut hero, ...)`
- Signature: `fn move_player(player: &mut Player)`

**The one rule behind all of it (unchanged since Day 2):** many `&` readers at once, **or** one `&mut` writer at a time, **never both.**

---

## 4. `self` and `&` are the _same_ concept

`&self` is literally `&` (a reference) applied to `self` (the instance). Understanding `&Player` borrows a player _is_ understanding `&self` borrows the instance — same `&`, just pointing at the instance parameter. Nothing new.

**Reading any signature becomes a superpower:**

```
fn take_item(player: &mut Player, rooms: &mut HashMap<String, Room>, item: &str)
```

Left to right: borrows the player **mutably** (will change it), borrows the rooms **mutably** (will change them), borrows a string **immutably** (only reads it). You know exactly what a function will and won't do to its inputs **from the signature alone**, before reading the body. Java would make you read the whole method to know if it mutates its arguments.

---

## 5. `if let` — a `match` with only the arm you care about

When an `Option` matters only in the `Some` case, a full `match` is overkill.

- `if let Some(x) = thing { ... }` runs the block only if it matches, and **binds** `x` for use inside.
- Add `else { ... }` for the `None` case.
- ≈ Java's `Optional.ifPresent(...)`, but it also binds the inner value.

Used today in `complete`, `start`, and `remove` — each acts on a task only if it was found, else prints a "no such id" message.

---

## 6. Consolidation: the Week 1 skills, combined

The project reused everything without new syntax:

- **Enum with a method** — `Status` (Todo / InProgress / Done) with a `label()` method that `match`es on `self`. `#[derive(PartialEq)]` enables `==` / `!=` comparisons.
- **Associated function as constructor** — `Task::new(...)`, using field shorthand (`id` instead of `id: id`).
- **`iter_mut()` + `find()`** — `find_mut` returns `Option<&mut Task>`: locate a task _and_ be allowed to change it.
- **`iter()` + `filter()` + `collect()` / `count()`** — pending list and progress summary (Day 5 queries doing real work).
- **`position()` + `remove()`** — delete by finding the index first, then removing. Split into two steps because you can't _search_ and _mutate_ a Vec at the same instant (the borrow rule); finish the search (get a plain `usize`) then mutate. `.remove(index)` hands back the **owned** value.

---

## Takeaways

- Methods live in `impl` blocks and are called with `.` on an instance; associated functions use `::` on the type.
- **`self` is just the instance as a parameter** — `&self` (read), `&mut self` (modify), `self` (consume), none (static/factory). Same ownership rules as any other value.
- **`&` = "a reference, not the value"** — one concept doing three jobs: create a loan (call site), accept a loan (signature), and `&mut` for mutable loans.
- `&self` **is** `&` applied to the instance — not a separate rule.
- A signature's `&` / `&mut` marks tell you what a function does to its inputs **before** you read the body.
- `if let` is a one-arm `match` (with optional `else`) that binds the inner value — cleaner than `match` when only `Some` matters.
- `position()` + `remove()` deletes from a Vec by locating the index first, then mutating — respecting the borrow rule.
