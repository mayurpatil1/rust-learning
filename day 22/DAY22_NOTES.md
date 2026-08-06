# Day 22 — Testing & Documentation

> Learning Rust by building small projects. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Project:** a standalone **library** crate (`cargo new --lib`) — a small loot-valuing library.

**Goal today:** the three kinds of test, and documentation that's **verified** — doc examples that run as tests, plus a generated docs website.

**Why `--lib`:** documentation and doc tests shine on libraries (code meant to be used by others). Gives `src/lib.rs` instead of `src/main.rs`.

---

## 1. Three kinds of test, three locations

| Kind              | Lives in                                    | Sees                | Tests                                 |
| ----------------- | ------------------------------------------- | ------------------- | ------------------------------------- |
| **Unit** (Day 13) | `#[cfg(test)] mod tests` in the source file | private internals   | internal logic                        |
| **Integration**   | a separate `tests/` directory               | **public API only** | the library as a real user sees it    |
| **Doc**           | inside `///` doc comments                   | public API          | that the docs' examples actually work |

`cargo test` runs **all three** with one command and labels each suite in the output.

- **Integration tests** are compiled as a **separate crate**, so they can only call the public API — catching "is my public interface complete and usable?" problems unit tests miss.
- **Unit tests** use `use super::*`, so they never name the crate externally.

---

## 2. Doc comments: `///` and `//!`

- Regular comment: `//`.
- **`///`** documents the item that **follows** it (a function, struct, etc.). Supports **Markdown**.
- **`//!`** documents the thing it's **inside** — used at the top of a file to describe the whole module/crate (the `!` = "inside").

That inside-vs-following distinction is the only tricky part of doc syntax.

---

## 3. Doc tests — the feature Java doesn't have

A fenced code block inside a `///` comment is a **doc test**: `cargo test` compiles and runs it.

````rust
/// # Examples
/// ```
/// use day22_docs::add;
/// assert_eq!(add(2, 3), 5);
/// ```
````

- If the code changes so the example breaks, **the test fails** — documentation can't silently rot.
- **Java analogy:** like Javadoc, but Javadoc examples are just text that goes stale; Rust **runs** them.
- Each doc test runs as a **separate program using the library**, so it needs its own `use` line to import what it references.
- **Part A win:** deliberately breaking a doc example (wrong `assert_eq!`) makes `cargo test` fail — proof the docs are verified.

---

## 4. `cargo doc` — free documentation website

`cargo doc --open` generates a Javadoc-style **website** from the doc comments — every `///` block rendered with formatting and examples, browsable in the browser. Zero extra work: professional docs from comments you'd write anyway.

---

## 5. The `E0432` lesson: crate name ≠ folder name

The integration test failed with `unresolved import` because the code imported `day22_docs` but the crate was actually named `_docs`.

- The string you `use` to import your **own** library is the crate's **package name** from `Cargo.toml` — **not** the folder name, **not** the filename.
- `cargo new` derives the crate name from the folder, so a folder like `_docs` produced crate `_docs`.
- **Fix:** either match the imports to the real name, or (cleaner) set `name = "..."` in `Cargo.toml`.
- **Why only integration/doc tests broke:** they live **outside** the library and import it by its external crate name, exactly like a real user. Unit tests (`use super::*`) never do. Same inside-vs-outside boundary as the test types.

---

## Takeaways

- **Three test kinds:** unit (in-source, private), integration (`tests/`, public-only, separate crate), doc (in `///` comments). `cargo test` runs all three.
- `///` documents what **follows**; `//!` documents what it's **inside** (crate/module headers).
- **Doc tests run the examples in your docs** — documentation is verified and can't rot. Java's Javadoc can't do this.
- Each doc test needs its own `use` line (it runs as an external user of the crate).
- **`cargo doc --open`** builds a browsable docs site from comments — free.
- You import your own crate by its **`Cargo.toml` package name**, not its folder/file name — the boundary that only affects outside-the-library code.
