// ============================================================
// STEP 09: Traits & Generics
// ============================================================
// Run: rustc main.rs && ./main
//
// Traits define shared behavior (like interfaces in other languages).
// Generics let you write code that works with many types.
// Together, they give Rust powerful polymorphism without inheritance.
// ============================================================

use std::fmt;

fn main() {
    // --- 9.1 Defining and Implementing Traits ---

    let circle = Circle { radius: 5.0 };
    let rect = Rect {
        width: 10.0,
        height: 3.0,
    };
    let triangle = Triangle {
        base: 6.0,
        height: 4.0,
    };

    println!("Circle:   area={:.2}, desc={}", circle.area(), circle.describe());
    println!("Rect:     area={:.2}, desc={}", rect.area(), rect.describe());
    println!("Triangle: area={:.2}, desc={}", triangle.area(), triangle.describe());

    // --- 9.2 Traits as Parameters ---

    print_shape_info(&circle);
    print_shape_info(&rect);
    print_shape_info(&triangle);

    // --- 9.3 Generic Functions ---

    println!("\n--- Generics ---");
    println!("Max i32: {}", max(10, 20));
    println!("Max f64: {}", max(3.14, 2.71));
    println!("Max str: {}", max("apple", "banana"));

    // --- 9.4 Generic Structs ---

    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 2.5 };
    println!("Int point: {:?}", int_point);
    println!("Float point: {:?}", float_point);

    let mixed = MixedPoint { x: 5, y: 3.14 };
    println!("Mixed: {:?}", mixed);

    // Using methods on generic structs:
    println!("Float distance from origin: {:.2}", float_point.distance_from_origin());

    // Mixup example:
    let p1 = MixedPoint { x: 1, y: 2.0 };
    let p2 = MixedPoint { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("Mixed up: {:?}", p3); // MixedPoint { x: 1, y: 'c' }

    // --- 9.5 Trait Bounds ---

    println!("\n--- Trait Bounds ---");
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("Largest number: {}", largest(&nums));

    let chars = vec!['a', 'z', 'm', 'b'];
    println!("Largest char: {}", largest(&chars));

    // --- 9.6 Multiple Trait Bounds ---

    print_and_debug(&42);
    print_and_debug(&"hello");

    // --- 9.7 Returning Traits (impl Trait) ---

    let shape = make_shape(true);
    println!("\nMade shape: area = {:.2}", shape.area());

    // --- 9.8 Trait Objects (Dynamic Dispatch) ---

    println!("\n--- Trait Objects (dyn) ---");
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rect {
            width: 4.0,
            height: 5.0,
        }),
        Box::new(Triangle {
            base: 6.0,
            height: 3.0,
        }),
    ];

    let mut total_area = 0.0;
    for shape in &shapes {
        println!("  {} area: {:.2}", shape.describe(), shape.area());
        total_area += shape.area();
    }
    println!("Total area: {:.2}", total_area);

    // --- 9.9 Common Standard Library Traits ---

    println!("\n--- Standard Traits ---");

    // Display trait (custom printing with {})
    let item = Product {
        name: String::from("Laptop"),
        price: 999.99,
    };
    println!("Display: {}", item);    // uses our Display impl
    println!("Debug: {:?}", item);     // uses derive(Debug)

    // Clone and Copy
    let a = String::from("hello");
    let b = a.clone(); // String implements Clone but not Copy
    println!("Cloned: {} {}", a, b);

    // PartialEq and Eq (comparison)
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 3, y: 4 };
    println!("p1 == p2: {}", p1 == p2); // true
    println!("p1 == p3: {}", p1 == p3); // false

    // PartialOrd and Ord (ordering)
    // Our Point doesn't have ordering, but built-in types do:
    println!("3 > 2: {}", 3_i32 > 2_i32);

    // Default trait
    let default_point: Point<i32> = Point::default();
    println!("Default point: {:?}", default_point); // Point { x: 0, y: 0 }

    // --- 9.10 Supertraits ---

    let person = Person {
        name: String::from("Alice"),
    };
    person.greet(); // from Greetable, requires Display

    // --- 9.11 Derive Macros (Auto-Implementing Traits) ---

    // #[derive(Debug, Clone, PartialEq)] auto-generates implementations.
    // Common derivable traits: Debug, Clone, Copy, PartialEq, Eq,
    //   PartialOrd, Ord, Hash, Default

    println!("\n--- Step 09 Complete! ---");
    println!("Next: step_10 — Lifetimes");
}

// ============================================================
// Trait Definitions and Implementations
// ============================================================

// --- 9.1 Basic Trait ---

trait Shape {
    fn area(&self) -> f64;

    // Default implementation — types can override or use as-is
    fn describe(&self) -> String {
        format!("Shape with area {:.2}", self.area())
    }
}

struct Circle {
    radius: f64,
}
struct Rect {
    width: f64,
    height: f64,
}
struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn describe(&self) -> String {
        format!("Circle(r={})", self.radius)
    }
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn describe(&self) -> String {
        format!("Rect({}x{})", self.width, self.height)
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
    // Uses default describe() implementation
}

// --- 9.2 Traits as Parameters ---

// `impl Trait` syntax (sugar for trait bounds):
fn print_shape_info(shape: &impl Shape) {
    println!("  Info: {} -> area {:.2}", shape.describe(), shape.area());
}

// Equivalent using trait bound syntax:
// fn print_shape_info<T: Shape>(shape: &T) { ... }

// --- 9.3 Generic Functions ---

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}

// --- 9.4 Generic Structs ---

#[derive(Debug, Clone, PartialEq, Default)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// Methods on generic structs:
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// Methods only available for specific types:
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> MixedPoint<T, U> {
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// --- 9.5 Trait Bounds ---

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut result = list[0];
    for &item in &list[1..] {
        if item > result {
            result = item;
        }
    }
    result
}

// --- 9.6 Multiple Trait Bounds ---

// where clause syntax (cleaner for multiple bounds):
fn print_and_debug<T>(item: &T)
where
    T: fmt::Display + fmt::Debug,
{
    println!("Display: {}, Debug: {:?}", item, item);
}

// --- 9.7 Returning impl Trait ---

fn make_shape(use_circle: bool) -> Box<dyn Shape> {
    if use_circle {
        Box::new(Circle { radius: 5.0 })
    } else {
        Box::new(Rect {
            width: 3.0,
            height: 4.0,
        })
    }
}

// --- 9.9 Display Trait Example ---

#[derive(Debug)]
struct Product {
    name: String,
    price: f64,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (${:.2})", self.name, self.price)
    }
}

// --- 9.10 Supertraits ---

// Greetable requires Display (supertrait)
trait Greetable: fmt::Display {
    fn greet(&self) {
        println!("Hello, {}!", self);
    }
}

struct Person {
    name: String,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Greetable for Person {}

// ============================================================
// EXERCISES:
// 1. Create a `Printable` trait with a `to_string_pretty()` method.
//    Implement it for Circle and Rect.
// 2. Write a generic function `sum<T>` that sums a slice of numbers.
//    (Hint: T needs Add + Copy + Default bounds)
// 3. Create a generic `Pair<T>` struct with `first` and `second`.
//    Add a method that returns the larger if T: PartialOrd.
// 4. Implement Display for Point<T> where T: Display.
// ============================================================
