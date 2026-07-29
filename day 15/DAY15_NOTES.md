# Day 15 — Lifetimes

> Learning Rust by building small projects. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Project:** a standalone tour of when lifetimes are invisible, when they're required, and why.

**Goal (calibrated):** **read** lifetimes with understanding — not write them fluently. They're rarely hand-written day to day. Success = looking at `<'a>` and knowing what it says.

**Why it feels alien:** Java's garbage collector hid this entirely. Lifetimes make visible a relationship the GC managed for you.

---

## 1. What a lifetime actually is

A lifetime is a **name** the compiler uses to track _"how long is this reference valid?"_ It enforces the rule already known since Day 3's `E0716`:

> **A reference can never outlive the thing it points to.**

**The key reframe:** a lifetime is a **label, not a lever.** It does **not** tell the compiler how long anything lives, and it can't change any lifespan. It _describes_ a relationship that already exists, so the compiler can verify references are sound.

---

## 2. Why they're usually invisible: elision

Most reference-using code needs no annotation — the compiler **infers** lifetimes ("lifetime elision"). That's why references have been written all month without ever typing `<'a>`.

An annotation is required only when the compiler genuinely can't infer the relationship — which is essentially one situation.

---

## 3. The situation that forces an annotation

**A reference output + more than one reference input.**

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str { ... }
```

- The return could come from `a` **or** `b`; the compiler can't tell which, so it can't verify the output won't outlive its source.
- `<'a>` declares a lifetime named `a` (the tick `'` just means "lifetime").
- `&'a str` on both inputs and the output says: _all three share lifetime `'a`_ → **the returned reference is valid only as long as BOTH inputs are.**
- You didn't change any lifespan — you **stated the relationship** so the compiler can check callers.

**Contrast — one reference input needs nothing:** `fn first_word(text: &str) -> &str` compiles with no annotation, because the output can only come from `text`. The relationship is obvious.

**The instinct to build:**

- one reference input → compiler infers, no annotation.
- reference output + 2+ reference inputs → you must annotate.

That's ~90% of when `<'a>` ever appears.

---

## 4. Lifetime checking works from the SIGNATURE ALONE

`pick_first` always returns `a`, yet still requires annotations. **The compiler never peeks at the function body** — it checks lifetimes from the signature only.

- Removing the annotations gives `error[E0106]: missing lifetime specifier`, with the help: _the return is borrowed, but the signature doesn't say whether from `a` or `_b`._
- It doesn't matter that the body only ever returns `a` — the **signature is the contract**, and it must state the relationship.
- Same principle as reading `&mut` from a signature (Day 6): the signature tells the whole story, so callers can rely on it and the body can change freely.

---

## 5. Structs that hold references need a lifetime

```rust
struct Excerpt<'a> {
    part: &'a str,
}
```

A struct **storing** a reference must declare a lifetime — it says _"an `Excerpt` cannot outlive the text it borrows from."_ Without it, the struct could hold a dangling reference. (Structs that own their data — `String`, not `&str` — need no lifetime, which is why none of the earlier projects' structs had them.)

---

## 6. Why the rule exists (the payoff)

The dangling-reference demo: a `result` reference set from a `temporary` string inside an inner scope, used _after_ that scope ends and `temporary` is dropped. The compiler stops it:

```
error: `temporary` does not live long enough
```

That is a **use-after-free caught at compile time** — the exact bug class Rust exists to prevent. The lifetime `'a` is what lets the compiler _prove_ the reference would dangle. In C this compiles and crashes at runtime; in Rust it never builds.

---

## Takeaways

- A lifetime is a **name/label** for "how long is this reference valid?" — it describes relationships, it doesn't change lifespans.
- It enforces one rule: **a reference can't outlive what it points to** (Day 3's `E0716`, generalized).
- **Elision** infers lifetimes almost always — that's why they've been invisible all month.
- Annotation is required mainly for **a reference output derived from 2+ reference inputs**; one input needs nothing.
- Lifetime checking uses the **signature only**, never the body — the signature is a contract (`E0106` when it's incomplete).
- **Structs holding references** need a lifetime; structs that own their data don't.
- The payoff: **use-after-free becomes a compile error**, provable via lifetimes.
