// ============================================================
// STEP 01: Hello World & Rust Basics
// ============================================================
// Run: rustc main.rs && ./main
//
// This is where every Rust journey begins. We'll cover:
// - The main function (entry point)
// - Printing to the console
// - Comments
// - Basic string formatting
// ============================================================

// Every Rust program starts with a `main` function.
// `fn` is the keyword to declare a function.
fn main() {
    // --- 1.1 Basic Printing ---

    // `println!` is a macro (note the `!`). It prints text followed by a newline.
    println!("Hello, World!");

    // `print!` prints without a newline.
    print!("Hello ");
    print!("without newline\n"); // \n adds a manual newline

    // --- 1.2 String Formatting with `{}` ---

    // Use `{}` as a placeholder. Rust replaces it with the value.
    println!("My name is {} and I am {} years old.", "Alice", 30);

    // Named placeholders for clarity:
    println!(
        "{name} is learning {language}",
        name = "Bob",
        language = "Rust"
    );

    // Positional placeholders:
    println!("{0} likes {1}. {0} also likes {2}.", "Alice", "Rust", "Coffee");

    // --- 1.3 Debug Printing ---

    // `{:?}` uses the Debug trait — useful for inspecting values.
    println!("Debug print: {:?}", (1, 2, 3)); // prints a tuple
    println!("Pretty debug:\n{:#?}", vec![1, 2, 3]); // pretty-printed

    // --- 1.4 Comments ---

    // This is a single-line comment.

    /* This is a
       multi-line comment. */

    /// This is a doc comment (used to document functions/structs).
    /// It supports Markdown and is extracted by `cargo doc`.

    // --- 1.5 Escape Characters ---

    println!("Tab:\tIndented");
    println!("Newline:\nSecond line");
    println!("Backslash: \\");
    println!("Quote: \"Hello\"");

    // Raw strings (no escape processing):
    println!(r"Raw string: \n is not a newline here");
    println!(r#"Can use "quotes" inside raw strings"#);

    // --- 1.6 Simple Math in Print ---

    println!("2 + 3 = {}", 2 + 3);
    println!("10 / 3 = {}", 10 / 3);     // integer division = 3
    println!("10.0 / 3.0 = {}", 10.0 / 3.0); // float division = 3.333...

    println!("\n--- Step 01 Complete! ---");
    println!("Next: step_02 — Variables, Types & Mutability");
}

// ============================================================
// EXERCISES:
// 1. Print your name and age using `println!` with placeholders.
// 2. Try using `{:?}` to debug-print a tuple of your favorite things.
// 3. What happens if you use `{}` instead of `{:?}` for a tuple?
//    (Hint: it won't compile — tuples don't implement Display by default.)
// ============================================================
