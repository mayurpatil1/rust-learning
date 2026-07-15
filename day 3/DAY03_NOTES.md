# Day 3 — Input, Parsing, Inventory & Save/Load

> Learning Rust by building a text adventure. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Goal today:** turn the scripted, self-playing tour into a real game — type commands, get responses — add an inventory, replace the crash-prone `.unwrap()`s from Day 2 with graceful handling, and persist the game to JSON.

**Milestone reached:** a complete, playable, saveable game. This closes the "3-day sprint."

---

## 1. The game loop and reading input

A `loop { }` prints a prompt, reads a line, and reacts — the same structure as the very first Day 1 program.

- Reading input is more verbose than Java's `Scanner` because Rust refuses to hide that I/O can fail.
- `read_line` writes the typed text into a **mutable** `String` and returns a `Result`.
- `.expect(...)` on that `Result` is acceptable for now ("give me the value or crash with this message").
- Input arrives as one raw string like `"go north\n"` — turning that into something useful is our job.

---

## 2. Parsing: turning text into words

Three methods do the work:

- `.trim()` strips the trailing newline and stray spaces.
- `.split_whitespace()` breaks the string into an iterator of words.
- `.collect()` gathers those words into a `Vec`.

**Ownership detail:** the result is `Vec<&str>`, not `Vec<String>`. The words are **borrowed slices** pointing into the original input string — no copying. Day 2 borrowing, quietly at work again.

---

## 3. Slice pattern matching — the star of today

Rust can `match` on the **shape and contents** of the word list at once — something Java cannot do cleanly.

- `["go", direction]` matches **only** when there are exactly two words **and** the first is `"go"`, and **captures** the second word in the same step.
- Other forms: one exact word (`["look"]`), alternatives (`["quit"] | ["exit"]`), the empty line (`[]`), and a catch-all (`_`).

**Java comparison:** this would be a chain of `if (parts.length == 2 && parts[0].equals("go"))` with manual indexing and length checks. Rust describes the pattern directly and binds the piece you want in one line.

### The bug this caused: patterns match length _exactly_

`take torch` (2 words) worked. `take rusty key` (3 words) fell through to the catch-all and printed "I don't understand." The pattern `["take", item]` matches **exactly two** elements — nothing fuzzy about it.

**Fix — the rest pattern (`..`):**

- `["take", item_words @ ..]` means: first word is `"take"`, then bind **all remaining words** to a name.
- `@` is the "bind this to a name" operator.
- `.join(" ")` glues the remaining words back into one owned `String`.
- **Order matters:** put the bare `["take"]` arm _first_, because `..` also matches zero words and would otherwise swallow it.

---

## 4. Graceful errors: retiring `.unwrap()`

Recall the Day 2 panic — `.unwrap()` on a missing map key crashed the program.

