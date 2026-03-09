// ============================================================
// STEP 16: Macros
// ============================================================
// Run: rustc main.rs && ./main
//
// Macros are metaprogramming — code that writes code.
// Rust has two kinds:
// 1. Declarative macros (macro_rules!) — pattern matching on syntax
// 2. Procedural macros (#[derive], attribute macros, function-like)
//    — these require a separate crate, so we focus on declarative here.
//
// NOTE: In Rust, macros must be defined BEFORE they are used in the file.
// That's why all macro_rules! definitions come before main().
// ============================================================

// ============================================================
// Macro Definitions (must come before main)
// ============================================================

// --- 16.2 Simple macros ---

macro_rules! say_hello {
    () => {
        println!("Hello from a macro!");
    };
}

macro_rules! double {
    ($e:expr) => {
        $e * 2
    };
}

// Fragment types:
// $e:expr   — expression (5 + 3, foo(), etc.)
// $t:ty     — type (i32, String, Vec<u8>)
// $i:ident  — identifier (variable/function name)
// $p:pat    — pattern
// $s:stmt   — statement
// $b:block  — block { ... }
// $l:literal — literal value (42, "hello", true)
// $tt:tt    — single token tree (most flexible)

// --- 16.3 Multiple patterns ---

macro_rules! min {
    ($a:expr, $b:expr) => {
        if $a < $b { $a } else { $b }
    };
    ($a:expr, $b:expr, $($rest:expr),+) => {
        min!($a, min!($b, $($rest),+))
    };
}

// --- 16.4 Repetition ---

macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

// Repetition syntax:
// $(...)*  — zero or more
// $(...)+  — one or more
// $(...)?  — zero or one
// Separators: , ; etc. between repetitions

macro_rules! make_vec {
    ($($element:expr),* $(,)?) => {
        {
            let mut v = Vec::new();
            $(v.push($element);)*
            v
        }
    };
}

// --- 16.5 Debug value macro ---

macro_rules! dbg_val {
    ($e:expr) => {
        println!("  {} = {:?}", stringify!($e), $e);
        //                      ^^^^^^^^^^^^ converts the expression to a string
    };
}

// --- 16.6 Trait implementation macro ---

trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;
struct Bird;

macro_rules! impl_animal {
    ($type:ty, $sound:expr) => {
        impl Animal for $type {
            fn speak(&self) {
                println!("  {} says {}", stringify!($type), $sound);
            }
        }
    };
}

impl_animal!(Dog, "Woof!");
impl_animal!(Cat, "Meow!");
impl_animal!(Bird, "Tweet!");

// --- 16.7 Assert macro ---

macro_rules! assert_between {
    ($val:expr, $min:expr, $max:expr) => {
        assert!(
            $val >= $min && $val <= $max,
            "{} = {:?} is not between {:?} and {:?}",
            stringify!($val),
            $val,
            $min,
            $max
        );
    };
}

// --- 16.8 Variadic ---

macro_rules! sum {
    ($x:expr) => { $x };
    ($x:expr, $($rest:expr),+) => {
        $x + sum!($($rest),+)
    };
}

macro_rules! csv_print {
    ($($val:expr),* $(,)?) => {
        {
            let mut first = true;
            $(
                if !first { print!(", "); }
                print!("{}", $val);
                #[allow(unused_assignments)]
                { first = false; }
            )*
            println!();
        }
    };
}

// --- 16.9 Mini HTML DSL ---

macro_rules! html_inner {
    // String literal content
    ($text:literal) => {
        $text.to_string()
    };
    // Nested tags
    ($($tag:ident { $($content:tt)* })*) => {
        {
            let mut result = String::from("\n");
            $(
                result.push_str(&format!("  <{}>", stringify!($tag)));
                result.push_str(&html_inner!($($content)*));
                result.push_str(&format!("</{}>\n", stringify!($tag)));
            )*
            result
        }
    };
}

macro_rules! html {
    // Match tag with content
    ($($tag:ident { $($content:tt)* })*) => {
        {
            let mut result = String::new();
            $(
                result.push_str(&format!("<{}>", stringify!($tag)));
                result.push_str(&html_inner!($($content)*));
                result.push_str(&format!("</{}>\n", stringify!($tag)));
            )*
            result
        }
    };
}

// ============================================================
// Main — Using all the macros defined above
// ============================================================

