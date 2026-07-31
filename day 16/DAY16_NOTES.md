# Day 16 — `Box<T>`, the First Smart Pointer

> Learning Rust by building small projects. Coming from Java/Spring Boot, so notes map Rust ideas onto Java where it helps.

**Project:** a standalone recursive dialogue/story tree and a recursive number list.

**Goal today:** heap allocation with `Box<T>`, and the problem it uniquely solves — **recursive types**. A concrete, hands-on breather after lifetimes.

---

## 1. Stack vs heap

- **Stack:** fast, automatic, but every value must have a size **known at compile time**.
- **Heap:** for values whose size is dynamic or large.

`String` and `Vec` already store their **contents** on the heap; the variable on the stack is just a small pointer + length. So the heap has been in use all along — just unnamed.

---

## 2. What `Box<T>` is

`Box<T>` = **an owned value of type `T`, living on the heap, with a fixed-size pointer to it on the stack.**

```rust
let boxed = Box::new(5); // the 5 lives on the heap
println!("{}", *boxed);  // * dereferences to reach the value
```

Boxing a plain number is pointless in real code. `Box` earns its keep for **one** job.

---

## 3. The job: recursive types

A type that contains **itself** won't compile directly:

```rust
enum Story {
    Scene(String, Story), // ERROR: recursive type has infinite size
    TheEnd,
}
```

**Why:** to know a `Story`'s size, the compiler needs the size of the `Story` inside it, which needs the one inside _that_... forever. A value directly containing itself would need infinite space.

**`Box` fixes it** — a `Box` is a **pointer**, and a pointer has a **fixed, known size** no matter how big the thing it points to is:

```rust
enum Story {
    Scene(String, Box<Story>), // compiles!
    TheEnd,
}
```

Each node now holds a `String` + a **pointer** to the next node, not the next node itself. Size becomes finite; the recursion lives on the heap, chained by pointers — exactly how linked lists and trees work under the hood.

**The error (Part A):** removing the `Box` gives _"recursive type has infinite size,"_ and the compiler **suggests inserting a `Box`**. That error is the entire reason `Box` exists.

---

## 4. Recursive data ↔ recursive functions

The code mirrors the data:

```rust
fn tell(story: &Story) {
    match story {
        Story::Scene(text, next) => {
            println!("{}", text);
            tell(next);          // recurse into the boxed next node
        }
        Story::TheEnd => println!("~ The End ~"),
    }
}
```

- The **data** is recursive (each `Scene` holds a `Box<Story>`).
- The **function** is recursive (it calls itself on `next`).
- The base case (`TheEnd`) stops the recursion — every recursive function needs one.

Same shape for `sum` over a recursive list, and for `count_scenes` (Part B): the recursive arm returns `something + recurse(rest)`, the base arm returns `0`.

---

## 5. Building recursive structures inside-out

Nested `Box::new(...)` calls construct the **innermost** value first (`TheEnd`), then wrap it in the scene before it, then wrap that, and so on. "Build from the end backward" is characteristic of recursive structures — expected, not intimidating once seen.

---

## 6. The cons list

`enum List { Node(i32, Box<List>), Empty }` is the classic `Box` teaching example (a "cons list"): a singly linked list where each node owns the next via a `Box`. Summed recursively: `value + sum(rest)`, with `Empty => 0`.

---

## Takeaways

- **Stack** = fixed, compile-time-known sizes; **heap** = dynamic/large. `String`/`Vec` already keep contents on the heap.
- `Box<T>` = an owned value on the **heap** with a **fixed-size pointer** on the stack; `*` dereferences it.
- A type containing **itself** has "infinite size" and won't compile — `Box` makes the recursive link a fixed-size **pointer**, so the size becomes finite.
- This is `Box`'s canonical use: **recursive types** (trees, linked lists, dialogue chains).
- Recursive **data** is walked by recursive **functions**, which always need a **base case** to stop.
- Recursive values are built **inside-out** (innermost first).