- `HashMap::get` returns an `Option` (Rust's `Optional`, but one you cannot ignore).
- `.unwrap()` means "give me the value **or crash**."
- The grown-up version is to `match` on the `Option` and handle **both** `Some` and `None`.

A missing room now prints a message instead of killing the program. That's the difference between a toy and something robust.

---

## 5. Structure: a function that returns ownership

World-building moved into `build_world()`, which **returns** the whole `HashMap`.

- The return type says the function hands back ownership of what it built; the caller becomes the new owner.
- The friendly face of ownership — a function _producing_ an owned value, versus the borrow-checker fights of Day 2.

---

## 6. Inventory: ownership transfer in action

The player got a `Vec<String>` inventory; rooms got a `Vec<String>` of items.

- `.get_mut()` is the **mutable cousin** of `.get()` — it hands back a `&mut` so the room's item list can be changed. Plain `.get()` only permits reading.
- `.iter().position(|i| i == item)` uses a **closure** (`|i| ...` is a mini-function, like a Java lambda) to find an item's index. A preview of Day 5's iterators; feels like Java Streams.
- `.remove(index)` doesn't just delete — it **returns the owned `String`**, which then gets `push`ed into the inventory. Ownership moves cleanly from room to player, **no `.clone()` needed**, because the value was genuinely moved out.
- Functions that mutate rooms need `&mut HashMap`, which in turn forces the binding to be `let mut rooms`. The borrow rule shapes the design.

---

## 7. Persistence with `serde` (≈ Jackson)

Two crates, added via `cargo add` (like adding a `<dependency>` to `pom.xml`, but one command):

- **`serde`** — the _framework_; defines what "serializable" means, knows nothing about JSON. (≈ Jackson core)
- **`serde_json`** — the _JSON codec_ built on top. (≈ jackson-databind)
- The `derive` feature unlocks macro support; without it you'd hand-write conversion code.

**Marking types serializable:** add `Serialize, Deserialize` to the `derive` line. No per-field annotations, no `ObjectMapper` config.

**What `derive` actually is:** a macro that **writes code at compile time**. The compiler generates the conversion logic for your specific struct and pastes it in before building.

- Jackson inspects objects at **runtime** via reflection.
- serde has already written the exact conversion code **ahead of time** — no reflection cost, and non-serializable fields fail at _compile time_, not in production.
- Same job as Jackson, moved earlier in the pipeline.

**Error handling here compounds everything from today:** serializing can fail, writing can fail, reading can fail, parsing can fail — four failure points, each returning a `Result`, each `match`ed explicitly, none able to crash the game. Compare to a Java `try/catch` wrapping the whole block, which might swallow all four indistinguishably.

**Signature details worth noting:**

- `save_game` takes `&Player` — an immutable borrow. It only _reads_ the player to write it out.
- `load_game` returns `Option<Player>` — "maybe I got you a player, maybe not; **you** decide." Can't be accidentally ignored.
- Type inference works **backwards from the signature**: `from_str` was never told what type to build — it deduced `Player` from the declared return type.
- Reassigning the loaded player (`hero = loaded`) only compiles because `hero` was declared `let mut` — a callback to Day 1's mutability rule.

---

## The error I hit: `E0716` — temporary dropped while borrowed

Lowercasing the input inline (to make commands case-insensitive) triggered:

```
error[E0716]: temporary value dropped while borrowed
```

**Why:** `.to_lowercase()` creates a **brand-new owned `String`**, but as an inline temporary it has no name and is freed at the end of that statement. The `Vec<&str>` borrows slices _from_ that temporary — so it would be left pointing at freed memory. A dangling reference: exactly what Rust exists to prevent.

**Fix:** bind the lowercased `String` to a variable so it lives long enough, _then_ split it. The code does the same thing — the owned value just gets a home.

**Lesson:** pure Day 2 ownership in new clothes. **A borrow can never outlive what it borrows from.** When `E0716` appears, look for a temporary that needs to become a named `let`.

---

## Other mistakes worth remembering

Three compile errors, all the same underlying lesson: **Rust never leaves a field or a mutability question implicit.**

1. **`cannot borrow as mutable`** — `rooms` was declared without `mut`, but take/drop needed `&mut` access.
2. **`missing field 'items'`** — added a field to the `Room` struct but didn't set it in the struct literals.
3. **`missing field 'inventory'`** — same thing on `Player`.

Rust structs have **no defaults**. If a field is declared, every literal must fill it. No silent `null` like Java might tolerate.

---

## A design gap (not a Rust error)

After loading a save, the picked-up torch appeared **both** in the inventory and back on the entrance floor — two torches.

**Why:** only the _player_ is saved. The world is rebuilt fresh from `build_world()` on every launch, so rooms never learn their items are gone. The saved state doesn't cover everything that changed.

**Proper fix:** save the rooms too, via a small wrapper struct holding both — tidier once modules arrive on Day 4.

**The distinction that matters:** the compiler catches _memory_ bugs, not _logic_ bugs. That's still on me.

---

## Takeaways

- Input arrives as one raw `String`; **parse** it (`trim` → `split_whitespace` → `collect`) into words.
- Split words are `Vec<&str>` — **borrowed slices**, not owned copies.
- **Slice patterns** match shape _and_ contents in one step, but match length **exactly** — use the rest pattern `..` with `@` for variable-length input, and order arms carefully.
- Replace `.unwrap()` with a `match` on `Option` to handle missing values **without crashing**.
- `.get_mut()` gives mutable access; `.remove()` **returns** the owned value, enabling clean ownership transfer with no clone.
- Functions can **return owned values**, transferring ownership to the caller.
- **`derive` macros write code at compile time** — serde does ahead of time what Jackson does at runtime via reflection.
- Every fallible operation returns a `Result` and gets handled explicitly — no silent failures.
- `E0716`: **a borrow can't outlive what it borrows from**; bind the temporary to a variable.
- Rust structs have **no default fields** — declare it, fill it.
- The compiler catches memory bugs, **not logic bugs** (see: the duplicate torch).
