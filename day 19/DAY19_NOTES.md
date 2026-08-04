# Day 19 — CLI Parsing with `clap`

> Learning Rust by building small projects. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Project:** a standalone character-creator CLI.

**Goal today:** turn programs into real command-line tools with flags and arguments — using a derive-based API that feels like Spring's annotation-driven config.

**Setup:** `cargo add clap --features derive` (the `derive` feature unlocks the annotation style, like `serde`'s did on Day 3).

---

## 1. The core idea: the struct IS the CLI

Describe the arguments as an **annotated struct**; `clap` generates the parser, validation, help text, and error messages.

```rust
#[derive(Parser, Debug)]
struct Args {
    name: String,
    #[arg(short, long, default_value = "rogue")]
    class: String,
}

let args = Args::parse(); // one line does all parsing
```

- `#[derive(Parser)]` is a derive macro (like `#[derive(Serialize)]`) that reads the struct and builds a full parser at compile time.
- **Feels like Spring:** you _declare_ what you want with annotations; the framework wires up the behavior.
- Each struct field = one argument. The field's **type drives behavior and validation**.

---

## 2. The three flavors of argument

| Flavor                     | Looks like                 | How to declare                          |
| -------------------------- | -------------------------- | --------------------------------------- |
| **Positional**             | `myprog Aria`              | a bare field (no `#[arg]` name needed)  |
| **Option** (takes a value) | `--class mage` / `-c mage` | `#[arg(short, long)]`                   |
| **Flag** (on/off bool)     | `--verbose`                | `#[arg(short, long)]` on a `bool` field |

- `short` → `-c`; `long` → `--class`. `short = 'H'` picks a specific letter.
- `-- ` in `cargo run -- ...` separates cargo's args from the program's.

---

## 3. Defaults, optional args, and validation

- **Default value:** `default_value_t = 100` (typed) or `default_value = "rogue"` (string) makes an argument optional.
- **Type-driven validation:** declaring `health: u32` makes clap reject `--health abc` automatically — no validation code.
- **Optional argument:** `load: Option<String>` models "may or may not be given." Provided → `Some(path)`; absent → `None`. This is the Day 3 `Option`, matched the same way. (Borrow it in the match — `&args.load` — don't move the `String` out.)

**Adding an argument = adding one struct field.** No parsing logic (Parts A & B: `--level`, `--gold`, `--load` were each just a field).

---

## 4. Restricting values with `ValueEnum`

An enum used as an argument type limits input to its variants:

```rust
#[derive(ValueEnum, Clone, Debug)]
enum Difficulty { Easy, Normal, Hard }
```

- `--difficulty banana` is rejected automatically, with the allowed values listed.
- **No validation logic — the type IS the validation.** Ties back to Day 12: an enum is a closed set of valid options.

---

## 5. Free `--help` and clean errors

- `--help` is **generated from the struct** — field names, defaults, and `#[command(about = "...")]` text, with zero help-text code. Every new field appears automatically.
- Bad or missing input prints an error + usage and **exits with a proper error code** — professional CLI behavior, not a Rust panic with a stack trace.

---

## Takeaways

- **The annotated struct _is_ the CLI spec**; `Args::parse()` does everything. `#[derive(Parser)]` is a derive macro like `serde`'s.
- Three argument flavors: **positional** (bare field), **option** (`#[arg(short, long)]`, takes a value), **flag** (`bool` field).
- **Field type drives validation** — `u32` rejects non-numbers; no code needed.
- `default_value_t` / `default_value` make an argument optional; `Option<T>` models "maybe provided" (missing = `None`).
- `ValueEnum` restricts an argument to an enum's variants — the type is the validation.
- **`--help` and clean error exits are free**, generated from the struct.
