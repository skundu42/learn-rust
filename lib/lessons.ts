export type Track = "fundamentals" | "advanced" | "dsa" | "projects";

export interface Lesson {
  id: number;
  slug: string;
  title: string;
  track: Track;
  description: string;
  concepts: string[];
  difficulty: "beginner" | "intermediate" | "advanced";
  starterCode: string;
  solutionHint: string;
}

export interface TrackMeta {
  id: Track;
  label: string;
  description: string;
  color: string;
  range: [number, number];
}

export const TRACKS: TrackMeta[] = [
  {
    id: "fundamentals",
    label: "Fundamentals",
    description: "Core Rust concepts every developer needs",
    color: "#e75a2b",
    range: [1, 12],
  },
  {
    id: "advanced",
    label: "Advanced Rust",
    description: "Deep dives into the power of Rust",
    color: "#3b82f6",
    range: [13, 20],
  },
  {
    id: "dsa",
    label: "Data Structures & Algorithms",
    description: "DSA patterns for technical interviews",
    color: "#a855f7",
    range: [21, 30],
  },
  {
    id: "projects",
    label: "Real-World Projects",
    description: "Build production-grade Rust applications",
    color: "#22c55e",
    range: [31, 36],
  },
];

export const LESSONS: Lesson[] = [
  // ── FUNDAMENTALS ─────────────────────────────────────────────────────────
  {
    id: 1,
    slug: "hello-world",
    title: "Hello World & Basics",
    track: "fundamentals",
    description:
      "Start your Rust journey. Learn about println!, string formatting, comments, and escape characters — the foundation of every Rust program.",
    concepts: ["println!", "string formatting", "comments", "escape characters"],
    difficulty: "beginner",
    starterCode: `// Welcome to Rust! Let's print your first message.
fn main() {
    // TODO: Print "Hello, Rustacean!" using println!
    // Then print your name and a fun fact about yourself.
    
    println!("Hello, World!");
}`,
    solutionHint: 'Use println!("Hello, {}!", name) with placeholders.',
  },
  {
    id: 2,
    slug: "variables-and-types",
    title: "Variables, Types & Mutability",
    track: "fundamentals",
    description:
      "Rust variables are immutable by default. Learn about let, mut, constants, shadowing, scalar types, tuples, arrays, and type conversions.",
    concepts: ["let", "mut", "const", "shadowing", "scalar types", "tuples", "arrays"],
    difficulty: "beginner",
    starterCode: `fn main() {
    // TODO: Create an immutable variable for your age (u8)
    // and a mutable variable for your score (i32).
    // Then shadow the score variable to double it.
    
    let age: u8 = 25;
    println!("Age: {}", age);
    
    // Try adding mut and changing a value:
    
}`,
    solutionHint: "Use `let mut` for mutable variables. Shadow with `let x = x * 2;`",
  },
  {
    id: 3,
    slug: "functions-and-control-flow",
    title: "Functions & Control Flow",
    track: "fundamentals",
    description:
      "Define functions, understand expressions vs statements, and master if/else, loop, while, and for constructs.",
    concepts: ["fn", "expressions", "statements", "if/else", "loop", "while", "for"],
    difficulty: "beginner",
    starterCode: `fn main() {
    // TODO: Write a function that checks if a number is even or odd
    // and call it for numbers 1 through 5 using a for loop.
    
    for i in 1..=5 {
        println!("{} is {}", i, if i % 2 == 0 { "even" } else { "odd" });
    }
}

// TODO: Write a is_prime(n: u32) -> bool function here`,
    solutionHint: "Use `fn name(param: Type) -> ReturnType { ... }` syntax.",
  },
  {
    id: 4,
    slug: "ownership-and-borrowing",
    title: "Ownership & Borrowing",
    track: "fundamentals",
    description:
      "THE most important Rust concept. Understand move semantics, references, mutable borrows, and string slices — how Rust achieves memory safety without GC.",
    concepts: ["ownership", "move semantics", "references", "mutable borrows", "slices"],
    difficulty: "beginner",
    starterCode: `fn main() {
    // TODO: Demonstrate ownership by:
    // 1. Creating a String
    // 2. Moving it into a function
    // 3. Borrowing it with &
    // 4. Mutably borrowing it with &mut
    
    let mut s = String::from("Hello");
    println!("Before: {}", s);
    
    append_exclamation(&mut s);
    println!("After: {}", s);
}

fn append_exclamation(s: &mut String) {
    s.push('!');
}`,
    solutionHint: "Pass `&value` to borrow, `&mut value` to mutably borrow.",
  },
  {
    id: 5,
    slug: "structs-and-enums",
    title: "Structs & Enums",
    track: "fundamentals",
    description:
      "Model your data with structs and enums. Implement methods with impl blocks, use Option<T> to handle nullable values.",
    concepts: ["struct", "enum", "impl", "methods", "Option<T>"],
    difficulty: "beginner",
    starterCode: `#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // TODO: Add a new() constructor
    // TODO: Add a distance_to(&self, other: &Point) -> f64 method
    
    fn display(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

fn main() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    p1.display();
    p2.display();
    // TODO: print the distance between p1 and p2
}`,
    solutionHint: "Use `(self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()`",
  },
  {
    id: 6,
    slug: "pattern-matching",
    title: "Pattern Matching",
    track: "fundamentals",
    description:
      "Master match expressions, guards, @bindings, if let, while let, and let-else for elegant control flow.",
    concepts: ["match", "guards", "@binding", "if let", "while let", "let-else"],
    difficulty: "beginner",
    starterCode: `#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String), // State quarter
}

fn value_in_cents(coin: &Coin) -> u32 {
    // TODO: Use match to return the correct value for each coin
    // Quarter should also print the state name
    match coin {
        Coin::Penny => 1,
        _ => 0, // Replace with proper matching
    }
}

fn main() {
    let coins = vec![
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter("Alaska".to_string()),
    ];
    
    for coin in &coins {
        println!("{:?} = {} cents", coin, value_in_cents(coin));
    }
}`,
    solutionHint: "Match on all enum variants. Use `Coin::Quarter(state)` to bind the state.",
  },
  {
    id: 7,
    slug: "collections",
    title: "Collections",
    track: "fundamentals",
    description:
      "Work with Vec, String, HashMap, VecDeque, HashSet, and BTreeMap — Rust's essential data collections.",
    concepts: ["Vec", "String", "HashMap", "HashSet", "BTreeMap", "VecDeque"],
    difficulty: "beginner",
    starterCode: `use std::collections::HashMap;

fn main() {
    // TODO: Create a word frequency counter
    // Count how many times each word appears in the text
    
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let mut counts: HashMap<&str, u32> = HashMap::new();
    
    for word in text.split_whitespace() {
        // TODO: increment the count for each word
    }
    
    // TODO: Print words sorted by frequency (highest first)
    let mut pairs: Vec<(&&str, &u32)> = counts.iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1));
    
    for (word, count) in pairs {
        println!("{}: {}", word, count);
    }
}`,
    solutionHint: "Use `*counts.entry(word).or_insert(0) += 1;`",
  },
  {
    id: 8,
    slug: "error-handling",
    title: "Error Handling",
    track: "fundamentals",
    description:
      "Handle errors gracefully with Result<T,E>, the ? operator, panic!, and custom error types.",
    concepts: ["Result", "panic!", "? operator", "custom errors", "From trait"],
    difficulty: "intermediate",
    starterCode: `use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    ParseError(ParseIntError),
    NegativeNumber(i32),
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e)
    }
}

fn parse_positive(s: &str) -> Result<i32, AppError> {
    // TODO: Parse the string as i32, then return an error if negative
    let n: i32 = s.parse()?; // The ? operator propagates ParseIntError
    if n < 0 {
        Err(AppError::NegativeNumber(n))
    } else {
        Ok(n)
    }
}

fn main() {
    let inputs = ["42", "-5", "abc", "100"];
    for input in inputs {
        match parse_positive(input) {
            Ok(n) => println!("{} -> Ok({})", input, n),
            Err(AppError::ParseError(e)) => println!("{} -> ParseError: {}", input, e),
            Err(AppError::NegativeNumber(n)) => println!("{} -> Negative: {}", input, n),
        }
    }
}`,
    solutionHint: "The ? operator automatically converts and returns Err. Implement From for type conversion.",
  },
  {
    id: 9,
    slug: "traits-and-generics",
    title: "Traits & Generics",
    track: "fundamentals",
    description:
      "Define shared behavior with traits, write generic functions with bounds, and use impl Trait and dyn Trait.",
    concepts: ["trait", "generics", "bounds", "impl Trait", "dyn Trait"],
    difficulty: "intermediate",
    starterCode: `trait Summary {
    fn summarize(&self) -> String;
    fn preview(&self) -> String {
        format!("{}...", &self.summarize()[..50.min(self.summarize().len())])
    }
}

struct Article {
    title: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
}

// TODO: Implement Summary for Article and Tweet

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let article = Article {
        title: "Rust is Amazing".to_string(),
        author: "Alice".to_string(),
        content: "Rust provides memory safety without garbage collection.".to_string(),
    };
    
    let tweet = Tweet {
        username: "alice_codes".to_string(),
        content: "Just learned about traits in Rust!".to_string(),
    };
    
    notify(&article);
    notify(&tweet);
}`,
    solutionHint: "Use `impl TraitName for StructName { fn method(&self) -> ReturnType { ... } }`",
  },
  {
    id: 10,
    slug: "lifetimes",
    title: "Lifetimes",
    track: "fundamentals",
    description:
      "Understand lifetime annotations, elision rules, struct lifetimes, and the 'static lifetime.",
    concepts: ["lifetime annotations", "elision", "struct lifetimes", "'static"],
    difficulty: "intermediate",
    starterCode: `// Lifetimes tell the compiler how long references are valid.

// TODO: Add a lifetime annotation to make this function compile.
// It should return the longer of two string slices.
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

// A struct that holds a reference needs a lifetime annotation:
struct Important<'a> {
    content: &'a str,
}

impl<'a> Important<'a> {
    fn announce(&self) -> &str {
        self.content
    }
}

fn main() {
    let s1 = String::from("long string is long");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        println!("Longest: {}", result);
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("no sentence");
    let important = Important { content: first_sentence };
    println!("Important: {}", important.announce());
}`,
    solutionHint: "Add `<'a>` after the function name: `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`",
  },
  {
    id: 11,
    slug: "closures-and-iterators",
    title: "Closures & Iterators",
    track: "fundamentals",
    description:
      "Write expressive code with closures (Fn, FnMut, FnOnce), iterator adaptors, and custom iterators.",
    concepts: ["closures", "Fn/FnMut/FnOnce", "map", "filter", "fold", "custom iterators"],
    difficulty: "intermediate",
    starterCode: `fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // TODO: Use iterator adaptors to:
    // 1. Filter even numbers
    // 2. Square each number
    // 3. Sum the results
    
    let result: i32 = numbers
        .iter()
        // .filter(...)
        // .map(...)
        .sum();
    
    println!("Sum of squares of even numbers: {}", result); // Expected: 220
    
    // TODO: Create a closure that adds a captured value
    let offset = 10;
    let add_offset = |x| x + offset;
    
    let shifted: Vec<i32> = numbers.iter().map(|&x| add_offset(x)).collect();
    println!("Shifted: {:?}", shifted);
}`,
    solutionHint: "Chain `.filter(|&&x| x % 2 == 0).map(|&x| x * x).sum()`",
  },
  {
    id: 12,
    slug: "modules-and-crates",
    title: "Modules & Crates",
    track: "fundamentals",
    description:
      "Organize code with mod, pub, use, re-exports, and the prelude pattern.",
    concepts: ["mod", "pub", "use", "re-exports", "prelude pattern"],
    difficulty: "intermediate",
    starterCode: `mod geometry {
    pub struct Circle {
        pub radius: f64,
    }
    
    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }
    
    pub trait Shape {
        fn area(&self) -> f64;
        fn perimeter(&self) -> f64;
    }
    
    // TODO: Implement Shape for Circle and Rectangle
    
    pub mod utils {
        pub fn format_area(area: f64) -> String {
            format!("{:.2} sq units", area)
        }
    }
}

use geometry::{Circle, Rectangle, Shape};
use geometry::utils::format_area;

fn main() {
    let c = Circle { radius: 5.0 };
    let r = Rectangle { width: 4.0, height: 6.0 };
    
    // TODO: uncomment after implementing Shape
    // println!("Circle area: {}", format_area(c.area()));
    // println!("Rectangle area: {}", format_area(r.area()));
    println!("Circle radius: {}", c.radius);
    println!("Rectangle: {}x{}", r.width, r.height);
}`,
    solutionHint: "Use `impl Shape for Circle { fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius } }`",
  },

  // ── ADVANCED ─────────────────────────────────────────────────────────────
  {
    id: 13,
    slug: "smart-pointers",
    title: "Smart Pointers",
    track: "advanced",
    description:
      "Use Box for heap allocation, Rc for shared ownership, RefCell for interior mutability, and Cow for efficient cloning.",
    concepts: ["Box", "Rc", "RefCell", "Cow", "Weak", "Deref", "Drop"],
    difficulty: "advanced",
    starterCode: `use std::rc::Rc;
use std::cell::RefCell;

// A simple linked list using Box
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Shared mutable state using Rc<RefCell<T>>
fn shared_counter() {
    let counter = Rc::new(RefCell::new(0));
    
    let c1 = Rc::clone(&counter);
    let c2 = Rc::clone(&counter);
    
    *c1.borrow_mut() += 1;
    *c2.borrow_mut() += 1;
    
    println!("Counter: {}", counter.borrow()); // 2
}

fn main() {
    // Build a list: 1 -> 2 -> 3 -> Nil
    let list = List::Cons(1,
        Box::new(List::Cons(2,
            Box::new(List::Cons(3,
                Box::new(List::Nil))))));
    
    println!("{:?}", list);
    shared_counter();
    
    // TODO: Create an Rc<Vec<String>> shared between two owners
    // and push items from each owner
}`,
    solutionHint: "Use `Rc::new(RefCell::new(vec![]))` and `rc.borrow_mut().push(...)`",
  },
  {
    id: 14,
    slug: "concurrency",
    title: "Concurrency",
    track: "advanced",
    description:
      "Spawn threads, communicate with channels (mpsc), share state with Mutex and Arc.",
    concepts: ["threads", "mpsc channels", "Mutex", "Arc", "parallel patterns"],
    difficulty: "advanced",
    starterCode: `use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // TODO: Use Arc<Mutex<Vec<i32>>> to collect results from 5 threads
    // Each thread should push its thread ID (0..5) into the shared vector
    
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    
    for i in 0..5 {
        let results = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let mut vec = results.lock().unwrap();
            vec.push(i);
        });
        handles.push(handle);
    }
    
    for h in handles {
        h.join().unwrap();
    }
    
    let mut final_results = results.lock().unwrap();
    final_results.sort();
    println!("Results: {:?}", final_results);
    
    // TODO: Also demonstrate message passing with std::sync::mpsc
}`,
    solutionHint: "Use `Arc::clone(&data)` before moving into the thread closure.",
  },
  {
    id: 15,
    slug: "async-await",
    title: "Async/Await",
    track: "advanced",
    description:
      "Write asynchronous Rust with async fn, .await, Futures, and a manual executor.",
    concepts: ["async fn", ".await", "Future", "executor", "async runtime"],
    difficulty: "advanced",
    starterCode: `use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};

// A simple async function
async fn fetch_data(id: u32) -> String {
    // In real code this would be an HTTP call
    format!("Data for id={}", id)
}

async fn process() {
    let data = fetch_data(42).await;
    println!("{}", data);
    
    // TODO: Await multiple futures
    let (a, b) = (fetch_data(1).await, fetch_data(2).await);
    println!("Got: {} and {}", a, b);
}

// Minimal block_on executor
fn block_on<F: Future>(f: F) -> F::Output {
    let waker = dummy_waker();
    let mut cx = Context::from_waker(&waker);
    let mut pinned = Box::pin(f);
    loop {
        match pinned.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => {}
        }
    }
}

fn dummy_waker() -> Waker {
    fn no_op(_: *const ()) {}
    fn clone(ptr: *const ()) -> RawWaker { RawWaker::new(ptr, &VTABLE) }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}

fn main() {
    block_on(process());
}`,
    solutionHint: "Mark functions with `async fn` and call them with `.await`.",
  },
  {
    id: 16,
    slug: "macros",
    title: "Macros",
    track: "advanced",
    description:
      "Write powerful macros with macro_rules!, understand fragment types, repetition, and build DSL macros.",
    concepts: ["macro_rules!", "fragment types", "repetition", "DSL macros"],
    difficulty: "advanced",
    starterCode: `// Create a macro that generates a HashMap from key-value pairs
macro_rules! map {
    // TODO: Match key => value pairs separated by commas
    ($($key:expr => $val:expr),* $(,)?) => {{
        let mut m = std::collections::HashMap::new();
        $(m.insert($key, $val);)*
        m
    }};
}

// Create a macro that prints with a label
macro_rules! debug_val {
    ($val:expr) => {
        println!("[DEBUG] {} = {:?}", stringify!($val), $val)
    };
}

fn main() {
    let scores = map!(
        "Alice" => 95,
        "Bob" => 87,
        "Carol" => 92,
    );
    
    debug_val!(scores);
    
    // TODO: Write a vec_of_strings! macro that creates Vec<String>
    // Usage: vec_of_strings!["hello", "world"] -> Vec<String>
    
    println!("Done!");
}`,
    solutionHint: "Use `$(,)?` for optional trailing comma. Use `stringify!($val)` to get variable name as string.",
  },
  {
    id: 17,
    slug: "unsafe-rust",
    title: "Unsafe Rust",
    track: "advanced",
    description:
      "Understand when and how to use unsafe: raw pointers, FFI C interop, mutable statics, and unions.",
    concepts: ["raw pointers", "unsafe blocks", "FFI", "mutable statics", "unions"],
    difficulty: "advanced",
    starterCode: `// Unsafe Rust gives you superpowers — use carefully!

fn main() {
    // Raw pointers bypass borrow checker
    let x = 42;
    let r = &x as *const i32;  // raw pointer
    
    unsafe {
        println!("Raw pointer value: {}", *r);
    }
    
    // TODO: Implement a safe wrapper around an unsafe operation
    // that swaps two values using raw pointers
    
    let mut a = 10;
    let mut b = 20;
    unsafe_swap(&mut a, &mut b);
    println!("After swap: a={}, b={}", a, b);
}

fn unsafe_swap(a: &mut i32, b: &mut i32) {
    unsafe {
        let temp = std::ptr::read(a);
        std::ptr::write(a, std::ptr::read(b));
        std::ptr::write(b, temp);
    }
}

// FFI example — calling a C function
extern "C" {
    fn abs(x: i32) -> i32;
}

fn safe_abs(x: i32) -> i32 {
    unsafe { abs(x) }
}`,
    solutionHint: "Wrap unsafe code in `unsafe { }` blocks. Use `std::ptr::read/write` for pointer operations.",
  },
  {
    id: 18,
    slug: "capstone-cli-task-manager",
    title: "Capstone: CLI Task Manager",
    track: "advanced",
    description:
      "Combine everything from steps 1-17 to build a full CLI task manager with structs, traits, iterators, error handling, and file persistence.",
    concepts: ["structs", "enums", "traits", "iterators", "error handling", "file I/O"],
    difficulty: "advanced",
    starterCode: `use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum Priority { Low, Medium, High }

#[derive(Debug, Clone, PartialEq)]
enum Status { Todo, InProgress, Done }

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    priority: Priority,
    status: Status,
}

impl Task {
    fn new(id: u32, title: &str, priority: Priority) -> Self {
        Task { id, title: title.to_string(), priority, status: Status::Todo }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = match self.status {
            Status::Todo => "[ ]",
            Status::InProgress => "[~]",
            Status::Done => "[x]",
        };
        write!(f, "{} #{} {:?} - {}", status, self.id, self.priority, self.title)
    }
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    fn new() -> Self { TaskManager { tasks: Vec::new(), next_id: 1 } }
    
    fn add(&mut self, title: &str, priority: Priority) -> u32 {
        let id = self.next_id;
        self.tasks.push(Task::new(id, title, priority));
        self.next_id += 1;
        id
    }
    
    fn complete(&mut self, id: u32) -> bool {
        if let Some(t) = self.tasks.iter_mut().find(|t| t.id == id) {
            t.status = Status::Done;
            true
        } else { false }
    }
    
    fn list(&self) -> &[Task] { &self.tasks }
    
    fn stats(&self) -> HashMap<String, usize> {
        let mut m = HashMap::new();
        m.insert("total".into(), self.tasks.len());
        m.insert("done".into(), self.tasks.iter().filter(|t| t.status == Status::Done).count());
        m
    }
}

fn main() {
    let mut manager = TaskManager::new();
    
    let id1 = manager.add("Learn Rust ownership", Priority::High);
    let id2 = manager.add("Build a CLI app", Priority::Medium);
    let _id3 = manager.add("Write tests", Priority::Low);
    
    manager.complete(id1);
    
    println!("=== Tasks ===");
    for task in manager.list() {
        println!("{}", task);
    }
    
    println!("\\n=== Stats ===");
    let stats = manager.stats();
    println!("Total: {}, Done: {}", stats["total"], stats["done"]);
}`,
    solutionHint: "This is a complete implementation. Extend it with delete, filter, and search methods.",
  },
  {
    id: 19,
    slug: "advanced-traits",
    title: "Advanced Traits",
    track: "advanced",
    description:
      "Operator overloading, blanket implementations, object safety, and disambiguation with fully qualified syntax.",
    concepts: ["operator overloading", "blanket impls", "object safety", "disambiguation"],
    difficulty: "advanced",
    starterCode: `use std::ops::{Add, Mul};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

// TODO: Implement Add for Vec2 (vector addition)
// TODO: Implement Mul<f64> for Vec2 (scalar multiplication)
// TODO: Implement Display for Vec2 to print as "(x, y)"

impl Vec2 {
    fn new(x: f64, y: f64) -> Self { Vec2 { x, y } }
    fn magnitude(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
    fn dot(&self, other: Vec2) -> f64 { self.x * other.x + self.y * other.y }
}

fn main() {
    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);
    
    // These will work after you implement the traits:
    // let sum = v1 + v2;
    // let scaled = v1 * 2.0;
    // println!("Sum: {}", sum);
    // println!("Scaled: {}", scaled);
    
    println!("v1 magnitude: {:.2}", v1.magnitude());
    println!("dot product: {}", v1.dot(v2));
}`,
    solutionHint: "Implement `impl Add for Vec2 { type Output = Vec2; fn add(self, rhs: Vec2) -> Vec2 { ... } }`",
  },
  {
    id: 20,
    slug: "type-system-patterns",
    title: "Type System Patterns",
    track: "advanced",
    description:
      "Newtype pattern, builder pattern, typestate pattern, phantom types, and const generics.",
    concepts: ["newtype", "builder pattern", "typestate", "phantom types", "const generics"],
    difficulty: "advanced",
    starterCode: `use std::marker::PhantomData;

// Newtype pattern — prevent mixing up similar types
struct Meters(f64);
struct Kilograms(f64);

// Builder pattern
struct QueryBuilder {
    table: String,
    conditions: Vec<String>,
    limit: Option<usize>,
}

impl QueryBuilder {
    fn new(table: &str) -> Self {
        QueryBuilder { table: table.to_string(), conditions: Vec::new(), limit: None }
    }
    fn where_clause(mut self, condition: &str) -> Self {
        self.conditions.push(condition.to_string());
        self
    }
    fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }
    fn build(self) -> String {
        let mut q = format!("SELECT * FROM {}", self.table);
        if !self.conditions.is_empty() {
            q.push_str(&format!(" WHERE {}", self.conditions.join(" AND ")));
        }
        if let Some(n) = self.limit {
            q.push_str(&format!(" LIMIT {}", n));
        }
        q
    }
}

fn main() {
    let m = Meters(100.0);
    let kg = Kilograms(70.0);
    println!("Distance: {} m, Weight: {} kg", m.0, kg.0);
    
    let query = QueryBuilder::new("users")
        .where_clause("age > 18")
        .where_clause("active = true")
        .limit(10)
        .build();
    
    println!("Query: {}", query);
}`,
    solutionHint: "Builder methods take `self` (not `&mut self`) and return `Self` to enable chaining.",
  },

  // ── DSA ──────────────────────────────────────────────────────────────────
  {
    id: 21,
    slug: "dsa-arrays-strings",
    title: "DSA: Arrays & Strings",
    track: "dsa",
    description:
      "Two pointers, sliding window, prefix sum, Kadane's algorithm, and anagram detection.",
    concepts: ["two pointers", "sliding window", "prefix sum", "Kadane's", "anagrams"],
    difficulty: "intermediate",
    starterCode: `fn main() {
    // Two Sum — find two indices that add to target
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    println!("Two Sum: {:?}", two_sum(&nums, target));
    
    // Max subarray sum (Kadane's algorithm)
    let arr = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Max subarray sum: {}", max_subarray(&arr));
    
    // Sliding window — longest substring without repeating chars
    let s = "abcabcbb";
    println!("Longest unique: {}", longest_unique(s));
}

fn two_sum(nums: &[i32], target: i32) -> Vec<usize> {
    use std::collections::HashMap;
    let mut seen: HashMap<i32, usize> = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        if let Some(&j) = seen.get(&(target - n)) {
            return vec![j, i];
        }
        seen.insert(n, i);
    }
    vec![]
}

fn max_subarray(nums: &[i32]) -> i32 {
    // TODO: Implement Kadane's algorithm
    // Track current_sum and max_sum
    let mut max_sum = nums[0];
    let mut cur = nums[0];
    for &n in &nums[1..] {
        cur = n.max(cur + n);
        max_sum = max_sum.max(cur);
    }
    max_sum
}

fn longest_unique(s: &str) -> usize {
    // TODO: Sliding window with a HashSet
    use std::collections::HashSet;
    let chars: Vec<char> = s.chars().collect();
    let mut set = HashSet::new();
    let mut left = 0;
    let mut max = 0;
    for right in 0..chars.len() {
        while set.contains(&chars[right]) {
            set.remove(&chars[left]);
            left += 1;
        }
        set.insert(chars[right]);
        max = max.max(right - left + 1);
    }
    max
}`,
    solutionHint: "Kadane's: `cur = n.max(cur + n); max_sum = max_sum.max(cur);`",
  },
  {
    id: 22,
    slug: "dsa-linked-lists",
    title: "DSA: Linked Lists",
    track: "dsa",
    description:
      "Build a singly linked list, reverse it, merge sorted lists, and detect cycles.",
    concepts: ["linked list", "reverse", "merge sorted", "cycle detection", "two pointers"],
    difficulty: "intermediate",
    starterCode: `type Link = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    val: i32,
    next: Link,
}

struct LinkedList {
    head: Link,
}

impl LinkedList {
    fn new() -> Self { LinkedList { head: None } }
    
    fn push_front(&mut self, val: i32) {
        let node = Box::new(Node { val, next: self.head.take() });
        self.head = Some(node);
    }
    
    fn to_vec(&self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = &self.head;
        while let Some(node) = current {
            result.push(node.val);
            current = &node.next;
        }
        result
    }
    
    fn reverse(mut head: Link) -> Link {
        let mut prev: Link = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

fn main() {
    let mut list = LinkedList::new();
    for i in (1..=5).rev() { list.push_front(i); }
    println!("Original: {:?}", list.to_vec());
    
    list.head = LinkedList::reverse(list.head);
    println!("Reversed: {:?}", list.to_vec());
}`,
    solutionHint: "Reverse iteratively: keep `prev` and `current`, rewire next pointers.",
  },
  {
    id: 23,
    slug: "dsa-stacks-queues",
    title: "DSA: Stacks, Queues & Heaps",
    track: "dsa",
    description:
      "Valid parentheses, monotonic stack, BinaryHeap usage, and top-k problems.",
    concepts: ["stack", "queue", "BinaryHeap", "monotonic stack", "top-k"],
    difficulty: "intermediate",
    starterCode: `use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    // Valid Parentheses
    let tests = ["()", "()[]{}", "(]", "([)]", "{[]}"];
    for t in tests {
        println!("{:?} -> {}", t, is_valid(t));
    }
    
    // Top-K frequent elements
    let nums = vec![1,1,1,2,2,3];
    println!("Top 2: {:?}", top_k_frequent(&nums, 2));
}

fn is_valid(s: &str) -> bool {
    let mut stack = Vec::new();
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => if stack.pop() != Some('(') { return false; },
            ']' => if stack.pop() != Some('[') { return false; },
            '}' => if stack.pop() != Some('{') { return false; },
            _ => {}
        }
    }
    stack.is_empty()
}

fn top_k_frequent(nums: &[i32], k: usize) -> Vec<i32> {
    use std::collections::HashMap;
    let mut freq: HashMap<i32, usize> = HashMap::new();
    for &n in nums { *freq.entry(n).or_insert(0) += 1; }
    
    // TODO: Use a min-heap of size k to find top k elements
    let mut heap: BinaryHeap<Reverse<(usize, i32)>> = BinaryHeap::new();
    for (&num, &count) in &freq {
        heap.push(Reverse((count, num)));
        if heap.len() > k { heap.pop(); }
    }
    heap.into_iter().map(|Reverse((_, n))| n).collect()
}`,
    solutionHint: "Use `BinaryHeap<Reverse<...>>` for a min-heap. Maintain size k.",
  },
  {
    id: 24,
    slug: "dsa-trees",
    title: "DSA: Trees & BSTs",
    track: "dsa",
    description:
      "In-order, pre-order, post-order traversals, balanced tree checks, BST operations, and serialize/deserialize.",
    concepts: ["BST", "traversals", "balanced check", "serialize", "recursion"],
    difficulty: "intermediate",
    starterCode: `type Tree = Option<Box<TreeNode>>;

#[derive(Debug)]
struct TreeNode { val: i32, left: Tree, right: Tree }

impl TreeNode {
    fn new(val: i32) -> Box<Self> {
        Box::new(TreeNode { val, left: None, right: None })
    }
}

fn inorder(root: &Tree) -> Vec<i32> {
    match root {
        None => vec![],
        Some(node) => {
            let mut result = inorder(&node.left);
            result.push(node.val);
            result.extend(inorder(&node.right));
            result
        }
    }
}

fn is_balanced(root: &Tree) -> bool {
    fn height(node: &Tree) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                let lh = height(&n.left);
                let rh = height(&n.right);
                if lh < 0 || rh < 0 || (lh - rh).abs() > 1 { -1 }
                else { 1 + lh.max(rh) }
            }
        }
    }
    height(root) >= 0
}

fn main() {
    //      4
    //    /   \\
    //   2     6
    //  / \\ / \\
    // 1  3 5  7
    let mut root = TreeNode::new(4);
    root.left = Some(TreeNode::new(2));
    root.right = Some(TreeNode::new(6));
    root.left.as_mut().unwrap().left = Some(TreeNode::new(1));
    root.left.as_mut().unwrap().right = Some(TreeNode::new(3));
    root.right.as_mut().unwrap().left = Some(TreeNode::new(5));
    root.right.as_mut().unwrap().right = Some(TreeNode::new(7));
    
    let tree = Some(root);
    println!("Inorder: {:?}", inorder(&tree));
    println!("Balanced: {}", is_balanced(&tree));
}`,
    solutionHint: "Inorder: left → root → right. Use recursive height(-1) sentinel for unbalanced detection.",
  },
  {
    id: 25,
    slug: "dsa-graphs",
    title: "DSA: Graphs",
    track: "dsa",
    description:
      "BFS, DFS, Dijkstra's shortest path, topological sort, and connected components.",
    concepts: ["BFS", "DFS", "Dijkstra", "topological sort", "adjacency list"],
    difficulty: "advanced",
    starterCode: `use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::Reverse;

fn bfs(graph: &HashMap<usize, Vec<usize>>, start: usize) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut order = Vec::new();
    
    visited.insert(start);
    queue.push_back(start);
    
    while let Some(node) = queue.pop_front() {
        order.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &next in neighbors {
                if visited.insert(next) {
                    queue.push_back(next);
                }
            }
        }
    }
    order
}

fn dijkstra(graph: &HashMap<usize, Vec<(usize, u32)>>, start: usize) -> HashMap<usize, u32> {
    let mut dist: HashMap<usize, u32> = HashMap::new();
    let mut heap = BinaryHeap::new();
    
    dist.insert(start, 0);
    heap.push(Reverse((0u32, start)));
    
    while let Some(Reverse((cost, node))) = heap.pop() {
        if cost > *dist.get(&node).unwrap_or(&u32::MAX) { continue; }
        if let Some(neighbors) = graph.get(&node) {
            for &(next, weight) in neighbors {
                let next_cost = cost + weight;
                if next_cost < *dist.get(&next).unwrap_or(&u32::MAX) {
                    dist.insert(next, next_cost);
                    heap.push(Reverse((next_cost, next)));
                }
            }
        }
    }
    dist
}

fn main() {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![3]);
    graph.insert(2, vec![3, 4]);
    graph.insert(3, vec![5]);
    graph.insert(4, vec![5]);
    
    println!("BFS from 0: {:?}", bfs(&graph, 0));
    
    let mut weighted: HashMap<usize, Vec<(usize, u32)>> = HashMap::new();
    weighted.insert(0, vec![(1, 4), (2, 1)]);
    weighted.insert(1, vec![(3, 1)]);
    weighted.insert(2, vec![(1, 2), (3, 5)]);
    weighted.insert(3, vec![]);
    
    let distances = dijkstra(&weighted, 0);
    let mut sorted: Vec<_> = distances.iter().collect();
    sorted.sort_by_key(|&(k, _)| k);
    for (node, dist) in sorted { println!("  {} -> {}", node, dist); }
}`,
    solutionHint: "Dijkstra uses a min-heap (BinaryHeap with Reverse). Always skip stale entries.",
  },
  {
    id: 26,
    slug: "dsa-sorting",
    title: "DSA: Sorting & Selection",
    track: "dsa",
    description:
      "Implement merge sort, quick sort, heap sort, counting sort, and the kth smallest element.",
    concepts: ["merge sort", "quick sort", "heap sort", "counting sort", "kth element"],
    difficulty: "intermediate",
    starterCode: `fn merge_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 { return arr; }
    let mid = arr.len() / 2;
    let left = merge_sort(arr[..mid].to_vec());
    let right = merge_sort(arr[mid..].to_vec());
    merge(left, right)
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] { result.push(left[i]); i += 1; }
        else { result.push(right[j]); j += 1; }
    }
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

fn quick_sort(arr: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let pivot = partition(arr, low, high);
        if pivot > 0 { quick_sort(arr, low, pivot - 1); }
        quick_sort(arr, pivot + 1, high);
    }
}

fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;
    for j in low..high {
        if arr[j] <= pivot { arr.swap(i, j); i += 1; }
    }
    arr.swap(i, high);
    i
}

fn main() {
    let data = vec![64, 34, 25, 12, 22, 11, 90];
    
    println!("Original: {:?}", data);
    
    let sorted = merge_sort(data.clone());
    println!("Merge sort: {:?}", sorted);
    
    let mut qs = data.clone();
    let len = qs.len();
    quick_sort(&mut qs, 0, len - 1);
    println!("Quick sort: {:?}", qs);
    
    // TODO: Implement counting sort for small non-negative integers
}`,
    solutionHint: "Counting sort: count occurrences, then reconstruct the sorted array from counts.",
  },
  {
    id: 27,
    slug: "dsa-dynamic-programming",
    title: "DSA: Dynamic Programming",
    track: "dsa",
    description:
      "Knapsack, LCS, coin change, edit distance, and common DP patterns.",
    concepts: ["knapsack", "LCS", "coin change", "edit distance", "memoization"],
    difficulty: "advanced",
    starterCode: `fn coin_change(coins: &[i32], amount: i32) -> i32 {
    let n = amount as usize + 1;
    let mut dp = vec![i32::MAX; n];
    dp[0] = 0;
    
    for i in 1..n {
        for &coin in coins {
            let c = coin as usize;
            if c <= i && dp[i - c] != i32::MAX {
                dp[i] = dp[i].min(dp[i - c] + 1);
            }
        }
    }
    
    if dp[amount as usize] == i32::MAX { -1 } else { dp[amount as usize] }
}

fn longest_common_subsequence(s1: &str, s2: &str) -> usize {
    let (m, n) = (s1.len(), s2.len());
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if s1[i-1] == s2[j-1] {
                dp[i-1][j-1] + 1
            } else {
                dp[i-1][j].max(dp[i][j-1])
            };
        }
    }
    dp[m][n]
}

fn edit_distance(s1: &str, s2: &str) -> usize {
    // TODO: Implement edit distance (Levenshtein)
    let (m, n) = (s1.len(), s2.len());
    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 0..=m { dp[i][0] = i; }
    for j in 0..=n { dp[0][j] = j; }
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if s1[i-1] == s2[j-1] { dp[i-1][j-1] }
            else { 1 + dp[i-1][j].min(dp[i][j-1]).min(dp[i-1][j-1]) };
        }
    }
    dp[m][n]
}

fn main() {
    println!("Coin change (11): {}", coin_change(&[1,5,6,9], 11));
    println!("LCS: {}", longest_common_subsequence("ABCBDAB", "BDCAB"));
    println!("Edit distance: {}", edit_distance("kitten", "sitting"));
}`,
    solutionHint: "Bottom-up DP: fill a 2D table where dp[i][j] depends on dp[i-1][j-1], dp[i-1][j], dp[i][j-1].",
  },
  {
    id: 28,
    slug: "dsa-interview-patterns",
    title: "DSA: Interview Patterns",
    track: "dsa",
    description:
      "Backtracking, binary search variants, two heaps pattern, and interval merging.",
    concepts: ["backtracking", "binary search", "two heaps", "intervals", "sliding window"],
    difficulty: "advanced",
    starterCode: `fn generate_permutations(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; nums.len()];
    
    fn backtrack(nums: &[i32], current: &mut Vec<i32>, used: &mut Vec<bool>, result: &mut Vec<Vec<i32>>) {
        if current.len() == nums.len() {
            result.push(current.clone());
            return;
        }
        for i in 0..nums.len() {
            if !used[i] {
                used[i] = true;
                current.push(nums[i]);
                backtrack(nums, current, used, result);
                current.pop();
                used[i] = false;
            }
        }
    }
    
    backtrack(&nums, &mut current, &mut used, &mut result);
    result
}

fn search_rotated(nums: &[i32], target: i32) -> i32 {
    let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);
    while lo <= hi {
        let mid = (lo + hi) / 2;
        if nums[mid as usize] == target { return mid; }
        if nums[lo as usize] <= nums[mid as usize] {
            if nums[lo as usize] <= target && target < nums[mid as usize] { hi = mid - 1; }
            else { lo = mid + 1; }
        } else {
            if nums[mid as usize] < target && target <= nums[hi as usize] { lo = mid + 1; }
            else { hi = mid - 1; }
        }
    }
    -1
}

fn merge_intervals(mut intervals: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    intervals.sort_by_key(|a| a[0]);
    let mut result: Vec<[i32; 2]> = Vec::new();
    for interval in intervals {
        if let Some(last) = result.last_mut() {
            if interval[0] <= last[1] { last[1] = last[1].max(interval[1]); continue; }
        }
        result.push(interval);
    }
    result
}

fn main() {
    let perms = generate_permutations(vec![1, 2, 3]);
    println!("Permutations of [1,2,3]: {} total", perms.len());
    
    let rotated = vec![4, 5, 6, 7, 0, 1, 2];
    println!("Search 0 in rotated: {}", search_rotated(&rotated, 0));
    
    let intervals = vec![[1,3],[2,6],[8,10],[15,18]];
    println!("Merged: {:?}", merge_intervals(intervals));
}`,
    solutionHint: "Backtracking: try each option, recurse, then undo (restore state).",
  },
  {
    id: 29,
    slug: "advanced-iterators",
    title: "Advanced Iterators & Pipelines",
    track: "dsa",
    description:
      "Custom iterators, lazy evaluation, combinator chains, and writing your own iterator types.",
    concepts: ["custom Iterator", "lazy evaluation", "combinators", "zip", "chain", "scan"],
    difficulty: "advanced",
    starterCode: `// A custom Fibonacci iterator
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self { Fibonacci { a: 0, b: 1 } }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(self.a) // yields 1, 1, 2, 3, 5, 8, ...
    }
}

// A range iterator that yields every Nth item
struct StepBy<I> { iter: I, step: usize, count: usize }

impl<I: Iterator> Iterator for StepBy<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        loop {
            let item = self.iter.next()?;
            if self.count % self.step == 0 { self.count += 1; return Some(item); }
            self.count += 1;
        }
    }
}

fn main() {
    // First 10 Fibonacci numbers
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("Fibonacci: {:?}", fibs);
    
    // Sum of squares of even Fibonacci numbers under 100
    let result: u64 = Fibonacci::new()
        .take_while(|&x| x < 100)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    println!("Sum of squares of even Fibs < 100: {}", result);
    
    // Running totals with scan
    let running: Vec<u64> = Fibonacci::new()
        .take(7)
        .scan(0u64, |acc, x| { *acc += x; Some(*acc) })
        .collect();
    println!("Running totals: {:?}", running);
}`,
    solutionHint: "Implement `Iterator` trait with `type Item` and `fn next(&mut self) -> Option<Self::Item>`.",
  },
  {
    id: 30,
    slug: "dsa-capstone",
    title: "DSA Capstone: Interview Study Planner",
    track: "dsa",
    description:
      "Combine topological sort, BFS, heap ranking, DP, and Trie prefix search into a study planner.",
    concepts: ["topological sort", "BFS", "priority queue", "0/1 knapsack", "Trie"],
    difficulty: "advanced",
    starterCode: `use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::Reverse;

#[derive(Debug, Clone)]
struct Topic { name: &'static str, hours: usize, value: usize }

fn topological_sort(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    let mut indegree = vec![0usize; n];
    for &(from, to) in edges { adj[from].push(to); indegree[to] += 1; }
    let mut queue: VecDeque<usize> = (0..n).filter(|&i| indegree[i] == 0).collect();
    let mut order = Vec::new();
    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &next in &adj[node] {
            indegree[next] -= 1;
            if indegree[next] == 0 { queue.push_back(next); }
        }
    }
    if order.len() == n { Some(order) } else { None }
}

fn best_revision_plan(topics: &[Topic], budget: usize) -> (usize, Vec<&'static str>) {
    let n = topics.len();
    let mut dp = vec![vec![0usize; budget + 1]; n + 1];
    let mut take = vec![vec![false; budget + 1]; n + 1];
    for i in 1..=n {
        let t = &topics[i-1];
        for cap in 0..=budget {
            dp[i][cap] = dp[i-1][cap];
            if t.hours <= cap {
                let candidate = dp[i-1][cap - t.hours] + t.value;
                if candidate > dp[i][cap] { dp[i][cap] = candidate; take[i][cap] = true; }
            }
        }
    }
    let mut chosen = Vec::new();
    let mut cap = budget;
    for i in (1..=n).rev() {
        if take[i][cap] { chosen.push(topics[i-1].name); cap -= topics[i-1].hours; }
    }
    (dp[n][budget], chosen)
}

fn main() {
    let topics = vec![
        Topic { name: "ownership",    hours: 2, value: 9 },
        Topic { name: "borrowing",    hours: 2, value: 8 },
        Topic { name: "collections",  hours: 3, value: 8 },
        Topic { name: "trees",        hours: 4, value: 10 },
        Topic { name: "graphs",       hours: 5, value: 10 },
        Topic { name: "dynamic-prog", hours: 5, value: 10 },
    ];
    let prereqs: Vec<(usize, usize)> = vec![(0,1),(0,2),(2,3),(3,4),(2,5)];
    
    match topological_sort(topics.len(), &prereqs) {
        Some(order) => println!("Learning order: {:?}", order.iter().map(|&i| topics[i].name).collect::<Vec<_>>()),
        None => println!("Cycle detected!"),
    }
    
    let (score, mut bundle) = best_revision_plan(&topics, 8);
    bundle.sort();
    println!("Best 8h revision: {:?} (score: {})", bundle, score);
}`,
    solutionHint: "Topological sort uses Kahn's algorithm (indegree + BFS). DP uses 0/1 knapsack table.",
  },

  // ── PROJECTS ─────────────────────────────────────────────────────────────
  {
    id: 31,
    slug: "serde-serialization",
    title: "Serde Serialization",
    track: "projects",
    description:
      "Serialize and deserialize JSON, TOML, with field attributes, enum tagging, and custom serializers using Serde.",
    concepts: ["serde", "serde_json", "derive macros", "field attributes", "enum tagging"],
    difficulty: "intermediate",
    starterCode: `// Note: In a real Cargo project, add to Cargo.toml:
// serde = { version = "1", features = ["derive"] }
// serde_json = "1"

// Serde lets you convert Rust structs to/from JSON, TOML, etc.
// This example shows the patterns without the external crate.

// With serde, you would write:
// #[derive(Serialize, Deserialize, Debug)]
// struct User {
//     #[serde(rename = "user_id")]
//     id: u32,
//     name: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     email: Option<String>,
// }

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    email: Option<String>,
}

impl User {
    // Manual JSON serialization (what serde does automatically)
    fn to_json(&self) -> String {
        let email = match &self.email {
            Some(e) => format!(r#","email":"{}""#, e),
            None => String::new(),
        };
        format!(r#"{{"user_id":{},"name":"{}"{}}}"#, self.id, self.name, email)
    }
}

fn main() {
    let user = User { id: 1, name: "Alice".to_string(), email: Some("alice@example.com".to_string()) };
    let user2 = User { id: 2, name: "Bob".to_string(), email: None };
    
    println!("{}", user.to_json());
    println!("{}", user2.to_json());
    
    // In production Rust, serde_json::to_string(&user) handles all of this!
    println!("\\nWith serde_json, it's just: serde_json::to_string(&user)?");
}`,
    solutionHint: "Add `#[derive(Serialize, Deserialize)]` and use `serde_json::to_string(&value)?`",
  },
  {
    id: 32,
    slug: "web-api-actix",
    title: "Web API with Actix-Web",
    track: "projects",
    description:
      "Build REST endpoints, CRUD operations, middleware, shared state, and JSON responses with Actix-Web.",
    concepts: ["actix-web", "REST", "CRUD", "middleware", "shared state", "JSON"],
    difficulty: "advanced",
    starterCode: `// Note: In a real Cargo project, add to Cargo.toml:
// actix-web = "4"
// serde = { version = "1", features = ["derive"] }
// tokio = { version = "1", features = ["full"] }

// This shows the Actix-Web patterns you'd use in production.

// Route handlers look like this:
// #[get("/users/{id}")]
// async fn get_user(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
//     let id = path.into_inner();
//     match data.users.lock().unwrap().get(&id) {
//         Some(user) => HttpResponse::Ok().json(user),
//         None => HttpResponse::NotFound().finish(),
//     }
// }

// State shared across requests:
// struct AppState {
//     users: Mutex<HashMap<u32, User>>,
// }

// Server setup:
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let state = web::Data::new(AppState { ... });
//     HttpServer::new(move || {
//         App::new()
//             .app_data(state.clone())
//             .service(get_user)
//             .service(create_user)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

// For now, let's simulate the logic without the framework:
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct User { id: u32, name: String, email: String }

struct InMemoryDb { users: HashMap<u32, User>, next_id: u32 }

impl InMemoryDb {
    fn new() -> Self { InMemoryDb { users: HashMap::new(), next_id: 1 } }
    fn create(&mut self, name: &str, email: &str) -> &User {
        let id = self.next_id;
        self.users.insert(id, User { id, name: name.to_string(), email: email.to_string() });
        self.next_id += 1;
        self.users.get(&(id)).unwrap()
    }
    fn get(&self, id: u32) -> Option<&User> { self.users.get(&id) }
    fn list(&self) -> Vec<&User> { self.users.values().collect() }
    fn delete(&mut self, id: u32) -> bool { self.users.remove(&id).is_some() }
}

fn main() {
    let mut db = InMemoryDb::new();
    let u1 = db.create("Alice", "alice@example.com");
    println!("Created: {:?}", u1);
    let u2 = db.create("Bob", "bob@example.com");
    println!("Created: {:?}", u2);
    
    println!("GET /users/1: {:?}", db.get(1));
    println!("GET /users: {} users", db.list().len());
    
    db.delete(1);
    println!("After DELETE /users/1: {} users", db.list().len());
}`,
    solutionHint: "In production, use `HttpResponse::Ok().json(data)` and `web::Data<Mutex<State>>` for shared state.",
  },
  {
    id: 33,
    slug: "cli-apps-clap",
    title: "CLI Apps with Clap",
    track: "projects",
    description:
      "Build professional CLI applications with Clap's derive macros, subcommands, colored output, and progress bars.",
    concepts: ["clap", "CLI", "subcommands", "derive macros", "colored output"],
    difficulty: "intermediate",
    starterCode: `// Note: In a real project, add to Cargo.toml:
// clap = { version = "4", features = ["derive"] }

// With Clap, your CLI looks like:
// #[derive(Parser)]
// #[command(name = "mytool", about = "A CLI tool")]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }
//
// #[derive(Subcommand)]
// enum Commands {
//     Add { name: String, #[arg(short, long, default_value = "medium")] priority: String },
//     List { #[arg(short, long)] all: bool },
//     Done { id: u32 },
// }

// Let's simulate a CLI without the crate:
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Simulate: tool add "My Task" --priority high
    //           tool list
    //           tool done 1
    
    let command = args.get(1).map(String::as_str).unwrap_or("help");
    
    match command {
        "add" => {
            let title = args.get(2).map(String::as_str).unwrap_or("Untitled");
            let priority = args.windows(2)
                .find(|w| w[0] == "--priority" || w[0] == "-p")
                .map(|w| w[1].as_str())
                .unwrap_or("medium");
            println!("[ADD] Task: '{}' (priority: {})", title, priority);
        }
        "list" => println!("[LIST] Showing all tasks"),
        "done" => {
            let id: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
            println!("[DONE] Marked task {} as complete", id);
        }
        "help" | _ => {
            println!("Usage: tool <command> [args]");
            println!("  add <title> [--priority low|medium|high]");
            println!("  list");
            println!("  done <id>");
        }
    }
}`,
    solutionHint: "With Clap derive: `#[derive(Parser)]` on your struct, `#[derive(Subcommand)]` on enum.",
  },
  {
    id: 34,
    slug: "database-sqlite",
    title: "Database with SQLite",
    track: "projects",
    description:
      "Use rusqlite for SQLite database access, repository pattern, transactions, and parameterized queries.",
    concepts: ["rusqlite", "SQLite", "repository pattern", "transactions", "prepared statements"],
    difficulty: "advanced",
    starterCode: `// Note: In a real project, add to Cargo.toml:
// rusqlite = { version = "0.31", features = ["bundled"] }

// With rusqlite, database access looks like:
// let conn = Connection::open("app.db")?;
// conn.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)", [])?;
// conn.execute("INSERT INTO users (name, email) VALUES (?1, ?2)", params![name, email])?;
// let mut stmt = conn.prepare("SELECT id, name, email FROM users")?;
// let users = stmt.query_map([], |row| {
//     Ok(User { id: row.get(0)?, name: row.get(1)?, email: row.get(2)? })
// })?;

// Repository pattern simulation:
use std::collections::HashMap;

trait Repository<T> {
    fn find(&self, id: u32) -> Option<&T>;
    fn save(&mut self, item: T) -> u32;
    fn delete(&mut self, id: u32) -> bool;
    fn all(&self) -> Vec<&T>;
}

#[derive(Debug, Clone)]
struct User { id: u32, name: String, email: String }

struct UserRepository { store: HashMap<u32, User>, next_id: u32 }

impl UserRepository {
    fn new() -> Self { UserRepository { store: HashMap::new(), next_id: 1 } }
    fn find_by_email(&self, email: &str) -> Option<&User> {
        self.store.values().find(|u| u.email == email)
    }
}

impl Repository<User> for UserRepository {
    fn find(&self, id: u32) -> Option<&User> { self.store.get(&id) }
    fn save(&mut self, mut item: User) -> u32 {
        let id = self.next_id; item.id = id;
        self.store.insert(id, item); self.next_id += 1; id
    }
    fn delete(&mut self, id: u32) -> bool { self.store.remove(&id).is_some() }
    fn all(&self) -> Vec<&User> { self.store.values().collect() }
}

fn main() {
    let mut repo = UserRepository::new();
    
    repo.save(User { id: 0, name: "Alice".into(), email: "alice@example.com".into() });
    repo.save(User { id: 0, name: "Bob".into(), email: "bob@example.com".into() });
    
    println!("All users: {} found", repo.all().len());
    println!("Find by email: {:?}", repo.find_by_email("alice@example.com"));
    println!("Find by id: {:?}", repo.find(1));
    
    repo.delete(1);
    println!("After delete: {} users", repo.all().len());
}`,
    solutionHint: "Use `params![val1, val2]` for parameterized queries to prevent SQL injection.",
  },
  {
    id: 35,
    slug: "testing-rust",
    title: "Testing in Rust",
    track: "projects",
    description:
      "Unit tests, assertion macros, panic tests, parameterized tests, and TDD in Rust.",
    concepts: ["#[test]", "assert!", "assert_eq!", "should_panic", "test modules", "TDD"],
    difficulty: "intermediate",
    starterCode: `// Rust testing is built-in — no external framework needed!

fn add(a: i32, b: i32) -> i32 { a + b }
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 { Err("Division by zero".into()) }
    else { Ok(a / b) }
}
fn is_palindrome(s: &str) -> bool {
    let clean: String = s.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_lowercase().next().unwrap()).collect();
    clean == clean.chars().rev().collect::<String>()
}

// TODO: Add more implementations and test them

fn fibonacci(n: u32) -> u64 {
    match n { 0 => 0, 1 => 1, _ => fibonacci(n-1) + fibonacci(n-2) }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }
    
    #[test]
    fn test_divide_ok() {
        assert!((divide(10.0, 2.0).unwrap() - 5.0).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10.0, 0.0).is_err());
        assert_eq!(divide(10.0, 0.0).unwrap_err(), "Division by zero");
    }
    
    #[test]
    fn test_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(!is_palindrome("hello"));
    }
    
    #[test]
    fn test_fibonacci() {
        let cases = [(0, 0), (1, 1), (2, 1), (5, 5), (10, 55)];
        for (n, expected) in cases {
            assert_eq!(fibonacci(n), expected, "fibonacci({}) failed", n);
        }
    }
    
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_out_of_bounds() {
        let v = vec![1, 2, 3];
        let _ = v[10]; // Should panic
    }
}

fn main() {
    println!("Run with: rustc --test main.rs && ./main");
    println!("Or in a Cargo project: cargo test");
    println!("\\nFunctions work correctly:");
    println!("add(2, 3) = {}", add(2, 3));
    println!("divide(10, 2) = {:?}", divide(10.0, 2.0));
    println!("is_palindrome('racecar') = {}", is_palindrome("racecar"));
}`,
    solutionHint: "Use `#[cfg(test)] mod tests { use super::*; }` to organize tests. Run with `cargo test`.",
  },
  {
    id: 36,
    slug: "rest-api-capstone",
    title: "REST API Capstone",
    track: "projects",
    description:
      "A full production REST API combining Actix-Web, SQLite, Serde, input validation, and structured logging.",
    concepts: ["actix-web", "SQLite", "serde", "validation", "logging", "error handling"],
    difficulty: "advanced",
    starterCode: `// Full-stack REST API capstone project!
// In a real Cargo project, your Cargo.toml would include:
// actix-web = "4"
// rusqlite = { version = "0.31", features = ["bundled"] }
// serde = { version = "1", features = ["derive"] }
// tokio = { version = "1", features = ["full"] }
// uuid = { version = "1", features = ["v4"] }
// chrono = { version = "0.4", features = ["serde"] }

// This capstone demonstrates the full architecture pattern:

// src/models.rs
mod models {
    #[derive(Debug, Clone)]
    pub struct User { pub id: String, pub name: String, pub email: String, pub created_at: String }
    #[derive(Debug, Clone)]
    pub struct Post { pub id: String, pub user_id: String, pub title: String, pub body: String }
}

// src/repository.rs  
mod repository {
    use super::models::{User, Post};
    use std::collections::HashMap;
    
    pub struct InMemoryStore {
        pub users: HashMap<String, User>,
        pub posts: HashMap<String, Post>,
    }
    
    impl InMemoryStore {
        pub fn new() -> Self { InMemoryStore { users: HashMap::new(), posts: HashMap::new() } }
    }
}

// src/handlers.rs (Actix-Web handlers)
// async fn create_user(body: web::Json<CreateUserRequest>, state: web::Data<AppState>) -> impl Responder
// async fn get_user(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder
// async fn list_posts(query: web::Query<ListQuery>, state: web::Data<AppState>) -> impl Responder

// src/validation.rs
fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 5
}

fn validate_name(name: &str) -> Result<(), String> {
    if name.trim().is_empty() { return Err("Name cannot be empty".into()); }
    if name.len() > 100 { return Err("Name too long (max 100 chars)".into()); }
    Ok(())
}

use models::{User, Post};
use repository::InMemoryStore;

fn main() {
    println!("=== REST API Capstone Architecture ===");
    println!();
    println!("Routes:");
    println!("  POST   /api/users          Create user");
    println!("  GET    /api/users/:id      Get user by ID");
    println!("  GET    /api/users          List all users");
    println!("  DELETE /api/users/:id      Delete user");
    println!("  POST   /api/posts          Create post");
    println!("  GET    /api/posts          List posts");
    println!("  GET    /api/posts/:id      Get post");
    println!();
    
    // Demonstrate validation
    let test_emails = ["alice@example.com", "invalid", "no-at.com"];
    for email in test_emails {
        println!("validate_email({:?}) -> {}", email, validate_email(email));
    }
    
    let test_names = ["Alice", "", "a".repeat(200).as_str()];
    for name in test_names {
        match validate_name(name) {
            Ok(()) => println!("validate_name({:?}) -> Ok", &name[..name.len().min(20)]),
            Err(e) => println!("validate_name({:?}) -> Err({})", &name[..name.len().min(20)], e),
        }
    }
    
    println!("\\n=== Step 36 Complete! You've finished the Rust learning track! ===");
}`,
    solutionHint: "Structure: models → repository → handlers → server. Use shared `web::Data<Mutex<State>>`.",
  },
];

export function getLessonBySlug(slug: string): Lesson | undefined {
  return LESSONS.find((l) => l.slug === slug);
}

export function getLessonById(id: number): Lesson | undefined {
  return LESSONS.find((l) => l.id === id);
}

export function getLessonsByTrack(track: Track): Lesson[] {
  return LESSONS.filter((l) => l.track === track);
}

export function getTrackProgress(track: Track, completedIds: number[]): number {
  const lessons = getLessonsByTrack(track);
  if (lessons.length === 0) return 0;
  const done = lessons.filter((l) => completedIds.includes(l.id)).length;
  return Math.round((done / lessons.length) * 100);
}

export function getNextLesson(currentId: number): Lesson | undefined {
  const idx = LESSONS.findIndex((l) => l.id === currentId);
  return LESSONS[idx + 1];
}

export function getPrevLesson(currentId: number): Lesson | undefined {
  const idx = LESSONS.findIndex((l) => l.id === currentId);
  return idx > 0 ? LESSONS[idx - 1] : undefined;
}
