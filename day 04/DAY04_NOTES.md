# Day 4 — Modules, Visibility & a Design Lesson

> Learning Rust by building a text adventure. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Goal today:** split a ~180-line `main.rs` into focused modules — and fix the duplicate-item save bug left over from Day 3.

**The twist:** the second half turned out not to be about modules at all. The compiler found a design flaw.

---

## 1. `mod` — declaring a module

Rust and Java are **opposites** here.

- **Java:** the file declares its own package (`package com.foo;`) and the compiler discovers files by walking directories.
- **Rust:** nothing exists until you declare it. Create `player.rs` and never mention it, and the compiler won't even look at the file.

A single line in `main.rs` — `mod player;` — means "there is a module called `player`; find it in `player.rs`." The **file name is the module name**. No directory scanning, no magic.

---

## 2. `pub` — private by default, per item

The big difference from Java.

- **Java:** no modifier = package-private, and you learn to type `public` reflexively.
- **Rust:** **everything is private unless marked `pub`**, and visibility is decided **per item** — including individual struct fields.

`pub` on the struct and `pub` on each field are **separate decisions**. Moving working code into modules produces a burst of visibility errors — not because anything is wrong, but because Rust just made every implicit assumption explicit.

Things that typically need `pub` when extracted:

- the module's types (`pub struct Room`)
- each field accessed from outside (`pub name: String`)
- every function called from another module (`pub fn build_world`)

**Reframe:** `pub` isn't boilerplate — it's declaring the module's **public API**. Everything without it is an implementation detail that's free to change. Same instinct as keeping fields private in a Spring service; Rust just makes it the default rather than the discipline.

---

## 3. `use` and `crate::` paths

- `crate` means "the root of this project."
- `use crate::player::Player;` ≈ `import com.game.player.Player;`
- It's a convenience, not a requirement — it just saves writing the full path everywhere.

**`mod` vs `use` are different jobs** (easy to conflate at first):

- `mod` = "this file is part of my project."
- `use` = "let me refer to this thing by its short name."

Each file needs **its own** `use` statements. Imports are not inherited from `main.rs` — every module is its own island. Functions in the **same** module call each other with no prefix, like methods in the same class.

---

## The structure

| File          | Responsibility                          |
| ------------- | --------------------------------------- |
| `main.rs`     | module declarations, imports, game loop |
| `player.rs`   | the `Player` type                       |
| `world.rs`    | the `Room` type + `build_world`         |
| `commands.rs` | look, move, take, drop, inventory       |
| `save.rs`     | save/load + `GameState`                 |

`main.rs` ended with **zero structs and zero helper functions** — it declares modules, imports what it needs, and runs the loop. Same program, five focused files.

---

## The visibility errors (two distinct ones)

### `E0603: function 'build_world' is private`

The item exists and the module was found, but the function has no `pub`. The door is locked.

### `E0451: field 'health' of struct 'Player' is private`

Subtly different: this fires specifically when **constructing a struct that has a private field**. The type is public and visible — but you can't fill in a field you're not allowed to touch. Proof that struct-level and field-level `pub` are independent.

---

## The real lesson: `E0382` and a design flaw

Fixing the two-torch bug meant saving the rooms alongside the player, so a `GameState { player, rooms }` wrapper was built at save time. That failed:

```
error[E0382]: borrow of moved value: `hero`
   value moved here, in previous iteration of loop
   this reinitialization might get skipped
```

**Why:** `GameState` **owns** its fields, so `GameState { player: hero, ... }` _moves_ `hero` into it. Iteration one moves the player away; iteration two needs a player that no longer exists. The compiler even analyzed the `load` arm — which _does_ assign a new player — and concluded it couldn't rely on it, since the command might never be typed. That's flow analysis across loop iterations.

The mismatch in one line: **trying to _lend_ the player and rooms for the duration of a save, using a type that _takes_ them.**

### Three options

1. **`.clone()` everything** — the compiler suggests it, and it works. But it deep-copies the entire world (every room, string, and item) on every save, then throws the copy away. Fine for three rooms, absurd for a real game. The "make the error go away" answer.
2. **Hold references** (`struct GameState<'a> { player: &'a Player }`) — the right instinct ("lend, don't give"), but it drags in lifetimes and breaks the load side, since deserializing must return **owned** data. Right idea, wrong day.
3. **Make `GameState` the actual game state** — not a save-time wrapper. ✅

### Why option 3 is the answer

`hero` and `rooms` were **always one concept** — "the state of the game" — artificially split into two variables. The flaw only surfaced when saving forced them together.

Owning them in one `state` variable from the start means:

- nothing ever moves
- saving is just `save_game(&state)` — an immutable borrow
- loading is one assignment: `state = loaded`
- the code got **shorter**, with fewer variables and cleaner signatures

**The takeaway:** Java would have let this flaw live forever — pass both to every method, never think twice. Rust made it a compile error the moment the seams showed.

> Fighting the borrow checker often means the ownership model doesn't match the problem. The answer usually isn't `.clone()` — it's rethinking who should own what.

---

## Bonus rule: borrows are tracked **per field**

This compiles:

```rust
take_item(&mut state.player, &mut state.rooms, &item);
```

Two mutable borrows in one call — doesn't that break the Day 2 rule? No. The rule is **per value**, and the borrow checker tracks struct fields **individually**. `state.player` and `state.rooms` are disjoint memory, so mutably borrowing both at once can't conflict. Rust is **precise** here rather than conservative. Same for mixing one `&mut` field with one `&` field.

---

## Takeaways

- **`mod` declares; nothing exists until it does.** File name = module name.
- **Private by default, per item.** `pub` on the type and on each field are separate choices.
- `pub` = declaring a **public API**, not boilerplate.
- `mod` ≠ `use`: one loads the file, the other shortens the path. Every module needs its own imports.
- `E0603` = private item; `E0451` = private field blocking struct construction.
- `E0382` across a loop = a value moved on one iteration and missing on the next; the compiler reasons about **flow**, including branches that might not run.
- **Owning vs lending is a type-level decision** — a struct with owned fields _takes_; it cannot _borrow_ on your behalf.
- **The compiler catching a "bug" sometimes means it caught a design flaw.** The fix made the code simpler, not more contorted.
- Borrows are tracked **per struct field**, so disjoint fields can be borrowed mutably at the same time.
