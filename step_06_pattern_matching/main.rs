// ============================================================
// STEP 06: Pattern Matching
// ============================================================
// Run: rustc main.rs && ./main
//
// `match` is one of Rust's most powerful features.
// It's like switch/case on steroids — exhaustive, safe, and expressive.
// ============================================================

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    California,
    NewYork,
}

#[derive(Debug)]
enum Command {
    Quit,
    Print(String),
    Move { x: i32, y: i32 },
    ChangeColor(u8, u8, u8),
}

fn main() {
    // --- 6.1 Basic match ---

    let number = 3;
    let word = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        4 | 5 => "four or five", // multiple patterns with |
        _ => "something else",   // `_` is the catch-all (wildcard)
    };
    println!("{} = {}", number, word);

    // match MUST be exhaustive — you must handle all possibilities.

    // --- 6.2 Match with Enums ---

    let coin = Coin::Quarter(UsState::California);
    let cents = value_in_cents(&coin);
    println!("{:?} = {} cents", coin, cents);

    // --- 6.3 Match with Destructuring ---

    let cmd = Command::Move { x: 10, y: 20 };
    match &cmd {
        Command::Quit => println!("Quit"),
        Command::Print(text) => println!("Print: {}", text),
        Command::Move { x, y } => println!("Move to ({}, {})", x, y),
        Command::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
    }

    // --- 6.4 Match Guards (extra conditions) ---

    let num = 4;
    let description = match num {
        n if n < 0 => "negative",
        0 => "zero",
        n if n % 2 == 0 => "positive even",
        _ => "positive odd",
    };
    println!("{} is {}", num, description);

    // --- 6.5 Binding with @ ---

    let age = 25;
    match age {
        0 => println!("newborn"),
        age @ 1..=12 => println!("child, age {}", age),
        age @ 13..=17 => println!("teenager, age {}", age),
        age @ 18..=64 => println!("adult, age {}", age),
        age @ 65.. => println!("senior, age {}", age),
        // Negative ages handled if type were i32:
        _ => unreachable!(),
    }

    // --- 6.6 Tuple Matching ---

    let point = (0, -2);
    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On x-axis at {}", x),
        (0, y) => println!("On y-axis at {}", y),
        (x, y) => println!("Point at ({}, {})", x, y),
    }

    // --- 6.7 Nested Destructuring ---

    #[derive(Debug)]
    struct Position {
        x: i32,
        y: i32,
    }

    #[derive(Debug)]
    enum GameEntity {
        Player(Position),
        Enemy { pos: Position, health: u32 },
        Item(String, Position),
    }

    let entity = GameEntity::Enemy {
        pos: Position { x: 5, y: 10 },
        health: 100,
    };

    match &entity {
        GameEntity::Player(Position { x, y }) => {
            println!("Player at ({}, {})", x, y);
        }
        GameEntity::Enemy {
            pos: Position { x, y },
            health,
        } => {
            println!("Enemy at ({}, {}) with {} HP", x, y, health);
        }
        GameEntity::Item(name, Position { x, y }) => {
            println!("Item '{}' at ({}, {})", name, x, y);
        }
    }

    // --- 6.8 if let (Concise Single-Pattern Match) ---

    let some_value: Option<i32> = Some(42);

    // Instead of:
    match some_value {
        Some(v) => println!("Got: {}", v),
        None => {}
    }

    // You can write:
    if let Some(v) = some_value {
        println!("Got (if let): {}", v);
    }

    // if let with else:
    if let Some(v) = some_value {
        println!("Value: {}", v);
    } else {
        println!("No value");
    }

    // --- 6.9 while let ---

    let mut stack = vec![1, 2, 3, 4, 5];
    print!("Popping: ");
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();

    // --- 6.10 let-else (Rust 1.65+) ---

    // Unwrap or diverge (return, break, panic, etc.)
    let config_value: Option<&str> = Some("8080");
    let Some(port_str) = config_value else {
        println!("No port configured");
        return;
    };
    println!("Port: {}", port_str);

    // --- 6.11 Matching References ---

    let reference = &4;

    match reference {
        &val => println!("Got a value via &: {}", val),
    }

    // Or dereference before matching:
    match *reference {
        val => println!("Got a value via *: {}", val),
    }

    // --- 6.12 Ignoring Parts of a Pattern ---

    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("1st: {}, 3rd: {}, 5th: {}", first, third, fifth);
        }
    }

    // Ignore remaining with `..`
    match numbers {
        (first, .., last) => {
            println!("First: {}, Last: {}", first, last);
        }
    }

    // --- 6.13 Comprehensive Example ---

    println!("\nGrading:");
    for score in [95, 85, 72, 60, 45] {
        let grade = grade_score(score);
        println!("  Score {} -> Grade {}", score, grade);
    }

    println!("\n--- Step 06 Complete! ---");
    println!("Next: step_07 — Collections");
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("  Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("  Quarter from {:?}", state);
            25
        }
    }
}

fn grade_score(score: u32) -> &'static str {
    match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        0..=59 => "F",
        _ => "Invalid",
    }
}

// ============================================================
// EXERCISES:
// 1. Write a match expression for a traffic light enum that returns
//    how many seconds to wait (Red=60, Yellow=5, Green=45).
// 2. Match on a tuple (bool, bool) representing (has_ticket, is_vip)
//    to decide if someone can enter an event.
// 3. Write a function using `if let` to print the inner value of
//    Option<Vec<i32>> only if it has elements.
// ============================================================
