// ============================================================
// STEP 07: Collections
// ============================================================
// Run: rustc main.rs && ./main
//
// Rust's standard library provides three key collections:
// - Vec<T>: Growable array
// - String: Growable UTF-8 text
// - HashMap<K, V>: Key-value store
// ============================================================

use std::collections::HashMap;

fn main() {
    // =========================
    // 7.1 Vec<T> — Dynamic Array
    // =========================

    // Create an empty vec:
    let mut numbers: Vec<i32> = Vec::new();

    // Or use the vec! macro:
    let mut fruits = vec!["apple", "banana", "cherry"];

    // Push elements:
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    println!("Numbers: {:?}", numbers);

    // Access by index (panics if out of bounds):
    println!("First: {}", numbers[0]);

    // Safe access with .get() (returns Option):
    match numbers.get(10) {
        Some(val) => println!("Got: {}", val),
        None => println!("Index 10 is out of bounds"),
    }

    // Pop (remove last):
    let last = numbers.pop(); // Returns Option<i32>
    println!("Popped: {:?}, Remaining: {:?}", last, numbers);

    // Insert and remove at index:
    fruits.insert(1, "blueberry"); // insert at index 1
    println!("After insert: {:?}", fruits);
    fruits.remove(0); // remove index 0
    println!("After remove: {:?}", fruits);

    // Length and capacity:
    println!("Len: {}, Capacity: {}", numbers.len(), numbers.capacity());

    // Iterating:
    print!("Fruits: ");
    for fruit in &fruits {
        print!("{} ", fruit);
    }
    println!();

    // Iterating with mutation:
    let mut scores = vec![80, 90, 75, 95];
    for score in &mut scores {
        *score += 5; // dereference to modify
    }
    println!("Curved scores: {:?}", scores);

    // Useful methods:
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
    println!("Contains 4? {}", nums.contains(&4));
    println!("Min: {:?}", nums.iter().min());
    println!("Max: {:?}", nums.iter().max());
    println!("Sum: {}", nums.iter().sum::<i32>());

    let mut sorted = nums.clone();
    sorted.sort();
    sorted.dedup(); // remove consecutive duplicates (sort first!)
    println!("Sorted unique: {:?}", sorted);

    // Slicing:
    let slice = &nums[2..5];
    println!("Slice [2..5]: {:?}", slice);

    // Retain only elements matching a condition:
    let mut evens = vec![1, 2, 3, 4, 5, 6, 7, 8];
    evens.retain(|&x| x % 2 == 0);
    println!("Evens: {:?}", evens);

    // Vec of different types using enums:
    #[derive(Debug)]
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        Cell::Int(1),
        Cell::Float(3.14),
        Cell::Text(String::from("hello")),
    ];
    println!("Mixed vec: {:?}", row);

    // =========================
    // 7.2 String — UTF-8 Text
    // =========================

    println!("\n--- Strings ---");

    // Two string types:
    // - &str: string slice (borrowed, immutable, usually string literals)
    // - String: owned, growable, heap-allocated

    let literal: &str = "hello";                    // &str
    let mut owned: String = String::from("hello");  // String
    let also_owned: String = "hello".to_string();   // another way
    println!("{} {} {}", literal, owned, also_owned);

    // Appending:
    owned.push(' ');            // push a single char
    owned.push_str("world");   // push a string slice
    println!("Appended: {}", owned);

    // Concatenation:
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let s3 = s1 + &s2; // s1 is moved, s2 is borrowed
    println!("Concat: {}", s3);
    // println!("{}", s1); // ERROR: s1 was moved

    // format! macro (doesn't move anything):
    let greeting = format!("{} {}!", "Hello", "Rust");
    println!("Format: {}", greeting);

    // String length (bytes vs chars):
    let emoji_str = "Hello 🦀";
    println!("'{}' byte len: {}", emoji_str, emoji_str.len());           // bytes
    println!("'{}' char count: {}", emoji_str, emoji_str.chars().count()); // chars

    // Iterating over characters:
    print!("Chars: ");
    for c in "Rust🦀".chars() {
        print!("[{}] ", c);
    }
    println!();

    // String methods:
    let text = "  Hello, World!  ";
    println!("Trimmed: '{}'", text.trim());
    println!("Uppercase: {}", text.trim().to_uppercase());
    println!("Lowercase: {}", text.trim().to_lowercase());
    println!("Contains 'World': {}", text.contains("World"));
    println!("Starts with '  H': {}", text.starts_with("  H"));
    println!("Replace: {}", text.trim().replace("World", "Rust"));

    // Splitting:
    let csv = "one,two,three,four";
    let parts: Vec<&str> = csv.split(',').collect();
    println!("Split: {:?}", parts);

    // =========================
    // 7.3 HashMap<K, V>
    // =========================

    println!("\n--- HashMap ---");

    // Create and insert:
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 95);
    scores.insert(String::from("Bob"), 87);
    scores.insert(String::from("Charlie"), 92);
    println!("Scores: {:?}", scores);

    // Access (returns Option):
    let alice_score = scores.get("Alice");
    println!("Alice: {:?}", alice_score);

    // Access with default:
    let dave_score = scores.get("Dave").copied().unwrap_or(0);
    println!("Dave (default): {}", dave_score);

    // Check if key exists:
    println!("Has Bob? {}", scores.contains_key("Bob"));

    // Remove:
    scores.remove("Bob");
    println!("After removing Bob: {:?}", scores);

    // Iterate:
    scores.insert(String::from("Bob"), 87);
    for (name, score) in &scores {
        println!("  {} -> {}", name, score);
    }

    // Update: overwrite existing value
    scores.insert(String::from("Alice"), 100);
    println!("Alice updated: {:?}", scores.get("Alice"));

    // Insert only if key doesn't exist:
    scores.entry(String::from("Dave")).or_insert(80);
    scores.entry(String::from("Alice")).or_insert(50); // won't overwrite
    println!("After entry: {:?}", scores);

    // Update based on old value (word counting example):
    let text = "hello world hello rust hello world";
    let mut word_count: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word counts: {:?}", word_count);

    // Create from iterators:
    let teams = vec!["Red", "Blue", "Green"];
    let initial_scores = vec![0, 0, 0];
    let team_scores: HashMap<_, _> = teams.into_iter().zip(initial_scores).collect();
    println!("Team scores: {:?}", team_scores);

    // =========================
    // 7.4 Other Collections (brief mention)
    // =========================

    println!("\n--- Other Collections ---");

    // VecDeque: double-ended queue
    use std::collections::VecDeque;
    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    println!("VecDeque: {:?}", deque);

    // HashSet: unique values
    use std::collections::HashSet;
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(2); // duplicate ignored
    set.insert(3);
    println!("HashSet: {:?}", set);

    let set_a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let set_b: HashSet<i32> = vec![2, 3, 4].into_iter().collect();
    println!("Union: {:?}", set_a.union(&set_b).collect::<Vec<_>>());
    println!("Intersection: {:?}", set_a.intersection(&set_b).collect::<Vec<_>>());
    println!("Difference: {:?}", set_a.difference(&set_b).collect::<Vec<_>>());

    // BTreeMap: sorted map
    use std::collections::BTreeMap;
    let mut btree: BTreeMap<&str, i32> = BTreeMap::new();
    btree.insert("charlie", 3);
    btree.insert("alice", 1);
    btree.insert("bob", 2);
    println!("BTreeMap (sorted): {:?}", btree);

    println!("\n--- Step 07 Complete! ---");
    println!("Next: step_08 — Error Handling");
}

// ============================================================
// EXERCISES:
// 1. Create a Vec<String> of 5 city names. Sort them alphabetically.
// 2. Write a function that takes a &str and returns a HashMap<char, usize>
//    counting how many times each character appears.
// 3. Use a HashSet to find unique words in a sentence.
// 4. Create a simple phone book with HashMap<String, String>.
//    Add, look up, and remove entries.
// ============================================================