fn main() {
    // =========================
    // 16.1 Why Macros?
    // =========================

    println!("--- Why Macros? ---");
    println!("Macros let you:");
    println!("  - Reduce boilerplate code");
    println!("  - Create domain-specific syntax");
    println!("  - Accept variable number of arguments");
    println!("  - Generate code at compile time\n");

    // You've already used macros: println!, vec!, format!, etc.

    // =========================
    // 16.2 Basic macro_rules!
    // =========================

    println!("--- Basic Macros ---");

    // Our simplest macro:
    say_hello!();

    // Macro with an expression:
    let x = double!(21);
    println!("double!(21) = {}", x);

    // Works with any expression:
    println!("double!(100 + 5) = {}", double!(100 + 5));

    // =========================
    // 16.3 Macros with Multiple Patterns
    // =========================

    println!("\n--- Multiple Patterns ---");

    // Macro that handles different argument counts:
    println!("min!(3, 7) = {}", min!(3, 7));
    println!("min!(5, 2, 8) = {}", min!(5, 2, 8));
    println!("min!(9, 3, 7, 1) = {}", min!(9, 3, 7, 1));

    // =========================
    // 16.4 Repetition in Macros
    // =========================

    println!("\n--- Repetition ---");

    // Create a HashMap with our macro:
    let colors = hashmap! {
        "red" => (255, 0, 0),
        "green" => (0, 255, 0),
        "blue" => (0, 0, 255),
    };
    println!("Colors: {:?}", colors);

    // Create a Vec with our macro:
    let nums = make_vec![1, 2, 3, 4, 5];
    println!("Vec: {:?}", nums);

    // =========================
    // 16.5 Expression-Based Macros
    // =========================

    println!("\n--- Expression Macros ---");

    // A debug-print macro that shows the expression and its value:
    let a = 5;
    let b = 10;
    dbg_val!(a + b);
    dbg_val!(a * b);
    dbg_val!(vec![1, 2, 3].len());

    // =========================
    // 16.6 Macro for Implementing Traits
    // =========================

    println!("\n--- Trait Implementation Macro ---");

    let dog = Dog;
    let cat = Cat;
    let bird = Bird;

    dog.speak();
    cat.speak();
    bird.speak();

    // =========================
    // 16.7 Macro for Creating Test Helpers
    // =========================

    println!("\n--- Assert Macros ---");

    // Custom assert with better messages:
    assert_between!(5, 1, 10);
    assert_between!(7.5, 0.0, 10.0);
    println!("  All assertions passed!");

    // =========================
    // 16.8 Variadic Function-Like Macros
    // =========================

    println!("\n--- Variadic Macros ---");

    // Sum any number of values:
    println!("sum!(1) = {}", sum!(1));
    println!("sum!(1, 2) = {}", sum!(1, 2));
    println!("sum!(1, 2, 3, 4, 5) = {}", sum!(1, 2, 3, 4, 5));

    // Comma-separated print:
    csv_print!("Alice", 30, true, 3.14);

    // =========================
    // 16.9 Macro for DSLs
    // =========================

    println!("\n--- Mini DSL ---");

    // A tiny HTML-like DSL:
    let page = html! {
        h1 { "Welcome" }
        p { "This is generated by a macro." }
        ul {
            li { "Item 1" }
            li { "Item 2" }
            li { "Item 3" }
        }
    };
    println!("{}", page);

    // =========================
    // 16.10 Procedural Macros (Reference)
    // =========================

    println!("--- Procedural Macros (Reference) ---");
    println!("Proc macros must be in a separate crate.");
    println!("Three kinds:");
    println!("  1. #[derive(MyTrait)]  — auto-implement traits");
    println!("  2. #[my_attribute]     — custom attributes");
    println!("  3. my_macro!(...)      — function-like proc macros");
    println!();
    println!("Popular proc macros you'll use:");
    println!("  #[derive(Debug, Clone, Serialize, Deserialize)]");
    println!("  #[tokio::main]");
    println!("  #[test]");

    println!("\n--- Step 16 Complete! ---");
    println!("Next: step_17 — Unsafe Rust");
}

// ============================================================
// EXERCISES:
// 1. Write a macro `max!` that works like our `min!` but returns
//    the maximum of 2+ values.
// 2. Write a macro `new_struct!` that generates a struct with named
//    fields and a `new()` constructor.
// 3. Write a macro `measure!` that times how long an expression takes
//    and prints the duration.
// 4. Write a macro that generates From implementations between types.
// ============================================================
