// ============================================================
// STEP 08: Error Handling
// ============================================================
// Run: rustc main.rs && ./main
//
// Rust has NO exceptions. Instead it uses:
// - Result<T, E> for recoverable errors
// - panic! for unrecoverable errors
// - The ? operator for ergonomic error propagation
// ============================================================

use std::fs;
use std::io;
use std::num::ParseIntError;

fn main() {
    // --- 8.1 panic! (Unrecoverable Errors) ---

    // `panic!` crashes the program immediately.
    // Uncomment to see:
    // panic!("Something went terribly wrong!");

    // Out-of-bounds access also panics:
    // let v = vec![1, 2, 3];
    // v[10]; // thread panics at runtime

    // --- 8.2 Result<T, E> (Recoverable Errors) ---

    // Result is an enum:
    //   enum Result<T, E> {
    //       Ok(T),   // success with value T
    //       Err(E),  // error with value E
    //   }

    // Parsing a string to a number can fail:
    let good: Result<i32, ParseIntError> = "42".parse();
    let bad: Result<i32, ParseIntError> = "not_a_number".parse();

    println!("Parsing '42': {:?}", good);           // Ok(42)
    println!("Parsing 'not_a_number': {:?}", bad);   // Err(...)

    // --- 8.3 Handling Results with match ---

    match "42".parse::<i32>() {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Failed to parse: {}", e),
    }

    // --- 8.4 unwrap and expect ---

    // `unwrap()` — get the value or panic
    let n: i32 = "100".parse().unwrap();
    println!("Unwrapped: {}", n);
    // "abc".parse::<i32>().unwrap(); // PANICS!

    // `expect()` — like unwrap but with a custom message
    let n: i32 = "200".parse().expect("Failed to parse number");
    println!("Expected: {}", n);

    // --- 8.5 unwrap_or, unwrap_or_else, unwrap_or_default ---

    let val: i32 = "abc".parse().unwrap_or(0);
    println!("unwrap_or: {}", val); // 0

    let val: i32 = "abc".parse().unwrap_or_else(|_| {
        println!("  (falling back to default)");
        -1
    });
    println!("unwrap_or_else: {}", val); // -1

    let val: i32 = "abc".parse().unwrap_or_default();
    println!("unwrap_or_default: {}", val); // 0 (default for i32)

    // --- 8.6 The ? Operator ---

    // The `?` operator propagates errors. If the Result is Err, it
    // returns early from the function with that error.

    match read_number_from_string("42") {
        Ok(n) => println!("\n? operator result: {}", n),
        Err(e) => println!("\n? operator error: {}", e),
    }

    match read_number_from_string("abc") {
        Ok(n) => println!("? operator result: {}", n),
        Err(e) => println!("? operator error: {}", e),
    }

    // --- 8.7 Chaining with ? ---

    match double_parse("21") {
        Ok(n) => println!("Double parsed: {}", n),
        Err(e) => println!("Double parse error: {}", e),
    }

    // --- 8.8 Custom Error Types ---

    match validate_age("25") {
        Ok(age) => println!("\nValid age: {}", age),
        Err(e) => println!("\nAge error: {:?}", e),
    }
    match validate_age("abc") {
        Ok(age) => println!("Valid age: {}", age),
        Err(e) => println!("Age error: {:?}", e),
    }
    match validate_age("200") {
        Ok(age) => println!("Valid age: {}", age),
        Err(e) => println!("Age error: {:?}", e),
    }
    match validate_age("-5") {
        Ok(age) => println!("Valid age: {}", age),
        Err(e) => println!("Age error: {:?}", e),
    }

    // --- 8.9 Box<dyn Error> (Simple Error Handling) ---

    match flexible_parse("42") {
        Ok(n) => println!("\nFlexible: {}", n),
        Err(e) => println!("\nFlexible error: {}", e),
    }

    // --- 8.10 Result Methods ---

    // map: transform the Ok value
    let result: Result<i32, ParseIntError> = "5".parse();
    let doubled = result.map(|n| n * 2);
    println!("\nMapped: {:?}", doubled); // Ok(10)

    // and_then: chain operations that can fail
    let chained = "5"
        .parse::<i32>()
        .and_then(|n| {
            if n > 0 {
                Ok(n * 2)
            } else {
                Err("negative".parse::<i32>().unwrap_err())
            }
        });
    println!("Chained: {:?}", chained);

    // map_err: transform the error
    let result: Result<i32, String> = "abc"
        .parse::<i32>()
        .map_err(|e| format!("Parse failed: {}", e));
    println!("map_err: {:?}", result);

    // is_ok / is_err:
    let ok_result: Result<i32, String> = Ok(42);
    let err_result: Result<i32, String> = Err("oops".into());
    println!("is_ok: {}, is_err: {}", ok_result.is_ok(), err_result.is_err());

    // --- 8.11 File I/O Example ---

    println!("\n--- File I/O Error Handling ---");
    match read_file_contents("/tmp/nonexistent_file.txt") {
        Ok(contents) => println!("File: {}", contents),
        Err(e) => println!("Could not read file: {}", e),
    }

    // Write then read:
    let temp_path = "/tmp/rust_tutorial_test.txt";
    fs::write(temp_path, "Hello from Rust!").expect("Could not write file");
    match read_file_contents(temp_path) {
        Ok(contents) => println!("Read back: {}", contents),
        Err(e) => println!("Error: {}", e),
    }
    let _ = fs::remove_file(temp_path); // cleanup

    println!("\n--- Step 08 Complete! ---");
    println!("Next: step_09 — Traits & Generics");
}

// --- Functions demonstrating error handling ---

fn read_number_from_string(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse::<i32>()?; // ? returns Err early if parse fails
    Ok(n * 2)
}

fn double_parse(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse::<i32>()?;  // step 1: parse
    let doubled = (n * 2).to_string().parse::<i32>()?; // step 2: double and re-parse
    Ok(doubled)
}

// --- Custom Error Type ---

#[derive(Debug)]
enum AgeError {
    ParseError(ParseIntError),
    TooYoung,
    TooOld,
    Negative,
}

// Implement Display so we can print it nicely
impl std::fmt::Display for AgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgeError::ParseError(e) => write!(f, "Could not parse age: {}", e),
            AgeError::TooYoung => write!(f, "Too young (must be >= 0)"),
            AgeError::TooOld => write!(f, "Too old (must be <= 150)"),
            AgeError::Negative => write!(f, "Age cannot be negative"),
        }
    }
}

// Allow automatic conversion from ParseIntError to AgeError
impl From<ParseIntError> for AgeError {
    fn from(e: ParseIntError) -> Self {
        AgeError::ParseError(e)
    }
}

fn validate_age(input: &str) -> Result<i32, AgeError> {
    let age: i32 = input.parse()?; // auto-converts ParseIntError -> AgeError
    if age < 0 {
        Err(AgeError::Negative)
    } else if age > 150 {
        Err(AgeError::TooOld)
    } else {
        Ok(age)
    }
}

// --- Box<dyn Error> approach ---

fn flexible_parse(s: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let n = s.parse::<i32>()?;
    Ok(n)
}

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

// ============================================================
// EXERCISES:
// 1. Write a function `divide(a: f64, b: f64) -> Result<f64, String>`
//    that returns Err if b is zero.
// 2. Create a function that reads a file and parses each line as an i32,
//    returning Vec<i32> or an error.
// 3. Create your own error enum for a login system:
//    UserNotFound, WrongPassword, AccountLocked.
// 4. Use the ? operator to chain: read file -> parse number -> double it.
// ============================================================
