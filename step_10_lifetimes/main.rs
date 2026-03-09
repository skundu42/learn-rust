// ============================================================
// STEP 10: Lifetimes
// ============================================================
// Run: rustc main.rs && ./main
//
// Lifetimes are Rust's way of ensuring references are always valid.
// Every reference has a lifetime — the scope for which it's valid.
// Usually lifetimes are inferred. When they can't be, you annotate them.
//
// The key insight: lifetimes don't change how long data lives.
// They just help the compiler verify that references are valid.
// ============================================================

fn main() {
    // --- 10.1 Why Lifetimes? ---

    // This won't compile (dangling reference):
    // let r;
    // {
    //     let x = 5;
    //     r = &x;   // ERROR: `x` does not live long enough
    // }
    // println!("{}", r); // `x` is already dropped!

    // This works — `x` lives long enough:
    let x = 5;
    let r = &x;
    println!("r = {}", r);

    // --- 10.2 Lifetime Annotations in Functions ---

    let string1 = String::from("long string");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("Longest: {}", result);
        // result is used WITHIN the scope where both strings are valid
    }
    // If we tried to use `result` here with string2 already dropped,
    // the compiler would catch it IF result could reference string2.

    // --- 10.3 How Lifetime Annotations Work ---

    let s1 = "hello";
    let s2 = "world!";
    println!("Longest: {}", longest(s1, s2));

    // Lifetime doesn't change anything at runtime — it's purely a
    // compile-time check that references won't dangle.

    // --- 10.4 Lifetimes in Structs ---

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence;
    {
        // ImportantExcerpt holds a reference, so the data it references
        // must outlive the struct.
        let i = ImportantExcerpt {
            part: novel.split('.').next().unwrap(),
        };
        first_sentence = i.part;
        println!("Excerpt: {:?}", i);
    }
    println!("First sentence: {}", first_sentence);

    // --- 10.5 Lifetime Elision Rules ---

    // Rust has 3 elision rules that let you skip annotations in common cases:
    //
    // Rule 1: Each input reference gets its own lifetime parameter.
    //   fn foo(x: &str) -> ... becomes fn foo<'a>(x: &'a str) -> ...
    //
    // Rule 2: If there's exactly one input lifetime, it's assigned to all outputs.
    //   fn foo(x: &str) -> &str becomes fn foo<'a>(x: &'a str) -> &'a str
    //
    // Rule 3: If one param is &self or &mut self, its lifetime is assigned to outputs.
    //   fn method(&self, x: &str) -> &str uses self's lifetime for output.

    // These functions DON'T need explicit annotations:
    let s = String::from("hello world");
    println!("First word: {}", first_word(&s));

    // --- 10.6 Explicit Lifetimes: When You Need Them ---

    let s1 = String::from("hello");
    let s2 = String::from("world");

    // Both inputs have the same lifetime annotation, so the compiler
    // knows the returned reference lives at most as long as the shorter one.
    let result = longest(&s1, &s2);
    println!("Longest of '{}' and '{}': '{}'", s1, s2, result);

    // --- 10.7 Struct Methods with Lifetimes ---

    let excerpt = ImportantExcerpt { part: "hello world" };
    println!("Level: {}", excerpt.level());
    println!("Announce: {}", excerpt.announce_and_return("Breaking news"));

    // --- 10.8 The 'static Lifetime ---

    // `'static` means the reference lives for the ENTIRE program.
    // String literals are always 'static:
    let s: &'static str = "I live forever!";
    println!("{}", s);

    // Be careful with 'static — rarely needed and often a red herring.
    // Don't use it as a quick fix for lifetime errors.

    // --- 10.9 Lifetime Bounds on Generics ---

    let announcement = String::from("Important:");
    let s1 = String::from("long string");
    let s2 = String::from("xyz");
    let result = longest_with_announcement(&s1, &s2, announcement.as_str());
    println!("{}", result);

    // --- 10.10 Multiple Lifetimes ---

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let (first, second) = first_and_second(&s1, &s2);
    println!("First: {}, Second: {}", first, second);

    // --- 10.11 Practical Example: String Parser ---

    let data = "name=Alice;age=30;city=NYC";
    let parser = ConfigParser::new(data);
    println!("\nParsing config:");
    if let Some(name) = parser.get("name") {
        println!("  Name: {}", name);
    }
    if let Some(age) = parser.get("age") {
        println!("  Age: {}", age);
    }
    if let Some(city) = parser.get("city") {
        println!("  City: {}", city);
    }

    println!("\n--- Step 10 Complete! ---");
    println!("Next: step_11 — Closures & Iterators");
}

// ============================================================
// Functions and types with lifetimes
// ============================================================

// --- 10.2 Lifetime annotations ---

// `'a` is a lifetime parameter. It means:
// "The returned reference will be valid for as long as BOTH inputs are valid."
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// --- 10.4 Structs with lifetimes ---

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str, // This reference must live at least as long as the struct
}

// --- 10.5 Elision — no annotations needed here ---

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}

// --- 10.7 Methods with lifetimes ---

impl<'a> ImportantExcerpt<'a> {
    // No annotation needed (elision rule 3: &self's lifetime applies)
    fn level(&self) -> i32 {
        3
    }

    // Returns a reference — compiler applies self's lifetime
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part
    }
}

// --- 10.9 Generics + Lifetimes ---

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("{} Comparing '{}' and '{}'", ann, x, y);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// --- 10.10 Multiple Lifetimes ---

// Different lifetimes for different references:
fn first_and_second<'a, 'b>(s1: &'a str, s2: &'b str) -> (&'a str, &'b str) {
    (s1, s2)
}

// --- 10.11 Practical Example ---

struct ConfigParser<'a> {
    data: &'a str,
}

impl<'a> ConfigParser<'a> {
    fn new(data: &'a str) -> Self {
        ConfigParser { data }
    }

    fn get(&self, key: &str) -> Option<&'a str> {
        for pair in self.data.split(';') {
            let mut parts = pair.splitn(2, '=');
            if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
                if k == key {
                    return Some(v);
                }
            }
        }
        None
    }
}

// ============================================================
// EXERCISES:
// 1. Write a function that takes two string slices and returns the
//    shorter one. What lifetime annotation does it need?
// 2. Create a struct `TextEditor<'a>` that holds a `&'a str` buffer.
//    Add a method `word_count(&self) -> usize`.
// 3. Why does this fail? Fix it:
//      fn broken() -> &str {
//          let s = String::from("hello");
//          &s
//      }
// 4. Write a function with two different lifetime parameters that
//    returns only the first reference.
// ============================================================
