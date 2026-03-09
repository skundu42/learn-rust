// ============================================================
// STEP 35: Testing in Rust
// ============================================================
// Run tests: cargo test
// Run with output: cargo test -- --nocapture
// Run specific: cargo test test_name
// Run ignored: cargo test -- --ignored
//
// Rust has built-in testing support. No external framework needed!
// Covers:
// - Unit tests (#[test])
// - assert!, assert_eq!, assert_ne!
// - Testing errors and panics
// - Test organization
// - Integration tests
// - Property-based thinking
// - Test-driven development (TDD) patterns
// ============================================================

fn main() {
    println!("=== Step 35: Testing in Rust ===\n");
    println!("This step is all about tests. Run them with:");
    println!("  cargo test");
    println!("  cargo test -- --nocapture  (see println output)");
    println!("  cargo test -- --ignored    (run slow tests)");
    println!("\nLet's demo the code that we're testing:\n");

    // Demo the calculator
    let mut calc = Calculator::new();
    calc.add(10.0);
    calc.multiply(3.0);
    calc.subtract(5.0);
    println!("Calculator: 0 + 10 * 3 - 5 = {}", calc.result());
    println!("History: {:?}", calc.history());

    // Demo the validator
    println!("\nEmail validation:");
    for email in &["user@example.com", "bad-email", "@no-user.com", "a@b.c"] {
        println!("  '{}' -> {}", email, if validate_email(email) { "valid" } else { "invalid" });
    }

    // Demo the stack
    println!("\nStack:");
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("  Pop: {:?}", stack.pop());
    println!("  Peek: {:?}", stack.peek());
    println!("  Size: {}", stack.size());

    println!("\nRun `cargo test` to see all tests pass!");
}

// ============================================================
// Code Under Test
// ============================================================

// --- Calculator ---

#[derive(Debug)]
pub struct Calculator {
    value: f64,
    history: Vec<String>,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            value: 0.0,
            history: Vec::new(),
        }
    }

    pub fn result(&self) -> f64 {
        self.value
    }

    pub fn history(&self) -> &[String] {
        &self.history
    }

    pub fn add(&mut self, n: f64) -> &mut Self {
        self.value += n;
        self.history.push(format!("+ {}", n));
        self
    }

    pub fn subtract(&mut self, n: f64) -> &mut Self {
        self.value -= n;
        self.history.push(format!("- {}", n));
        self
    }

    pub fn multiply(&mut self, n: f64) -> &mut Self {
        self.value *= n;
        self.history.push(format!("* {}", n));
        self
    }

    pub fn divide(&mut self, n: f64) -> Result<&mut Self, &'static str> {
        if n == 0.0 {
            Err("Cannot divide by zero")
        } else {
            self.value /= n;
            self.history.push(format!("/ {}", n));
            Ok(self)
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        self.value = 0.0;
        self.history.clear();
        self
    }
}

// --- Email Validator ---

pub fn validate_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let (local, domain) = (parts[0], parts[1]);
    !local.is_empty()
        && !domain.is_empty()
        && domain.contains('.')
        && domain.len() > 2
        && !domain.starts_with('.')
        && !domain.ends_with('.')
}

// --- Generic Stack ---

pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

// --- Fibonacci ---

pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0u64, 1u64);
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

// --- String utilities ---

