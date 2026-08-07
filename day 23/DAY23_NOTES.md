# Day 23 — `impl Trait` vs `Box<dyn Trait>` (Static vs Dynamic Dispatch)

> Learning Rust by building small projects. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Project:** a standalone demo storing a mix of creatures behind one trait.

**Goal today:** the last major piece of the trait system — how to store a _mixed_ collection of different types that share a trait, and the tradeoff between the two ways.

---

## 1. The problem

From Day 8: `Goblin`, `Dragon`, `Chest` all implement `Describable`, but this fails:

```rust
let creatures = vec![goblin, dragon, chest]; // ERROR — different types
```

A `Vec<T>` holds **one** type. A goblin and a dragon are different types even though both are `Describable`. Two ways to resolve it — with a real tradeoff.

---

## 2. Static dispatch: `impl Trait`

When each value is a **known concrete type** at compile time:

```rust
fn announce(thing: &impl Describable) { ... }   // used since Day 8
```

- "Static" = the compiler knows the exact type and generates **specialized code** (monomorphization — the Day 10 generics mechanism).
- **Zero runtime cost.**
- **Limit:** one concrete type per call — can't hold a _mix_.

---

## 3. Dynamic dispatch: `Box<dyn Trait>`

When you need a genuine **mix** of types behind one trait:

```rust
let dungeon: Vec<Box<dyn Describable>> = vec![
    Box::new(goblin),
    Box::new(dragon),
    Box::new(chest),
];
```

- Read `Box<dyn Describable>` as _"a heap value that is SOME `Describable` — I don't know which concrete type."_
- `dyn` = **dynamic**: the exact type (and which `describe()` to call) is resolved **at runtime**.
- Now the `Vec` works: every element is the same surface type, `Box<dyn Describable>`.
- Calling `thing.describe()` in a loop dispatches to **each element's own** implementation.

---

## 4. Why `Box` is required (Part B)

A `Vec` needs **same-size** elements, but different types have different sizes (a `Dragon` holds a `String`, a `Chest` holds a `bool`).

- `Vec<dyn Describable>` (no `Box`) fails: _"the size for values of type `dyn Describable` cannot be known at compilation time"_ (`Sized` not satisfied).
- `Box` is a **pointer** — always the same size — so `Box<dyn Describable>` is **uniform** regardless of what it points at.
- **Exactly the Day 16 reasoning** (pointers are fixed-size) applied to trait objects instead of recursive types. Same tool, same logic, new problem.

---

## 5. The tradeoff

|                 | `impl Trait` (static)               | `Box<dyn Trait>` (dynamic)              |
| --------------- | ----------------------------------- | --------------------------------------- |
| **Speed**       | faster — no runtime lookup          | small runtime cost (vtable lookup)      |
| **Flexibility** | one concrete type only              | mix any implementors, incl. future ones |
| **Mechanism**   | monomorphization (specialized code) | a hidden dispatch table (**vtable**)    |

- The "vtable" is the **same concept as Java's virtual method dispatch**.
- **Java does dynamic dispatch by default** for interface/virtual calls — you never think about it. Rust defaults to the **faster static** version and makes you write `dyn` explicitly when you need the mix.
- At the call site, dynamic dispatch is **invisible** — a normal method call / iterator chain. Iterators (Day 5) work fine on a heterogeneous `Vec<Box<dyn Trait>>`.

---

## 6. When to use which

- **`impl Trait` / generics** — the default. One type at a time, maximum speed.
- **`Box<dyn Trait>`** — when you genuinely need a **collection of mixed types** behind a shared trait, or to return different concrete types from one function. Accept the tiny runtime cost for the flexibility.

---

## Takeaways

- A `Vec<T>` holds one type; storing a **mix** of trait implementors needs a **trait object**.
- **`impl Trait` = static dispatch:** compiler knows the type, specialized code, zero cost, single type only.
- **`Box<dyn Trait>` = dynamic dispatch:** the concrete type is resolved at **runtime** via a vtable; enables **heterogeneous collections**.
- **`Box` is required** because trait objects have **no compile-time size** (`Sized`); a pointer is uniform — the Day 16 size argument again.
- Tradeoff: static is **faster**, dynamic is **more flexible**. Java uses dynamic by default; Rust makes the choice explicit.
- Dynamic dispatch is invisible at the call site — iterators and method calls work normally.
