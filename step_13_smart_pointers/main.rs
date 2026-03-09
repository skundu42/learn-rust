// ============================================================
// STEP 13: Smart Pointers
// ============================================================
// Run: rustc main.rs && ./main
//
// Smart pointers are data structures that act like pointers but have
// additional metadata and capabilities. Key smart pointers:
// - Box<T>: heap allocation
// - Rc<T>: reference counting (shared ownership)
// - RefCell<T>: interior mutability
// - Cow<T>: clone-on-write
// ============================================================

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // =========================
    // 13.1 Box<T> — Heap Allocation
    // =========================

    println!("--- Box<T> ---");

    // Box puts data on the heap instead of the stack.
    let boxed = Box::new(5);
    println!("Boxed value: {}", boxed);
    println!("Dereferenced: {}", *boxed); // deref to get inner value

    // Use case 1: Known size for recursive types
    // Without Box, the compiler can't know the size of a recursive type.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List: {:?}", list);
    println!("Sum: {}", list.sum());

    // Use case 2: Large data you don't want on the stack
    let large = Box::new([0u8; 1_000_000]); // 1MB on heap, not stack
    println!("Large array length: {}", large.len());

    // Use case 3: Trait objects (covered in step 09)
    let shapes: Vec<Box<dyn Area>> = vec![
        Box::new(Square { side: 5.0 }),
        Box::new(CircleShape { radius: 3.0 }),
    ];
    for shape in &shapes {
        println!("Area: {:.2}", shape.area());
    }

    // =========================
    // 13.2 Deref Trait
    // =========================

    println!("\n--- Deref ---");

    // The Deref trait lets you customize the behavior of `*` (dereference).
    let my_box = MyBox::new(String::from("hello"));
    println!("MyBox: {}", *my_box); // uses our Deref implementation

    // Deref coercion: Rust auto-converts &MyBox<String> -> &String -> &str
    hello(&my_box); // works because of deref coercion chain

    // =========================
    // 13.3 Drop Trait
    // =========================

    println!("\n--- Drop ---");
    {
        let _resource = Resource {
            name: String::from("database connection"),
        };
        let _resource2 = Resource {
            name: String::from("file handle"),
        };
        println!("Resources created");
        // They will be dropped in reverse order at end of scope
    }
    println!("After scope — resources were dropped");

    // Force early drop:
    let resource = Resource {
        name: String::from("early drop"),
    };
    println!("Before explicit drop");
    drop(resource); // explicitly drop early
    println!("After explicit drop");
    // resource is no longer valid here

    // =========================
    // 13.4 Rc<T> — Reference Counting
    // =========================

    println!("\n--- Rc<T> ---");

    // Rc lets multiple owners share the same data (single-threaded only).
    let shared_data = Rc::new(String::from("shared value"));
    println!("Reference count: {}", Rc::strong_count(&shared_data)); // 1

    let clone1 = Rc::clone(&shared_data); // increment ref count (cheap!)
    println!("Reference count: {}", Rc::strong_count(&shared_data)); // 2

    {
        let clone2 = Rc::clone(&shared_data);
        println!("Reference count: {}", Rc::strong_count(&shared_data)); // 3
        println!("clone2: {}", clone2);
    } // clone2 dropped, count decremented

    println!("Reference count: {}", Rc::strong_count(&shared_data)); // 2
    println!("clone1: {}", clone1);
    println!("original: {}", shared_data);

    // Practical use: shared ownership in a graph/tree
    let node_a = Rc::new(RcNode {
        value: 1,
        next: None,
    });
    let node_b = Rc::new(RcNode {
        value: 2,
        next: Some(Rc::clone(&node_a)),
    });
    let node_c = Rc::new(RcNode {
        value: 3,
        next: Some(Rc::clone(&node_a)), // both b and c point to a
    });
    println!("B -> {:?}", node_b);
    println!("C -> {:?}", node_c);
    println!("A ref count: {}", Rc::strong_count(&node_a)); // 3

    // =========================
    // 13.5 RefCell<T> — Interior Mutability
    // =========================

    println!("\n--- RefCell<T> ---");

    // RefCell allows mutable borrows at RUNTIME (checked dynamically).
    // Useful when you need to mutate data behind an immutable reference.

    let data = RefCell::new(vec![1, 2, 3]);

    // Borrow immutably:
    println!("Data: {:?}", data.borrow());

    // Borrow mutably:
    data.borrow_mut().push(4);
    println!("After push: {:?}", data.borrow());

    // Multiple immutable borrows are OK:
    {
        let _r1 = data.borrow();
        let _r2 = data.borrow();
        // let _w = data.borrow_mut(); // PANICS at runtime! Can't mix
    }

    // =========================
    // 13.6 Rc<RefCell<T>> — Shared Mutable Data
    // =========================

    println!("\n--- Rc<RefCell<T>> ---");

    // The power combo: multiple owners + mutability
    let shared_list = Rc::new(RefCell::new(vec![1, 2, 3]));

    let owner1 = Rc::clone(&shared_list);
    let owner2 = Rc::clone(&shared_list);

    // owner1 modifies:
    owner1.borrow_mut().push(4);

    // owner2 also modifies:
    owner2.borrow_mut().push(5);

    // Everyone sees the changes:
    println!("Shared list: {:?}", shared_list.borrow());

    // Practical example: Observer pattern
    let logger = Rc::new(RefCell::new(MessageLogger::new()));
    let logger_clone = Rc::clone(&logger);

    logger.borrow_mut().log("First message");
    logger_clone.borrow_mut().log("Second message");
    logger.borrow_mut().log("Third message");

    println!("All messages: {:?}", logger.borrow().messages());

    // =========================
    // 13.7 Cow<T> — Clone on Write
    // =========================

    println!("\n--- Cow<T> ---");

    use std::borrow::Cow;

    // Cow delays cloning until mutation is needed.
    fn process_name(name: &str) -> Cow<str> {
        if name.contains(' ') {
            // Need to modify — must clone
            Cow::Owned(name.replace(' ', "_"))
        } else {
            // No modification needed — just borrow
            Cow::Borrowed(name)
        }
    }

    let name1 = process_name("alice");        // borrows, no allocation
    let name2 = process_name("bob smith");    // allocates new String
    println!("name1: {} (borrowed)", name1);
    println!("name2: {} (owned)", name2);

    // =========================
    // 13.8 Weak<T> — Breaking Reference Cycles
    // =========================

    println!("\n--- Weak<T> ---");

    use std::rc::Weak;

    // Rc can create reference cycles (memory leak). Weak breaks them.
    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        parent: RefCell<Weak<TreeNode>>,
        children: RefCell<Vec<Rc<TreeNode>>>,
    }

    let leaf = Rc::new(TreeNode {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(TreeNode {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Set leaf's parent to branch (using Weak to avoid cycle)
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade().map(|p| p.value));
    println!("Branch children: {:?}", branch.children.borrow().iter().map(|c| c.value).collect::<Vec<_>>());
    println!("Branch strong count: {}", Rc::strong_count(&branch));
    println!("Branch weak count: {}", Rc::weak_count(&branch));

    println!("\n--- Step 13 Complete! ---");
    println!("Next: step_14 — Concurrency");
}

// ============================================================
// Supporting Types
// ============================================================

// --- 13.1 Recursive type with Box ---

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

impl List {
    fn sum(&self) -> i32 {
        match self {
            Cons(val, next) => val + next.sum(),
            Nil => 0,
        }
    }
}

trait Area {
    fn area(&self) -> f64;
}

struct Square {
    side: f64,
}
struct CircleShape {
    radius: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
impl Area for CircleShape {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// --- 13.2 Custom smart pointer with Deref ---

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// --- 13.3 Drop trait ---

struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("  Dropping resource: {}", self.name);
    }
}

// --- 13.4 Rc node ---

#[derive(Debug)]
struct RcNode {
    value: i32,
    next: Option<Rc<RcNode>>,
}

// --- 13.6 Message logger ---

#[derive(Debug)]
struct MessageLogger {
    msgs: Vec<String>,
}

impl MessageLogger {
    fn new() -> Self {
        MessageLogger { msgs: Vec::new() }
    }
    fn log(&mut self, msg: &str) {
        self.msgs.push(msg.to_string());
    }
    fn messages(&self) -> &[String] {
        &self.msgs
    }
}

// ============================================================
// EXERCISES:
// 1. Create a binary tree using Box<T> with insert and search methods.
// 2. Use Rc<T> to create a graph where multiple nodes point to a shared node.
// 3. Use RefCell<T> to implement a simple cache that computes values lazily.
// 4. Create a parent-child relationship using Rc and Weak to avoid cycles.
// ============================================================
