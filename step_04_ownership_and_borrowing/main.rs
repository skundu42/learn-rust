// ============================================================
// STEP 04: Ownership & Borrowing
// ============================================================
// Run: rustc main.rs && ./main
//
// THIS IS THE MOST IMPORTANT CONCEPT IN RUST.
// Ownership is how Rust achieves memory safety without a garbage collector.
//
// The Three Rules of Ownership:
// 1. Each value has exactly one owner.
// 2. When the owner goes out of scope, the value is dropped (freed).
// 3. There can only be one owner at a time.
// ============================================================

fn main() {
    // --- 4.1 Scope and Dropping ---
    {
        let s = String::from("hello"); // `s` is valid from here
        println!("{}", s);
    } // `s` goes out of scope and is dropped (memory freed)
    // println!("{}", s); // ERROR: `s` no longer exists

    // --- 4.2 Move Semantics ---

    // For heap-allocated types (like String), assignment MOVES ownership.
    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED to s2. s1 is now invalid!

    // println!("{}", s1); // ERROR: value borrowed here after move
    println!("s2 = {}", s2);

    // Why? Rust avoids "double free" errors. Only one variable owns the data.

    // --- 4.3 Clone (Deep Copy) ---

    // If you truly want a copy, use `.clone()`:
    let s3 = String::from("world");
    let s4 = s3.clone(); // deep copy — both are valid
    println!("s3 = {}, s4 = {}", s3, s4);

    // --- 4.4 Copy Types (Stack-Only Data) ---

    // Simple types like integers implement the `Copy` trait.
    // They are copied, not moved.
    let x = 5;
    let y = x; // x is COPIED, not moved
    println!("x = {}, y = {}", x, y); // both valid!

    // Copy types: i32, f64, bool, char, tuples of Copy types, etc.

    // --- 4.5 Ownership and Functions ---

    let name = String::from("Alice");
    takes_ownership(name);
    // println!("{}", name); // ERROR: `name` was moved into the function

    let num = 42;
    makes_copy(num);
    println!("num is still valid: {}", num); // OK: i32 is Copy

    // --- 4.6 Returning Ownership ---

    let s5 = gives_ownership();
    println!("Got ownership: {}", s5);

    let s6 = String::from("hello");
    let s7 = takes_and_gives_back(s6);
    // s6 is invalid, s7 has the value now
    println!("Got back: {}", s7);

    // --- 4.7 References and Borrowing ---

    // Passing ownership everywhere is tedious. Use REFERENCES instead.
    // A reference lets you use a value WITHOUT taking ownership.

    let greeting = String::from("Hello, Rust!");

    // `&greeting` creates a reference (borrows the value)
    let len = calculate_length(&greeting);
    println!("'{}' has length {}", greeting, len); // greeting is still valid!

    // --- 4.8 Mutable References ---

    let mut message = String::from("Hello");
    append_world(&mut message); // pass a mutable reference
    println!("After append: {}", message);

    // RULE: You can have EITHER:
    //   - One mutable reference, OR
    //   - Any number of immutable references
    // But NOT both at the same time.

    let mut data = String::from("data");
    let r1 = &data;     // immutable borrow
    let r2 = &data;     // another immutable borrow — OK
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut data; // mutable borrow — OK because r1, r2 are done
    r3.push_str("!");
    println!("{}", r3);

    // This prevents data races at compile time!

    // --- 4.9 Dangling References ---

    // Rust prevents dangling references (references to freed memory):
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s  // ERROR: `s` is dropped but we're returning a reference to it
    // }

    // Instead, return the owned value:
    let safe = no_dangle();
    println!("No dangle: {}", safe);

    // --- 4.10 String Slices ---

    let sentence = String::from("Hello World");

    // A string slice is a reference to a portion of a String.
    let hello = &sentence[0..5];   // "Hello"
    let world = &sentence[6..11];  // "World"
    println!("Slice: '{}' '{}'", hello, world);

    // Shorthand:
    let hello2 = &sentence[..5];   // from beginning
    let world2 = &sentence[6..];   // to end
    let whole = &sentence[..];     // whole string
    println!("{} {} {}", hello2, world2, whole);

    // `first_word` returns a slice — tied to the original string's lifetime
    let word = first_word(&sentence);
    println!("First word: {}", word);

    // --- 4.11 Other Slices ---

    // Array slices work the same way:
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3]; // [2, 3]
    println!("Array slice: {:?}", slice);

    println!("\n--- Step 04 Complete! ---");
    println!("Next: step_05 — Structs & Enums");
}

// --- Functions demonstrating ownership ---

fn takes_ownership(s: String) {
    println!("Took ownership of: {}", s);
} // `s` is dropped here

fn makes_copy(n: i32) {
    println!("Got a copy: {}", n);
} // `n` is a copy, nothing special happens

fn gives_ownership() -> String {
    String::from("yours now")
}

fn takes_and_gives_back(s: String) -> String {
    s // ownership is returned
}

// `&String` means we borrow it — we don't own it
fn calculate_length(s: &String) -> usize {
    s.len()
    // We can read `s` but not modify it
}

// `&mut String` means we borrow it mutably — we can modify it
fn append_world(s: &mut String) {
    s.push_str(", World!");
}

fn no_dangle() -> String {
    let s = String::from("safe");
    s // ownership is moved out, no dangling reference
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

// ============================================================
// EXERCISES:
// 1. Write a function that takes a String, appends " is great", and
//    returns it. Call it and print the result.
// 2. Write a function that borrows a Vec<i32> and returns the sum.
// 3. What happens if you try to create two mutable references at the
//    same time? Try it and read the compiler error.
// 4. Write a function `second_word(s: &str) -> &str` that returns the
//    second word of a string.
// ============================================================
