# Day 17 — `Rc` and `RefCell`

> Learning Rust by building small projects. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Project:** a standalone demo of shared ownership and interior mutability.

**Goal today:** the two escape hatches from Rust's core rules — and _why_ they're deliberate exceptions, not defaults.

---

## The two rules being escaped

Everything since Day 2 rested on:

1. **One owner** per value.
2. **Many readers OR one writer**, never both.

Great rules — until a structure genuinely needs to break them. Today's two types each break **one** of them.

---

## 1. `Rc<T>` — multiple owners

`Rc` = "Reference Counted." It lets a value have **more than one owner** (breaks rule 1). It tracks how many owners exist and frees the value only when the **last** one is dropped.

```rust
let shared = Rc::new(String::from("treasure"));
let a = Rc::clone(&shared); // now 2 owners
let b = Rc::clone(&shared); // now 3 owners
Rc::strong_count(&shared);  // 3
```

- **`Rc::clone` is cheap — NOT a deep copy.** It bumps the owner count and hands back another **handle to the same data**. (Unlike normal `.clone()`, which duplicates.)
- **Convention:** write `Rc::clone(&x)`, not `x.clone()`, to signal "sharing, not copying."
- The owner count rises with each clone and **falls automatically** as owners go out of scope — a tiny, _deterministic_ garbage collector you can watch.
- **The catch:** `Rc` gives shared ownership but the data stays **immutable**. Many owners, none can change it.

This is Java's default (every object reference is a shared handle) — Rust just makes you opt in.

---

## 2. `RefCell<T>` — mutate through a shared reference

`RefCell` lets you mutate a value even through a shared (`&`) reference (breaks rule 2). It does this by moving the "one writer at a time" check from **compile time to runtime**.

```rust
let cell = RefCell::new(5);
*cell.borrow_mut() += 10;   // write handle
cell.borrow();              // read handle
```

- `.borrow()` = read handle; `.borrow_mut()` = write handle.
- **The rules still apply** — but breaking them is a **runtime panic**, not a compile error.
- **The trade:** flexibility bought with a panic risk. That's why `RefCell` is a tool of last resort, not a default.

---

## 3. The trade made concrete: the panic (Part A)

Holding two write handles at once:

```
thread 'main' panicked: RefCell already borrowed
```

Same rule as Day 2 (no two mutable borrows at once), enforced at a **later checkpoint**. With plain `&mut` this is a compile error caught before running; `RefCell` defers the same check to runtime, where the failure is a panic. Same rule, worse failure mode — the cost of the flexibility.

---

## 4. `Rc<RefCell<T>>` — the combo

Together they give **multiple owners who can all mutate** the shared value — essentially **Java's default object**: a shared, mutable thing everyone can point at and change.

- `Rc` provides the **sharing**; `RefCell` provides the **mutability**.
- Common enough to recognize on sight.
- **The lesson:** seeing how much machinery Rust needs to reproduce Java's default shows how much Java was quietly doing — and why Rust makes you ask for it explicitly.

---

## 5. Living with `RefCell`: borrow briefly

The practical rule that avoids Part A's panic: **hold a borrow for as short a time as possible.**

- Do mutations inline — `*x.borrow_mut() += 1` — so the handle is created, used, and dropped on one line.
- Don't stash a `borrow_mut()` handle in a variable that lingers; two live handles at once is what panics.
- In the Part B counter, each `borrow_mut()` was a separate statement, so no two overlapped.

---

## Takeaways

- Rust's defaults: **one owner**, **many-readers-xor-one-writer**. `Rc` and `RefCell` each break one — deliberately, as exceptions.
- `Rc<T>` = **multiple owners** via reference counting; freed when the last owner drops. `Rc::clone` shares (cheap), it doesn't copy.
- `Rc` data is **immutable** on its own.
- `RefCell<T>` = **mutate through a shared reference** by moving the borrow check to **runtime** — breaking the rule **panics** instead of failing to compile.
- `Rc<RefCell<T>>` = **shared + mutable** ≈ Java's default object; recognize the pattern.
- Practical rule: **borrow for the shortest time possible** (mutate inline) to avoid runtime borrow panics.
