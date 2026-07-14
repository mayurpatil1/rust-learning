# Day 3 — Input & Parsing

> Learning Rust by building a text adventure. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Goal today:** turn the scripted, self-playing tour into a real game — type commands, get responses — and replace the crash-prone `.unwrap()`s from Day 2 with graceful handling.

**The big idea:** Input arrives as one raw `String`. The job is to _parse_ it into meaningful pieces and then _match on the shape of those pieces_ to decide what to do. Two features carry the day: parsing text into words, and slice pattern matching.

---

## 1. The game loop and reading input

A `loop { }` prints a prompt, reads a line, and reacts — same structure as the very first Day 1 program.

- Reading input is more verbose than Java's `Scanner` because Rust refuses to hide that I/O can fail.
- `read_line` writes the typed text into a **mutable** `String` and returns a `Result`.
- For now, `.expect(...)` on that `Result` is acceptable ("give me the value or crash with this message"). It gets refined later.
- Key point: input arrives as one big string like `"go north\n"` — turning that into something useful is our job.

---

## 2. Parsing: turning text into words

Two string methods do the work:

- `.trim()` strips the trailing newline and stray spaces.
- `.split_whitespace()` breaks the string into an iterator of words.
- `.collect()` gathers those words into a `Vec`.

**Ownership detail worth noticing:** the result type is `Vec<&str>`, not `Vec<String>`. The words are **borrowed slices** pointing into the original input string — no copying. This is the same borrowing idea from Day 2, quietly at work again.

---

## 3. Slice pattern matching — the star of today

Rust can `match` on the **shape and contents** of the word list at the same time — something Java simply cannot do cleanly.

- A pattern like `["go", direction]` matches **only** when there are exactly two words **and** the first is `"go"`, and it **captures** the second word into a variable in the same step.
- Other patterns: one exact word (`["look"]`), alternatives (`["quit"] | ["exit"]`), the empty line (`[]`), and a catch-all (`_`).

**Java comparison:** in Java this would be a chain of `if (parts.length == 2 && parts[0].equals("go"))` with manual indexing and length checks. Rust lets you describe the pattern directly and bind the piece you want in one line. This is where pattern matching feels like a superpower, not just a fancier `switch`.

---

## 4. Graceful errors: retiring `.unwrap()`

Recall the Day 2 panic — `.unwrap()` on a missing map key crashed the program.

- `HashMap::get` returns an `Option` (Rust's `Optional`, but one you cannot ignore).
- `.unwrap()` means "give me the value **or crash**."
- The grown-up version is to `match` on the `Option` and handle **both** the `Some` and `None` cases.

Result: a missing room now prints a message instead of killing the program. That's the difference between a toy and something robust.

---

## Structure improvement: a function that returns ownership

World-building moved into its own `build_world()` function that **returns** the whole `HashMap`.

- The return type says the function hands back ownership of the map it built, and `main` becomes the new owner.
- A clean, everyday example of a function _producing_ an owned value and transferring it out — the friendly side of ownership (Day 2), versus the borrow-checker fights.

Both display functions now `match` on the `Option` from `.get()` instead of unwrapping, so nothing can panic from a missing room anymore. Movement also auto-triggers a "look" after arriving, so each new room is described on entry.

---

## The error I hit: `E0716` — temporary dropped while borrowed

Making the parser case-insensitive by lowercasing the input inline triggered:

```
error[E0716]: temporary value dropped while borrowed
```

**Why:** `.to_lowercase()` creates a **brand-new owned `String`**, but as an inline temporary it has no name and dies at the end of the line. Meanwhile the `Vec<&str>` borrows slices _from_ that temporary. So the vector would be left pointing at a `String` that's already been freed — a dangling reference, exactly what Rust exists to prevent.

**Fix:** give the lowercased `String` a home (bind it to a variable) so it lives long enough for the borrows into it to stay valid, _then_ split it.

**Lesson:** this is pure Day 2 ownership wearing new clothes — the same "who owns this value, and how long does it live?" question, in a new disguise. A borrow can never outlive the thing it borrows from.

---

## Takeaways

- Input arrives as one raw `String`; **parsing** (`trim` → `split_whitespace` → `collect`) turns it into words.
- Split words come back as `Vec<&str>` — **borrowed slices**, not owned copies (Day 2 borrowing, again).
- **Slice pattern matching** matches shape + contents and binds captured pieces in one step — far cleaner than Java's length/index checks.
- Replace `.unwrap()` with a `match` on `Option` (`Some`/`None`) to handle missing values **without crashing**.
- A function can **return an owned value** (`build_world`), transferring ownership to the caller — the friendly face of ownership.
- `E0716` (temporary dropped while borrowed) is the borrow rule again: **a borrow can't outlive what it borrows from** — bind the temporary to a variable so it lives long enough.
