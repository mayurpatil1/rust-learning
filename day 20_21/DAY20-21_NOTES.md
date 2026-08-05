# Days 20–21 — High-Score CLI (Consolidation)

> Learning Rust by building small projects. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Project:** a standalone high-score tracker CLI — `add`, `list`, `top` — persisted to JSON.

**Goal:** assemble Week 3's tools into one polished, modular, tested tool that never panics on bad input.

**Crates used:** clap (Day 19), serde + serde_json (Day 3), thiserror (Day 18) — all familiar.

---

## 1. The new piece: clap subcommands

Real tools (`git commit`, `cargo run`) use **subcommands**. clap models these as an **enum**, where each variant is a subcommand and its fields are that subcommand's arguments:

```rust
#[derive(Subcommand)]
enum Command {
    Add { name: String, score: u32 },
    List,
    Top,
}
```

- `add Aria 1500` parses into `Command::Add { name: "Aria", score: 1500 }`.
- Then `match` on the command and act.
- **This is Day 12 (data-carrying enums) meeting Day 19 (clap).**
- A top-level struct holds it via `#[command(subcommand)]`.
- Doc comments (`/// Record a new score`) become the subcommand's help text.

---

## 2. The `run()` / `main()` pattern

The idiomatic way to make `?` usable everywhere:

- **`run() -> Result<(), Error>`** holds all the real logic, so every fallible call (`load()?`, `save()?`, `top()?`) can use `?`.
- **`main()`** does one thing: call `run()`, and if it returns `Err`, print a friendly message and exit.

```rust
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e); // stderr
        std::process::exit(1);     // proper error exit code
    }
}
```

**No panics, no stack traces** — a clean message and a proper exit code. The professional shape for a Rust CLI. (`?` can't be used in a bare `main` because it returns `()`; `run()` gives the errors somewhere to go — Day 18's rule.)

---

## 3. Converting error types: `.map_err()`

Different libraries return **their own** error types, but a function returns **one** custom error. `.map_err()` transforms one into the other before `?` propagates it:

```rust
serde_json::from_str(&json).map_err(|e| ScoreError::Corrupt(e.to_string()))?
```

- serde's parse error → your `ScoreError::Corrupt` → then `?` returns it.
- This is how many error sources funnel into a single custom error enum (the real-world pattern Day 18 set up).

---

## 4. Bridging `Option` and `Result`: `.ok_or()`

`max_by_key` returns `Option` (maybe no entries), but the function returns `Result`:

```rust
self.entries.iter().max_by_key(|e| e.score).ok_or(ScoreError::Empty)
```

- `.ok_or(err)` = "`Some` → `Ok`; `None` → `Err(err)`."
- A constant, tiny bridge between the two types known since Day 3. (Its cousin `.ok()` goes the other way: `Result` → `Option`.)

---

## 5. What each earlier day contributed

| Piece                               | From   |
| ----------------------------------- | ------ |
| modules (`error.rs`, `scores.rs`)   | Day 4  |
| iterators + `sort_by` for ranking   | Day 5  |
| `Default` derive for an empty board | Day 9  |
| serde load/save                     | Day 3  |
| data-carrying enum (subcommands)    | Day 12 |
| unit tests for the logic            | Day 13 |
| custom errors + `?`                 | Day 18 |
| clap parsing                        | Day 19 |

Nothing new except subcommands — the day was **assembly**.

---

## 6. Design notes worth keeping

- **A missing file is not an error.** `load()` treats "no file yet" as an empty board (`Ok(default())`), reserving errors for genuine problems (corruption, write failure). Distinguishing "absent" from "broken" is good design.
- **Test the logic, not the I/O.** Unit tests target `add` / `ranked` / `top` on in-memory boards — fast and deterministic — rather than file reads.
- **Graceful failure (Part C):** a corrupted `scores.json` prints `Error: the score file is corrupted: ...` and exits cleanly — **not** a panic. That's the whole payoff of the error-handling work.

---

## Takeaways

- **clap subcommands** = an enum of variants (`git`/`cargo` style); each variant's fields are its arguments. Doc comments become help text.
- **`run() -> Result` + thin `main()`** makes `?` usable throughout and turns errors into clean messages + exit codes, never panics.
- **`.map_err()`** converts a foreign error type into your custom one before `?` propagates it — funnels many sources into one error enum.
- **`.ok_or()`** turns `Option` into `Result` (`.ok()` goes back the other way).
- Good design: treat **absent** differently from **corrupt**; **test logic, not I/O**.
- The project was pure assembly of Days 3–19 — a sign the toolkit now composes into real tools.

---

## Week 3 (through Day 21) status

The toolkit now spans: ownership & borrowing, lifetimes, `Box`/`Rc`/`RefCell`, traits & generics, collections, data-carrying enums, idiomatic error handling, testing, and CLI parsing — composed into a working, modular, tested command-line tool.
