// ============================================================
// STEP 19: Advanced Trait Patterns
// ============================================================
// Run: rustc main.rs && ./main
//
// Deep dive into traits: associated types, operator overloading,
// orphan rule workarounds, blanket implementations, and more.
// ============================================================

use std::fmt;
use std::ops::{Add, Deref, Mul};

fn main() {
    // =========================
    // 19.1 Associated Types vs Generics
    // =========================

    println!("--- Associated Types ---");

    // Associated types: ONE implementation per type.
    // Generics: MANY implementations per type.

    // Iterator uses an associated type (not generic):
    //   trait Iterator { type Item; fn next(&mut self) -> Option<Self::Item>; }
    // This means a type can only implement Iterator ONCE.

    let mut counter = Doubles::new(1, 5);
    while let Some(val) = counter.next() {
        print!("{} ", val);
    }
    println!();

    // With generics, a type can implement the trait multiple times:
    let p = Point { x: 1.0, y: 2.0 };
    let dist_f64: f64 = p.convert();
    let dist_i32: i32 = p.convert();
    println!("As f64: {}, As i32: {}", dist_f64, dist_i32);

    // =========================
    // 19.2 Operator Overloading
    // =========================

    println!("\n--- Operator Overloading ---");

    let v1 = Vec2 { x: 1.0, y: 2.0 };
    let v2 = Vec2 { x: 3.0, y: 4.0 };

    let sum = v1 + v2;
    println!("{} + {} = {}", v1, v2, sum);

    let scaled = v1 * 3.0;
    println!("{} * 3.0 = {}", v1, scaled);

    println!("v1 == v1? {}", v1 == v1);
    println!("v1 == v2? {}", v1 == v2);

    // Indexing
    let matrix = Matrix([[1, 2], [3, 4]]);
    println!("matrix[0][1] = {}", matrix[0][1]);

    // =========================
    // 19.3 Newtype Pattern for Orphan Rule
    // =========================

    println!("\n--- Newtype Pattern ---");

    // Can't impl Display for Vec<T> directly (orphan rule).
    // Wrap it in a newtype:
    let wrapped = Wrapper(vec!["hello", "world", "rust"]);
    println!("Display for Vec: {}", wrapped);

    // Newtype for units:
    let distance = Meters(100.0);
    let time = Seconds(9.58);
    let speed = distance.0 / time.0;
    println!("{} in {} = {:.2} m/s", distance, time, speed);

    // =========================
    // 19.4 Blanket Implementations
    // =========================

    println!("\n--- Blanket Implementations ---");

    // A blanket impl provides a default for ALL types meeting a bound.
    println!("{}", 42.to_pretty_string());
    println!("{}", "hello".to_pretty_string());
    println!("{}", 3.14.to_pretty_string());

    // =========================
    // 19.5 Supertraits
    // =========================

    println!("\n--- Supertraits ---");

    let user = User {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
    };
    print_info(&user);

    // =========================
    // 19.6 Dynamic Dispatch vs Static Dispatch
    // =========================

    println!("\n--- Static vs Dynamic Dispatch ---");

    // Static dispatch (monomorphization) — compiler generates code for each type
    static_dispatch(&42);
    static_dispatch(&"hello");

    // Dynamic dispatch (vtable) — resolved at runtime
    let items: Vec<Box<dyn fmt::Display>> = vec![
        Box::new(42),
        Box::new("hello"),
        Box::new(3.14),
    ];
    for item in &items {
        dynamic_dispatch(item.as_ref());
    }

    // =========================
    // 19.7 Object Safety
    // =========================

    println!("\n--- Object Safety ---");
    println!("A trait is object-safe if:");
    println!("  1. It doesn't return Self");
    println!("  2. It has no generic type parameters");
    println!("  3. All methods have a receiver (&self, &mut self, etc.)");
    println!("Non-object-safe traits can't be used as `dyn Trait`.");

    // Clone is NOT object-safe (returns Self), but we can work around it:
    let shapes: Vec<Box<dyn CloneableShape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Square { side: 3.0 }),
    ];
    for shape in &shapes {
        println!("  {} area: {:.2}", shape.name(), shape.area());
    }

    // =========================
    // 19.8 Trait Composition & Extension
    // =========================

    println!("\n--- Trait Composition ---");

    let doc = Document {
        title: String::from("Rust Guide"),
        content: String::from("Learn Rust step by step"),
    };
    process(&doc);

    // =========================
    // 19.9 The Deref and From/Into Patterns
    // =========================

    println!("\n--- From/Into ---");

    // From<T> automatically gives you Into<T>
    let email = Email::from("user@example.com".to_string());
    println!("Email: {}", email);

    // Into works because From is implemented:
    let email2: Email = "admin@example.com".to_string().into();
    println!("Email2: {}", email2);

    // Functions that accept Into<T> are very flexible:
    send_email("test@example.com".to_string());
    send_email(Email(String::from("direct@example.com")));

    // =========================
    // 19.10 Fully Qualified Syntax
    // =========================

    println!("\n--- Fully Qualified Syntax ---");

    let pilot = Human;
    pilot.fly();          // calls Human's fly
    Pilot::fly(&pilot);   // calls Pilot's fly
    Wizard::fly(&pilot);  // calls Wizard's fly

    // For associated functions (no self):
    println!("Human: {}", Human::name());
    println!("Pilot: {}", <Human as Pilot>::name());
    println!("Wizard: {}", <Human as Wizard>::name());

    println!("\n--- Step 19 Complete! ---");
}

