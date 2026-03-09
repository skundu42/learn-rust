// ============================================================
// STEP 11: Closures & Iterators
// ============================================================
// Run: rustc main.rs && ./main
//
// Closures are anonymous functions that can capture their environment.
// Iterators provide a lazy, composable way to process sequences.
// Together they enable Rust's functional programming style.
// ============================================================

fn main() {
    // =========================
    // 11.1 Closure Basics
    // =========================

    // A closure is an anonymous function you can save in a variable.
    let add = |a: i32, b: i32| -> i32 { a + b };
    println!("add(2, 3) = {}", add(2, 3));

    // Type annotations are optional when types can be inferred:
    let double = |x| x * 2;
    println!("double(5) = {}", double(5));

    // Single-expression closures don't need braces:
    let square = |x: i32| x * x;
    println!("square(4) = {}", square(4));

    // Zero-parameter closure:
    let greet = || println!("Hello from a closure!");
    greet();

    // =========================
    // 11.2 Closures Capture Environment
    // =========================

    // Unlike regular functions, closures can capture variables from their scope.
    let name = String::from("Rust");
    let greeting = || println!("Hello, {}!", name); // captures `name` by reference
    greeting();
    println!("name is still valid: {}", name); // name wasn't moved

    // Capture by mutable reference:
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("Count: {}", count);
    };
    increment();
    increment();
    increment();
    // Can use count again after the closure is done borrowing:
    drop(increment); // explicitly end the borrow
    println!("Final count: {}", count);

    // Capture by value with `move`:
    let data = vec![1, 2, 3];
    let owns_data = move || {
        println!("Moved data: {:?}", data);
    };
    owns_data();
    // println!("{:?}", data); // ERROR: data was moved into the closure

    // =========================
    // 11.3 Closures as Function Parameters
    // =========================

    // Three closure traits:
    // - FnOnce: takes ownership, can only be called once
    // - FnMut: borrows mutably, can be called multiple times
    // - Fn: borrows immutably, can be called multiple times

    apply(5, |x| println!("  Applied: {}", x * 2));
    apply(10, |x| println!("  Applied: {}", x + 1));

    let result = transform(5, |x| x * x);
    println!("Transformed: {}", result);

    // =========================
    // 11.4 Returning Closures
    // =========================

    let adder = make_adder(10);
    println!("Adder: {}", adder(5));   // 15
    println!("Adder: {}", adder(20));  // 30

    let multiplier = make_multiplier(3);
    println!("Multiplier: {}", multiplier(7)); // 21

    // =========================
    // 11.5 Iterator Basics
    // =========================

    println!("\n--- Iterators ---");

    let numbers = vec![1, 2, 3, 4, 5];

    // `.iter()` creates an iterator that borrows each element
    let mut iter = numbers.iter();
    println!("Next: {:?}", iter.next()); // Some(1)
    println!("Next: {:?}", iter.next()); // Some(2)
    println!("Next: {:?}", iter.next()); // Some(3)

    // for loop uses iterators internally:
    for num in numbers.iter() {
        print!("{} ", num);
    }
    println!();

    // Three iterator types:
    // .iter()       -> borrows (&T)
    // .iter_mut()   -> borrows mutably (&mut T)
    // .into_iter()  -> takes ownership (T)

    // =========================
    // 11.6 Iterator Adaptors (Lazy Transformations)
    // =========================

    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map: transform each element
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // filter: keep elements matching a condition
    let evens: Vec<&i32> = nums.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Evens: {:?}", evens);

    // Combined chain:
    let result: Vec<i32> = nums
        .iter()
        .filter(|&&x| x % 2 == 0)  // keep evens
        .map(|&x| x * x)            // square them
        .collect();
    println!("Even squares: {:?}", result);

    // enumerate: add index
    for (i, val) in nums.iter().enumerate().take(5) {
        println!("  [{}] = {}", i, val);
    }

    // zip: combine two iterators
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![30, 25, 35];
    let people: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("Zipped: {:?}", people);

    // chain: concatenate iterators
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let combined: Vec<&i32> = a.iter().chain(b.iter()).collect();
    println!("Chained: {:?}", combined);

    // flat_map: map and flatten
    let sentences = vec!["hello world", "foo bar baz"];
    let words: Vec<&str> = sentences.iter().flat_map(|s| s.split_whitespace()).collect();
    println!("Words: {:?}", words);

    // skip and take:
    let middle: Vec<&i32> = nums.iter().skip(3).take(4).collect();
    println!("Skip 3, take 4: {:?}", middle);

    // =========================
    // 11.7 Consuming Adaptors (Produce a Value)
    // =========================

    println!("\n--- Consuming Adaptors ---");

    let nums = vec![1, 2, 3, 4, 5];

    // sum
    let total: i32 = nums.iter().sum();
    println!("Sum: {}", total);

    // product
    let product: i32 = nums.iter().product();
    println!("Product: {}", product);

    // count
    let count = nums.iter().count();
    println!("Count: {}", count);

    // min, max
    println!("Min: {:?}", nums.iter().min());
    println!("Max: {:?}", nums.iter().max());

    // any, all
    println!("Any > 3? {}", nums.iter().any(|&x| x > 3));
    println!("All > 0? {}", nums.iter().all(|&x| x > 0));

    // find
    let first_even = nums.iter().find(|&&x| x % 2 == 0);
    println!("First even: {:?}", first_even);

    // position
    let pos = nums.iter().position(|&x| x == 3);
    println!("Position of 3: {:?}", pos);

    // fold (like reduce) — most powerful consuming adaptor
    let sum = nums.iter().fold(0, |acc, &x| acc + x);
    println!("Fold sum: {}", sum);

    let sentence = nums
        .iter()
        .fold(String::new(), |acc, &x| {
            if acc.is_empty() {
                x.to_string()
            } else {
                format!("{}, {}", acc, x)
            }
        });
    println!("Fold to string: {}", sentence);

    // =========================
    // 11.8 Creating Your Own Iterator
    // =========================

    println!("\n--- Custom Iterator ---");

    let counter = Counter::new(5);
    for val in counter {
        print!("{} ", val);
    }
    println!();

    // Using iterator methods on our custom iterator:
    let sum: u32 = Counter::new(5).sum();
    println!("Counter sum: {}", sum);

    let doubled: Vec<u32> = Counter::new(5).map(|x| x * 2).collect();
    println!("Counter doubled: {:?}", doubled);

    // Zip two counters:
    let pairs: Vec<(u32, u32)> = Counter::new(3).zip(Counter::new(3).skip(1)).collect();
    println!("Counter pairs: {:?}", pairs);

    // =========================
    // 11.9 Practical Examples
    // =========================

    println!("\n--- Practical Examples ---");

    // Word frequency
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let mut freq: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    text.split_whitespace().for_each(|word| {
        *freq.entry(word).or_insert(0) += 1;
    });
    let mut freq_vec: Vec<_> = freq.iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("Word frequency (sorted):");
    for (word, count) in freq_vec.iter().take(5) {
        println!("  {}: {}", word, count);
    }

    // Pipeline: process a list of scores
    let scores = vec![85, 92, 45, 67, 88, 73, 95, 55, 78, 82];
    let passing_average: f64 = {
        let passing: Vec<&i32> = scores.iter().filter(|&&s| s >= 60).collect();
        let count = passing.len() as f64;
        let sum: i32 = passing.into_iter().sum();
        sum as f64 / count
    };
    println!("Average of passing scores: {:.1}", passing_average);

    // Fibonacci using iterators
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("Fibonacci(10): {:?}", fibs);

    println!("\n--- Step 11 Complete! ---");
    println!("Next: step_12 — Modules & Crates");
}

// ============================================================
// Helper Functions and Types
// ============================================================

// --- 11.3 Closure Parameters ---

fn apply<F: Fn(i32)>(value: i32, f: F) {
    f(value);
}

fn transform<F: Fn(i32) -> i32>(value: i32, f: F) -> i32 {
    f(value)
}

// --- 11.4 Returning Closures ---

fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn make_multiplier(factor: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x * factor)
}

// --- 11.8 Custom Iterator ---

struct Counter {
    max: u32,
    current: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { max, current: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32; // Associated type: what the iterator yields

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

// --- Fibonacci Iterator ---

struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.a;
        self.a = self.b;
        self.b = next + self.b;
        Some(next)
    }
}

// ============================================================
// EXERCISES:
// 1. Use .map() and .filter() to get the lengths of words with 4+ chars
//    from a sentence.
// 2. Write a closure that captures a Vec and returns its sum.
// 3. Use .fold() to find the longest string in a Vec<&str>.
// 4. Create a custom iterator `Range` that yields numbers from start to end.
// 5. Use iterators to flatten a Vec<Vec<i32>> into a single Vec<i32>.
// ============================================================
