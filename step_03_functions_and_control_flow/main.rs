// ============================================================
// STEP 03: Functions & Control Flow
// ============================================================
// Run: rustc main.rs && ./main
//
// Key concepts:
// - Function definitions and return values
// - Expressions vs statements
// - if/else, loops, while, for
// ============================================================

fn main() {
    // --- 3.1 Calling Functions ---

    greet("World");
    greet("Rustacean");

    // --- 3.2 Functions with Return Values ---

    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);

    let result = multiply(4, 7);
    println!("4 * 7 = {}", result);

    // --- 3.3 Expressions vs Statements ---

    // A statement performs an action but doesn't return a value.
    // An expression evaluates to a value.

    // This is a statement:
    let _x = 5; // `let _x = 5;` doesn't return anything

    // This block is an expression (note: no semicolon on last line):
    let y = {
        let a = 10;
        let b = 20;
        a + b // <-- expression (no semicolon) = this is the block's value
    };
    println!("Block expression: {}", y); // 30

    // --- 3.4 if / else ---

    let number = 7;

    if number < 0 {
        println!("Negative");
    } else if number == 0 {
        println!("Zero");
    } else {
        println!("{} is positive", number);
    }

    // `if` is an expression — you can use it on the right side of `let`:
    let description = if number % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", number, description);

    // --- 3.5 loop (infinite loop with break) ---

    let mut counter = 0;
    let final_value = loop {
        counter += 1;
        if counter == 5 {
            break counter * 10; // `break` can return a value from `loop`
        }
    };
    println!("Loop returned: {}", final_value); // 50

    // --- 3.6 while loop ---

    let mut countdown = 5;
    while countdown > 0 {
        print!("{}... ", countdown);
        countdown -= 1;
    }
    println!("Liftoff!");

    // --- 3.7 for loop (the most common loop in Rust) ---

    // Iterating over a range:
    print!("Range: ");
    for i in 1..=5 {
        // 1..=5 is inclusive (1, 2, 3, 4, 5)
        print!("{} ", i);
    }
    println!();

    print!("Exclusive range: ");
    for i in 1..5 {
        // 1..5 is exclusive (1, 2, 3, 4)
        print!("{} ", i);
    }
    println!();

    // Iterating over an array:
    let fruits = ["apple", "banana", "cherry"];
    for fruit in fruits.iter() {
        println!("I like {}", fruit);
    }

    // With index using enumerate:
    for (index, fruit) in fruits.iter().enumerate() {
        println!("  [{}] {}", index, fruit);
    }

    // Reverse iteration:
    print!("Countdown: ");
    for i in (1..=5).rev() {
        print!("{} ", i);
    }
    println!();

    // --- 3.8 Nested Loops with Labels ---

    // Labels let you break/continue outer loops from inner ones.
    'outer: for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                println!("Breaking outer loop at ({}, {})", i, j);
                break 'outer;
            }
            print!("({},{}) ", i, j);
        }
        println!();
    }
    println!();

    // --- 3.9 Putting It Together ---

    println!("Is 7 prime? {}", is_prime(7));
    println!("Is 12 prime? {}", is_prime(12));
    println!("Is 2 prime? {}", is_prime(2));

    println!("Fibonacci(10) = {}", fibonacci(10));

    println!("Fahrenheit 100 -> Celsius {:.1}", fahrenheit_to_celsius(100.0));
    println!("Celsius 37 -> Fahrenheit {:.1}", celsius_to_fahrenheit(37.0));

    println!("\n--- Step 03 Complete! ---");
    println!("Next: step_04 — Ownership & Borrowing");
}

// --- Function Definitions ---

// Parameters must have type annotations. No return type = returns `()`.
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Return type is specified after `->`
fn add(a: i32, b: i32) -> i32 {
    a + b // expression without semicolon = implicit return
}

fn multiply(a: i32, b: i32) -> i32 {
    return a * b; // explicit return also works (but idiomatic Rust prefers no `return`)
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    // Check divisors up to sqrt(n)
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

// ============================================================
// EXERCISES:
// 1. Write a function `max(a: i32, b: i32) -> i32` that returns the larger.
// 2. Write a function that takes an array slice and returns the sum.
// 3. Use a `for` loop to print the multiplication table for 7.
// 4. Write a fizzbuzz function for numbers 1 to 30.
// ============================================================