// ============================================================
// Supporting Types and Implementations
// ============================================================

// --- 19.1 Associated Types ---

struct Doubles {
    current: i32,
    max: i32,
}

impl Doubles {
    fn new(start: i32, max: i32) -> Self {
        Doubles { current: start, max }
    }
}

impl Iterator for Doubles {
    type Item = i32; // associated type — fixed for this impl
    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.max {
            None
        } else {
            let val = self.current * 2;
            self.current += 1;
            Some(val)
        }
    }
}

// Generic trait — can implement multiple times for different T
trait ConvertTo<T> {
    fn convert(&self) -> T;
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl ConvertTo<f64> for Point {
    fn convert(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl ConvertTo<i32> for Point {
    fn convert(&self) -> i32 {
        (self.x * self.x + self.y * self.y).sqrt() as i32
    }
}

// --- 19.2 Operator Overloading ---

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f64) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON && (self.y - other.y).abs() < f64::EPSILON
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}

struct Matrix([[i32; 2]; 2]);

impl std::ops::Index<usize> for Matrix {
    type Output = [i32; 2];
    fn index(&self, index: usize) -> &[i32; 2] {
        &self.0[index]
    }
}

// --- 19.3 Newtype ---

struct Wrapper<T>(Vec<T>);

impl<T: fmt::Display> fmt::Display for Wrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items: Vec<String> = self.0.iter().map(|x| x.to_string()).collect();
        write!(f, "[{}]", items.join(", "))
    }
}

struct Meters(f64);
struct Seconds(f64);

impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}m", self.0)
    }
}
impl fmt::Display for Seconds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}s", self.0)
    }
}

// --- 19.4 Blanket Impl ---

trait PrettyString {
    fn to_pretty_string(&self) -> String;
}

// Blanket impl: ANY type that implements Display gets PrettyString for free
impl<T: fmt::Display> PrettyString for T {
    fn to_pretty_string(&self) -> String {
        format!(">> {} <<", self)
    }
}

// --- 19.5 Supertraits ---

trait Printable: fmt::Display + fmt::Debug {
    fn print_info(&self) {
        println!("Display: {}", self);
        println!("Debug:   {:?}", self);
    }
}

#[derive(Debug)]
struct User {
    name: String,
    email: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

impl Printable for User {}

fn print_info(item: &dyn Printable) {
    item.print_info();
}

// --- 19.6 Static vs Dynamic Dispatch ---

fn static_dispatch(item: &impl fmt::Display) {
    println!("  Static: {}", item);
}

fn dynamic_dispatch(item: &dyn fmt::Display) {
    println!("  Dynamic: {}", item);
}

// --- 19.7 Object Safety workaround ---

trait CloneableShape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle {
    radius: f64,
}
struct Square {
    side: f64,
}

impl CloneableShape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn name(&self) -> &str {
        "Circle"
    }
}

impl CloneableShape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn name(&self) -> &str {
        "Square"
    }
}

// --- 19.8 Trait Composition ---

trait Readable {
    fn read(&self) -> &str;
}

trait Titled {
    fn title(&self) -> &str;
}

// Require both traits:
trait Processable: Readable + Titled {}

struct Document {
    title: String,
    content: String,
}

impl Readable for Document {
    fn read(&self) -> &str {
        &self.content
    }
}

impl Titled for Document {
    fn title(&self) -> &str {
        &self.title
    }
}

impl Processable for Document {}

fn process(item: &dyn Processable) {
    println!("Processing '{}': {}", item.title(), item.read());
}

// --- 19.9 From/Into ---

#[derive(Debug)]
struct Email(String);

impl From<String> for Email {
    fn from(s: String) -> Self {
        Email(s)
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn send_email(to: impl Into<Email>) {
    let email: Email = to.into();
    println!("Sending to: {}", email);
}

// --- 19.10 Disambiguation ---

trait Pilot {
    fn fly(&self) {
        println!("  Pilot flying");
    }
    fn name() -> &'static str {
        "Pilot"
    }
}

trait Wizard {
    fn fly(&self) {
        println!("  Wizard flying on broom");
    }
    fn name() -> &'static str {
        "Wizard"
    }
}

struct Human;

impl Pilot for Human {}
impl Wizard for Human {}

impl Human {
    fn fly(&self) {
        println!("  Human waving arms");
    }
    fn name() -> &'static str {
        "Human"
    }
}
