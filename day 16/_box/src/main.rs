// Day 16 — Box<T>, the first smart pointer
// Box<T> = an owned value living on the HEAP, with a fixed-size pointer on the stack.
// Its canonical job: making RECURSIVE types possible.

// ---------------------------------------------------------------
// A recursive enum: each Step can point to the NEXT Step.
// Without Box this is "infinite size" — a Step would contain a Step
// would contain a Step... forever. Box makes each link a pointer
// (fixed size), so the size is finite and the chain lives on the heap.
// ---------------------------------------------------------------
enum Story {
    Scene(String, Box<Story>), // some text, then a pointer to what comes next
    TheEnd,
}

// Walk the recursive structure. Notice the function calls ITSELF —
// recursion in code, matching the recursion in the data.
fn tell(story: &Story) {
    match story {
        Story::Scene(text, next) => {
            println!("{}", text);
            tell(next); // recurse into the boxed next scene
        }
        Story::TheEnd => println!("~ The End ~"),
    }
}

// A simple recursive list for contrast (a "cons list" — the classic Box example).
enum List {
    Node(i32, Box<List>),
    Empty,
}

fn sum(list: &List) -> i32 {
    match list {
        List::Node(value, rest) => value + sum(rest), // add this value + the rest
        List::Empty => 0,
    }
}

fn main() {
    // ---- Basic Box: a value on the heap ----
    let boxed = Box::new(42);
    println!("Boxed value: {}", *boxed); // * dereferences to reach the i32
    println!("Plus one:    {}", *boxed + 1);

    // ---- Build a story by nesting scenes, innermost first ----
    let story = Story::Scene(
        String::from("You enter a dark cave."),
        Box::new(Story::Scene(
            String::from("A dragon stirs in the shadows."),
            Box::new(Story::Scene(
                String::from("You draw your sword..."),
                Box::new(Story::TheEnd),
            )),
        )),
    );

    println!("\n--- The Story ---");
    tell(&story);

    // ---- A recursive list: 1 -> 2 -> 3 -> Empty ----
    let numbers = List::Node(
        1,
        Box::new(List::Node(
            2,
            Box::new(List::Node(3, Box::new(List::Empty))),
        )),
    );

    println!("\nSum of list: {}", sum(&numbers)); // 6
}