pub fn reverse_words(s: &str) -> String {
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

pub fn is_pangram(s: &str) -> bool {
    let lower = s.to_lowercase();
    ('a'..='z').all(|c| lower.contains(c))
}

pub fn caesar_cipher(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                (b'a' + (c as u8 - b'a' + shift) % 26) as char
            } else if c.is_ascii_uppercase() {
                (b'A' + (c as u8 - b'A' + shift) % 26) as char
            } else {
                c
            }
        })
        .collect()
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================
    // 35.1 Basic Assertions
    // =========================

    #[test]
    fn test_basic_assertions() {
        assert!(true);
        assert_eq!(2 + 2, 4);
        assert_ne!(2 + 2, 5);
    }

    #[test]
    fn test_with_message() {
        let x = 42;
        assert!(x > 0, "x should be positive, got {}", x);
        assert_eq!(x, 42, "Expected 42, got {}", x);
    }

    // =========================
    // 35.2 Calculator Tests
    // =========================

    #[test]
    fn test_calculator_new() {
        let calc = Calculator::new();
        assert_eq!(calc.result(), 0.0);
        assert!(calc.history().is_empty());
    }

    #[test]
    fn test_calculator_add() {
        let mut calc = Calculator::new();
        calc.add(5.0);
        assert_eq!(calc.result(), 5.0);
    }

    #[test]
    fn test_calculator_chaining() {
        let mut calc = Calculator::new();
        calc.add(10.0).multiply(2.0).subtract(5.0);
        assert_eq!(calc.result(), 15.0);
    }

    #[test]
    fn test_calculator_divide() {
        let mut calc = Calculator::new();
        calc.add(10.0);
        assert!(calc.divide(2.0).is_ok());
        assert_eq!(calc.result(), 5.0);
    }

    #[test]
    fn test_calculator_divide_by_zero() {
        let mut calc = Calculator::new();
        calc.add(10.0);
        let result = calc.divide(0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot divide by zero");
    }

    #[test]
    fn test_calculator_history() {
        let mut calc = Calculator::new();
        calc.add(5.0).multiply(2.0);
        assert_eq!(calc.history().len(), 2);
        assert_eq!(calc.history()[0], "+ 5");
        assert_eq!(calc.history()[1], "* 2");
    }

    #[test]
    fn test_calculator_reset() {
        let mut calc = Calculator::new();
        calc.add(100.0).multiply(50.0);
        calc.reset();
        assert_eq!(calc.result(), 0.0);
        assert!(calc.history().is_empty());
    }

    // Float comparison with epsilon
    #[test]
    fn test_float_precision() {
        let mut calc = Calculator::new();
        calc.add(0.1).add(0.2);
        // Don't use assert_eq! for floats directly
        assert!((calc.result() - 0.3).abs() < f64::EPSILON * 10.0);
    }

    // =========================
    // 35.3 Email Validation Tests
    // =========================

    #[test]
    fn test_valid_emails() {
        assert!(validate_email("user@example.com"));
        assert!(validate_email("test.user@domain.org"));
        assert!(validate_email("a@b.co"));
    }

    #[test]
    fn test_invalid_emails() {
        assert!(!validate_email(""));
        assert!(!validate_email("no-at-sign"));
        assert!(!validate_email("@domain.com"));
        assert!(!validate_email("user@"));
        assert!(!validate_email("user@.com"));
        assert!(!validate_email("user@com."));
        assert!(!validate_email("user@@domain.com"));
    }

    // =========================
    // 35.4 Stack Tests
    // =========================

    #[test]
    fn test_stack_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.size(), 3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);
        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.size(), 1); // peek doesn't remove
    }

    #[test]
    fn test_stack_with_strings() {
        let mut stack = Stack::new();
        stack.push(String::from("hello"));
        stack.push(String::from("world"));
        assert_eq!(stack.pop(), Some(String::from("world")));
    }

    // =========================
    // 35.5 Fibonacci Tests
    // =========================

    #[test]
    fn test_fibonacci_base_cases() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn test_fibonacci_sequence() {
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6765);
    }

    // =========================
    // 35.6 String Utility Tests
    // =========================

    #[test]
    fn test_reverse_words() {
        assert_eq!(reverse_words("hello world"), "world hello");
        assert_eq!(reverse_words("a b c"), "c b a");
        assert_eq!(reverse_words("single"), "single");
        assert_eq!(reverse_words(""), "");
    }

    #[test]
    fn test_is_pangram() {
        assert!(is_pangram("The quick brown fox jumps over the lazy dog"));
        assert!(!is_pangram("Hello, World!"));
    }

    #[test]
    fn test_caesar_cipher() {
        assert_eq!(caesar_cipher("abc", 1), "bcd");
        assert_eq!(caesar_cipher("xyz", 3), "abc");
        assert_eq!(caesar_cipher("Hello, World!", 13), "Uryyb, Jbeyq!");
    }

    #[test]
    fn test_caesar_cipher_roundtrip() {
        let original = "Hello, World!";
        let encrypted = caesar_cipher(original, 13);
        let decrypted = caesar_cipher(&encrypted, 13);
        assert_eq!(decrypted, original); // ROT13 is self-inverse
    }

    // =========================
    // 35.7 Testing Panics
    // =========================

    #[test]
    #[should_panic]
    fn test_index_out_of_bounds() {
        let v = vec![1, 2, 3];
        let _ = v[10]; // panics!
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_panic_message() {
        let v = vec![1, 2, 3];
        let _ = v[10];
    }

    // =========================
    // 35.8 Result-Based Tests
    // =========================

    #[test]
    fn test_parse_int() -> Result<(), Box<dyn std::error::Error>> {
        let value: i32 = "42".parse()?;
        assert_eq!(value, 42);
        Ok(())
    }

    // =========================
    // 35.9 Ignored Tests
    // =========================

    #[test]
    #[ignore]
    fn test_slow_operation() {
        // Run with: cargo test -- --ignored
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert!(true, "Slow test completed");
    }

    // =========================
    // 35.10 Parameterized-Style Tests
    // =========================

    #[test]
    fn test_fibonacci_table() {
        let cases = vec![
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (5, 5),
            (10, 55),
        ];

        for (input, expected) in cases {
            assert_eq!(
                fibonacci(input),
                expected,
                "fibonacci({}) should be {}",
                input,
                expected
            );
        }
    }

    // =========================
    // 35.11 Test Helpers & Setup
    // =========================

    fn make_test_stack() -> Stack<i32> {
        let mut stack = Stack::new();
        for i in 1..=5 {
            stack.push(i);
        }
        stack
    }

    #[test]
    fn test_with_helper() {
        let mut stack = make_test_stack();
        assert_eq!(stack.size(), 5);
        assert_eq!(stack.pop(), Some(5));
    }

    // =========================
    // 35.12 Module Organization
    // =========================

    mod calculator_tests {
        use super::*;

        #[test]
        fn test_complex_calculation() {
            let mut calc = Calculator::new();
            calc.add(100.0);
            let _ = calc.divide(4.0);
            calc.multiply(2.0).subtract(10.0);
            assert_eq!(calc.result(), 40.0);
        }
    }
}

// ============================================================
// WHAT YOU LEARNED:
// - #[test] attribute for test functions
// - assert!, assert_eq!, assert_ne! macros
// - #[should_panic] for testing panics
// - Result-based tests (returns Result<(), E>)
// - #[ignore] for slow tests
// - #[cfg(test)] to conditionally compile test modules
// - Test organization with submodules
// - Helper functions for test setup
// - Float comparison with epsilon
// - Table-driven (parameterized) tests
//
// TESTING COMMANDS:
//   cargo test                       # run all tests
//   cargo test test_name             # run specific test
//   cargo test -- --nocapture        # show println output
//   cargo test -- --ignored          # run ignored tests
//   cargo test -- --test-threads=1   # single-threaded
//   cargo test calculator            # run matching tests
//
// EXERCISES:
// 1. Add tests for edge cases in validate_email
// 2. Write a function and tests using TDD (test first!)
// 3. Add doc tests (/// # Examples) to a function
// 4. Create integration tests in tests/ directory
// ============================================================
