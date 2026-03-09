// ============================================================
// STEP 02: Variables, Types & Mutability
// ============================================================
// Run: rustc main.rs && ./main
//
// Key concepts:
// - Variables are immutable by default
// - `mut` makes them mutable
// - Rust has strong static typing with type inference
// - Shadowing lets you re-declare a variable
// ============================================================

fn main() {
    // --- 2.1 Immutable Variables (Default) ---

    let x = 5;
    println!("x = {}", x);

    // This would NOT compile — variables are immutable by default:
    // x = 10; // ERROR: cannot assign twice to immutable variable

    // --- 2.2 Mutable Variables ---

    let mut y = 10;
    println!("y = {}", y);
    y = 20; // OK because `y` is declared with `mut`
    println!("y is now = {}", y);

    // --- 2.3 Constants ---

    // Constants must have a type annotation and use SCREAMING_SNAKE_CASE.
    // They are always immutable and evaluated at compile time.
    const MAX_POINTS: u32 = 100_000; // underscores improve readability
    println!("Max points: {}", MAX_POINTS);

    // --- 2.4 Shadowing ---

    // You can re-declare a variable with `let`. This "shadows" the previous one.
    let z = 5;
    let z = z + 1;       // z is now 6
    let z = z * 2;       // z is now 12
    println!("z = {}", z);

    // Shadowing even lets you change the type:
    let spaces = "   ";          // &str
    let spaces = spaces.len();   // now it's usize
    println!("Number of spaces: {}", spaces);

    // --- 2.5 Scalar Types ---

    // Integers
    let a: i8 = -128;          // 8-bit signed: -128 to 127
    let b: u8 = 255;           // 8-bit unsigned: 0 to 255
    let c: i32 = 2_147_483_647; // 32-bit signed (default integer type)
    let d: i64 = 9_000_000_000; // 64-bit signed
    let e: isize = 42;         // pointer-sized (depends on architecture)
    println!("Integers: {} {} {} {} {}", a, b, c, d, e);

    // Integer literals in different bases:
    let hex = 0xff;        // 255
    let octal = 0o77;      // 63
    let binary = 0b1010;   // 10
    let byte = b'A';       // 65 (u8 only)
    println!("Hex: {}, Octal: {}, Binary: {}, Byte: {}", hex, octal, binary, byte);

    // Floating point
    let f1: f64 = 3.14;    // 64-bit float (default)
    let f2: f32 = 2.71;    // 32-bit float
    println!("Floats: {} {}", f1, f2);

    // Boolean
    let is_active: bool = true;
    let is_admin = false;   // type inferred
    println!("Active: {}, Admin: {}", is_active, is_admin);

    // Character — Rust `char` is 4 bytes (Unicode scalar value)
    let letter = 'A';
    let emoji = '🦀';
    let chinese = '中';
    println!("Chars: {} {} {}", letter, emoji, chinese);

    // --- 2.6 Compound Types ---

    // Tuple — fixed size, can hold different types
    let tup: (i32, f64, char) = (500, 6.4, 'R');
    let (tx, ty, tz) = tup;                  // destructuring
    println!("Tuple: ({}, {}, {})", tx, ty, tz);
    println!("Access by index: {}", tup.0);   // 500

    // Unit type — an empty tuple, used when there's no meaningful value
    let _unit: () = ();

    // Array — fixed size, all elements same type, stack-allocated
    let arr = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);
    println!("First element: {}", arr[0]);
    println!("Array length: {}", arr.len());

    // Initialize array with same value:
    let zeros = [0; 5]; // [0, 0, 0, 0, 0]
    println!("Zeros: {:?}", zeros);

    // --- 2.7 Type Conversions ---

    // Rust does NOT do implicit type conversions. You must be explicit.
    let integer: i32 = 42;
    let float: f64 = integer as f64; // explicit cast
    println!("{} as f64 = {}", integer, float);

    let big: i64 = 1000;
    let small: i8 = big as i8; // truncation! 1000 doesn't fit in i8
    println!("1000 as i8 = {} (truncated!)", small);

    // --- 2.8 Type Aliases ---

    type Meters = f64;
    type Seconds = f64;
    let distance: Meters = 100.0;
    let time: Seconds = 9.58;
    println!("Speed: {:.2} m/s", distance / time);

    println!("\n--- Step 02 Complete! ---");
    println!("Next: step_03 — Functions & Control Flow");
}

// ============================================================
// EXERCISES:
// 1. Create variables for your height (f64) and age (u8). Print them.
// 2. Try to assign a string to a variable declared as i32. What error?
// 3. Create a tuple with 4 elements and access the third one by index.
// 4. What happens if you access arr[10] on a 5-element array? (Runtime panic!)
// 5. Shadow a variable from &str to its length (usize).
// ============================================================
