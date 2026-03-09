// ============================================================
// STEP 05: Structs & Enums
// ============================================================
// Run: rustc main.rs && ./main
//
// Structs and enums are the building blocks of Rust data modeling.
// - Structs group related data together (like classes without inheritance)
// - Enums represent a type that can be one of several variants
// ============================================================

// --- 5.1 Defining Structs ---

// A struct groups named fields together.
#[derive(Debug)] // Allows printing with {:?}
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

// Tuple struct — fields have no names, just types.
#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Point(f64, f64);

// Unit struct — no fields at all. Useful for markers/traits.
struct _AlwaysEqual;

// --- 5.2 Implementing Methods ---

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// `impl` block defines methods on a struct.
impl Rectangle {
    // Method: takes `&self` as first parameter
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }

    // Method that takes ownership of self (rare, consumes the struct)
    fn describe(self) -> String {
        format!("Rectangle({}x{})", self.width, self.height)
    }

    // Associated function (no `self`) — like a static method.
    // Called with `Rectangle::square(5.0)`, not `rect.square(5.0)`.
    fn square(size: f64) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height } // field init shorthand
    }
}

// --- 5.3 Enums ---

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Enums can hold data in each variant!
#[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Enums with different data types per variant:
#[derive(Debug)]
enum Message {
    Quit,                        // no data
    Echo(String),                // single String
    Move { x: i32, y: i32 },    // named fields (like a struct)
    Color(u8, u8, u8),           // tuple-like
}

// Enums can have methods too!
impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("  Quitting..."),
            Message::Echo(text) => println!("  Echo: {}", text),
            Message::Move { x, y } => println!("  Moving to ({}, {})", x, y),
            Message::Color(r, g, b) => println!("  Color: rgb({}, {}, {})", r, g, b),
        }
    }
}

// --- 5.4 The Option Enum (Rust's Null Replacement) ---

// Rust has NO null. Instead, it uses `Option<T>`:
//   enum Option<T> {
//       Some(T),
//       None,
//   }
// This forces you to handle the "no value" case explicitly.

fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some(String::from("Alice")),
        2 => Some(String::from("Bob")),
        _ => None,
    }
}

fn main() {
    // --- Using Structs ---

    // Create an instance:
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        age: 30,
        active: true,
    };
    println!("User: {:?}", user1);
    println!("Username: {}", user1.username);

    // Mutable struct — all fields become mutable:
    let mut user2 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        age: 25,
        active: true,
    };
    user2.age = 26; // mutate a field
    println!("Bob's new age: {}", user2.age);

    // Struct update syntax — copy remaining fields from another instance:
    let user3 = User {
        username: String::from("charlie"),
        email: String::from("charlie@example.com"),
        ..user2 // take `age` and `active` from user2
    };
    println!("User3: {:?}", user3);

    // Tuple structs:
    let red = Color(255, 0, 0);
    let origin = Point(0.0, 0.0);
    println!("Color: {:?}, Point: {:?}", red, origin);
    println!("Red value: {}", red.0);

    // --- Using Methods ---

    let rect = Rectangle::new(10.0, 5.0);
    println!("\nRectangle: {:?}", rect);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Is square? {}", rect.is_square());

    let square = Rectangle::square(7.0);
    println!("Square: {:?}", square);
    println!("Square area: {}", square.area());
    println!("Is square? {}", square.is_square());

    // `.describe()` takes ownership, so `square` is consumed:
    let desc = square.describe();
    println!("{}", desc);
    // println!("{:?}", square); // ERROR: square was moved

    // --- Using Enums ---

    let dir = Direction::North;
    println!("\nDirection: {:?}", dir);

    let home = IpAddress::V4(127, 0, 0, 1);
    let loopback = IpAddress::V6(String::from("::1"));
    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);

    // Enums with methods:
    println!("\nProcessing messages:");
    let messages = vec![
        Message::Echo(String::from("hello")),
        Message::Move { x: 10, y: 20 },
        Message::Color(255, 128, 0),
        Message::Quit,
    ];
    for msg in &messages {
        msg.process();
    }

    // --- Using Option ---

    println!("\nLooking up users:");
    for id in 1..=3 {
        match find_user(id) {
            Some(name) => println!("  Found user {}: {}", id, name),
            None => println!("  User {} not found", id),
        }
    }

    // Handy Option methods:
    let maybe_number: Option<i32> = Some(42);
    println!("unwrap_or: {}", maybe_number.unwrap_or(0));   // 42
    println!("is_some: {}", maybe_number.is_some());         // true

    let nothing: Option<i32> = None;
    println!("unwrap_or: {}", nothing.unwrap_or(0));         // 0
    println!("is_none: {}", nothing.is_none());              // true

    // `if let` — concise way to handle one variant:
    if let Some(name) = find_user(1) {
        println!("Found: {}", name);
    }

    println!("\n--- Step 05 Complete! ---");
    println!("Next: step_06 — Pattern Matching");
}

// ============================================================
// EXERCISES:
// 1. Create a `Circle` struct with a `radius` field. Add methods for
//    `area()` and `circumference()`. Add `Circle::new(radius)`.
// 2. Create an enum `Shape` with variants Circle(f64), Rectangle(f64, f64),
//    Triangle(f64, f64, f64). Add an `area()` method using match.
// 3. Write a function that takes Option<String> and prints the value
//    or "no value provided" if None.
// ============================================================
