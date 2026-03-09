// ============================================================
// STEP 12: Modules & Crates
// ============================================================
// Run: rustc main.rs && ./main
//
// Modules organize code within a crate. Crates are compilation units.
// - `mod` defines a module
// - `pub` makes items public
// - `use` brings items into scope
//
// NOTE: For a real project, use `cargo new myproject` to get a proper
// project structure with Cargo.toml, src/, tests/, etc.
// This file demonstrates module concepts in a single file.
// ============================================================

// =========================
// 12.1 Defining Modules
// =========================

// Modules create a namespace hierarchy.
mod math {
    // By default, everything is PRIVATE.
    // Use `pub` to make items accessible outside the module.

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    // Private function — only accessible within this module
    fn _secret_formula(x: i32) -> i32 {
        x * 42
    }

    // Nested module
    pub mod advanced {
        pub fn power(base: f64, exp: u32) -> f64 {
            let mut result = 1.0;
            for _ in 0..exp {
                result *= base;
            }
            result
        }

        pub fn factorial(n: u64) -> u64 {
            (1..=n).product()
        }
    }
}

// =========================
// 12.2 Structs in Modules
// =========================

mod shapes {
    #[derive(Debug)]
    pub struct Circle {
        pub radius: f64,     // public field
        center_x: f64,       // private field!
        center_y: f64,       // private field!
    }

    impl Circle {
        // Public constructor — needed because some fields are private
        pub fn new(radius: f64, x: f64, y: f64) -> Circle {
            Circle {
                radius,
                center_x: x,
                center_y: y,
            }
        }

        pub fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }

        pub fn center(&self) -> (f64, f64) {
            (self.center_x, self.center_y)
        }
    }

    // Public enum — all variants are public if the enum is pub
    #[derive(Debug)]
    pub enum Color {
        Red,
        Green,
        Blue,
        Custom(u8, u8, u8),
    }

    impl Color {
        pub fn to_rgb(&self) -> (u8, u8, u8) {
            match self {
                Color::Red => (255, 0, 0),
                Color::Green => (0, 255, 0),
                Color::Blue => (0, 0, 255),
                Color::Custom(r, g, b) => (*r, *g, *b),
            }
        }
    }
}

// =========================
// 12.3 The `use` Keyword
// =========================

// Bring items into scope to avoid long paths:
use math::advanced::factorial;
// You can rename with `as`:
use math::advanced::power as pow;

// Bring multiple items from the same module:
use shapes::{Circle, Color};

// =========================
// 12.4 Re-exporting with `pub use`
// =========================

mod geometry {
    mod internal {
        pub fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
            ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
        }
    }

    // Re-export so users don't need to know about `internal`
    pub use self::internal::distance;
}

// =========================
// 12.5 Module with Types and Traits
// =========================

mod animal {
    pub trait Speak {
        fn speak(&self) -> String;
        fn name(&self) -> &str;
    }

    pub struct Dog {
        pub name: String,
    }

    pub struct Cat {
        pub name: String,
    }

    impl Speak for Dog {
        fn speak(&self) -> String {
            format!("{} says: Woof!", self.name)
        }
        fn name(&self) -> &str {
            &self.name
        }
    }

    impl Speak for Cat {
        fn speak(&self) -> String {
            format!("{} says: Meow!", self.name)
        }
        fn name(&self) -> &str {
            &self.name
        }
    }

    // A function using the trait
    pub fn introduce(animal: &dyn Speak) {
        println!("Meet {}! {}", animal.name(), animal.speak());
    }
}

// =========================
// 12.6 Prelude Pattern
// =========================

// A common pattern: create a `prelude` module that re-exports commonly used items.
mod mylib {
    pub mod prelude {
        pub use super::config::Config;
        pub use super::logger::log;
    }

    pub mod config {
        #[derive(Debug)]
        pub struct Config {
            pub debug: bool,
            pub verbose: bool,
        }

        impl Config {
            pub fn default() -> Self {
                Config {
                    debug: false,
                    verbose: false,
                }
            }
        }
    }

    pub mod logger {
        pub fn log(msg: &str) {
            println!("[LOG] {}", msg);
        }
    }
}

// Import everything from the prelude:
use mylib::prelude::*;

fn main() {
    // --- 12.1 Using Module Functions ---

    println!("--- Module Functions ---");
    println!("5 + 3 = {}", math::add(5, 3));          // full path
    println!("5 - 3 = {}", math::subtract(5, 3));
    println!("2^10 = {}", pow(2.0, 10));               // using `use` alias
    println!("5! = {}", factorial(5));                   // using `use`

    // Can't call private function:
    // math::_secret_formula(5); // ERROR: private function

    // --- 12.2 Structs from Modules ---

    println!("\n--- Structs from Modules ---");
    let circle = Circle::new(5.0, 0.0, 0.0);
    println!("Circle: {:?}", circle);
    println!("Area: {:.2}", circle.area());
    println!("Center: {:?}", circle.center());
    println!("Radius: {}", circle.radius); // public field
    // println!("{}", circle.center_x); // ERROR: private field

    let color = Color::Custom(128, 64, 255);
    println!("Color: {:?} -> RGB {:?}", color, color.to_rgb());

    // --- 12.4 Re-exported Functions ---

    println!("\n--- Re-exports ---");
    let dist = geometry::distance(0.0, 0.0, 3.0, 4.0);
    println!("Distance: {}", dist);

    // --- 12.5 Traits from Modules ---

    println!("\n--- Traits from Modules ---");
    use animal::{Cat, Dog, Speak};
    let dog = Dog {
        name: String::from("Rex"),
    };
    let cat = Cat {
        name: String::from("Whiskers"),
    };
    animal::introduce(&dog);
    animal::introduce(&cat);
    println!("{}", dog.speak());

    // --- 12.6 Prelude Pattern ---

    println!("\n--- Prelude Pattern ---");
    let config = Config::default();
    log(&format!("Config: {:?}", config));

    // =========================
    // 12.7 Cargo Project Structure (Reference)
    // =========================

    println!("\n--- Cargo Project Structure ---");
    println!("To create a real project:");
    println!("  cargo new myproject        # binary");
    println!("  cargo new mylib --lib      # library");
    println!();
    println!("Typical structure:");
    println!("  myproject/");
    println!("  ├── Cargo.toml            # dependencies & metadata");
    println!("  ├── src/");
    println!("  │   ├── main.rs           # binary entry point");
    println!("  │   ├── lib.rs            # library root");
    println!("  │   ├── module_name.rs    # module in a file");
    println!("  │   └── module_name/      # module in a directory");
    println!("  │       ├── mod.rs        # module root");
    println!("  │       └── submod.rs     # submodule");
    println!("  ├── tests/                # integration tests");
    println!("  ├── benches/              # benchmarks");
    println!("  └── examples/             # example programs");

    println!("\n--- Step 12 Complete! ---");
    println!("Next: step_13 — Smart Pointers");
}

// ============================================================
// EXERCISES:
// 1. Create a module `string_utils` with functions: `capitalize`,
//    `reverse`, and `word_count`. Make them all public.
// 2. Create a module `bank` with a private `balance` field in an
//    `Account` struct. Provide public methods for deposit/withdraw.
// 3. Create a prelude module that re-exports the most useful items
//    from multiple submodules.
// 4. Run `cargo new my_first_project` and move some of this code
//    into a proper Cargo project with multiple files.
// ============================================================
