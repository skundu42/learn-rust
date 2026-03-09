// ============================================================
// STEP 17: Unsafe Rust
// ============================================================
// Run: rustc main.rs && ./main
//
// Rust's safety guarantees are enforced at compile time.
// `unsafe` lets you opt out of some checks when you KNOW what you're doing.
//
// unsafe lets you:
// 1. Dereference raw pointers
// 2. Call unsafe functions
// 3. Access/modify mutable static variables
// 4. Implement unsafe traits
// 5. Access fields of unions
//
// IMPORTANT: unsafe doesn't turn off the borrow checker or disable
// all safety. It only enables the 5 things above. Use it sparingly.
// ============================================================

fn main() {
    // =========================
    // 17.1 Raw Pointers
    // =========================

    println!("--- Raw Pointers ---");

    let mut num = 42;

    // Creating raw pointers is SAFE (it's dereferencing that's unsafe):
    let r1 = &num as *const i32;    // immutable raw pointer
    let r2 = &mut num as *mut i32;  // mutable raw pointer

    println!("r1 address: {:?}", r1);
    println!("r2 address: {:?}", r2);

    // Dereferencing requires `unsafe`:
    unsafe {
        println!("r1 value: {}", *r1);
        println!("r2 value: {}", *r2);

        // Modify through mutable raw pointer:
        *r2 = 100;
        println!("After mutation: {}", *r2);
    }
    println!("num is now: {}", num);

    // Raw pointers can point to arbitrary addresses (dangerous!):
    // let suspicious = 0x012345usize as *const i32;
    // unsafe { println!("{}", *suspicious); } // likely segfault!

    // =========================
    // 17.2 Unsafe Functions
    // =========================

    println!("\n--- Unsafe Functions ---");

    unsafe {
        let result = dangerous_add(5, 3);
        println!("dangerous_add(5, 3) = {}", result);
    }

    // Safe wrapper around unsafe code — common pattern:
    let values = [3, 1, 4, 1, 5, 9, 2, 6];
    let (left, right) = safe_split_at(&values, 3);
    println!("Left:  {:?}", left);
    println!("Right: {:?}", right);

    // =========================
    // 17.3 Calling C Functions (FFI)
    // =========================

    println!("\n--- FFI (Foreign Function Interface) ---");

    // Call C standard library functions:
    unsafe {
        // abs from C's stdlib
        let result = abs(-42);
        println!("C abs(-42) = {}", result);

        // strlen-like behavior
        let c_string = b"Hello, C!\0";
        let len = strlen(c_string.as_ptr() as *const i8);
        println!("C strlen(\"Hello, C!\") = {}", len);
    }

    // =========================
    // 17.4 Mutable Static Variables
    // =========================

    println!("\n--- Mutable Statics ---");

    // Immutable statics are safe:
    println!("MAX_SIZE: {}", MAX_SIZE);

    // Mutable statics require unsafe (data races possible):
    unsafe {
        COUNTER += 1;
        COUNTER += 1;
        COUNTER += 1;
        println!("COUNTER: {}", COUNTER);
    }

    // In real code, use atomics or Mutex instead of mutable statics:
    use std::sync::atomic::{AtomicU32, Ordering};
    static SAFE_COUNTER: AtomicU32 = AtomicU32::new(0);
    SAFE_COUNTER.fetch_add(1, Ordering::SeqCst);
    SAFE_COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("SAFE_COUNTER: {}", SAFE_COUNTER.load(Ordering::SeqCst));

    // =========================
    // 17.5 Unsafe Traits
    // =========================

    println!("\n--- Unsafe Traits ---");

    // Some traits are unsafe to implement because the compiler can't
    // verify their invariants. Send and Sync are examples.

    let my_type = MySync { data: 42 };
    println!("MySync data: {}", my_type.data);

    // =========================
    // 17.6 Unions
    // =========================

    println!("\n--- Unions ---");

    // Unions allow different types to share the same memory.
    // Like C unions. Accessing fields is unsafe.
    let int_or_float = IntOrFloat { i: 42 };
    unsafe {
        println!("As int: {}", int_or_float.i);
        // Reading as float when it was set as int is undefined behavior
        // in general, but we can see what the bits look like:
        println!("Same bits as float: {}", int_or_float.f);
    }

    // =========================
    // 17.7 Practical: Safe Abstractions over Unsafe Code
    // =========================

    println!("\n--- Safe Abstractions ---");

    // This is THE common pattern in Rust: encapsulate unsafe in a
    // safe API. The standard library does this extensively.

    // Example: a simple stack-allocated fixed-size array wrapper
    let mut arr = FixedArray::new();
    arr.push(10);
    arr.push(20);
    arr.push(30);
    println!("FixedArray: {:?}", arr.as_slice());
    println!("arr[1] = {}", arr.get(1).unwrap());

    // Example: split_at_mut — this exists in std, implemented with unsafe
    let mut data = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = data.split_at_mut(3);
    left[0] = 100;
    right[0] = 400;
    println!("After split_at_mut: {:?}", data);

    // =========================
    // 17.8 Common Unsafe Patterns
    // =========================

    println!("\n--- Common Unsafe Patterns ---");

    // 1. Transmute — reinterpret bits as a different type (VERY dangerous)
    let bytes: [u8; 4] = [0x48, 0x65, 0x6C, 0x6F]; // "Helo"
    let as_str = unsafe { std::str::from_utf8_unchecked(&bytes) };
    println!("Transmuted bytes to str: {}", as_str);

    // 2. Pointer arithmetic
    let arr = [10, 20, 30, 40, 50];
    let ptr = arr.as_ptr();
    unsafe {
        println!("ptr[0] = {}", *ptr);
        println!("ptr[2] = {}", *ptr.add(2)); // pointer arithmetic
        println!("ptr[4] = {}", *ptr.add(4));
    }

    // 3. Interfacing with hardware/OS (conceptual)
    println!("\nUnsafe is used for:");
    println!("  - OS system calls");
    println!("  - Hardware register access");
    println!("  - SIMD intrinsics");
    println!("  - Custom allocators");
    println!("  - Lock-free data structures");

    // =========================
    // 17.9 Guidelines
    // =========================

    println!("\n--- Unsafe Guidelines ---");
    println!("1. Minimize unsafe blocks — keep them small");
    println!("2. Document invariants and safety requirements");
    println!("3. Wrap unsafe code in safe abstractions");
    println!("4. Use #[deny(unsafe_op_in_unsafe_fn)] for extra safety");
    println!("5. Prefer safe alternatives when they exist");
    println!("6. Use tools like Miri to check for UB");
    println!("   cargo +nightly miri run");

    println!("\n--- Step 17 Complete! ---");
    println!("Next: step_18 — Capstone Project");
}

// ============================================================
// Unsafe Definitions
// ============================================================

// --- 17.2 Unsafe function ---

unsafe fn dangerous_add(a: i32, b: i32) -> i32 {
    // This function is marked unsafe because... well, for demo purposes.
    // In real code, only mark functions unsafe if callers must uphold invariants.
    a + b
}

// Safe wrapper using unsafe internally:
fn safe_split_at(slice: &[i32], mid: usize) -> (&[i32], &[i32]) {
    assert!(mid <= slice.len(), "mid out of bounds");

    let ptr = slice.as_ptr();
    unsafe {
        (
            std::slice::from_raw_parts(ptr, mid),
            std::slice::from_raw_parts(ptr.add(mid), slice.len() - mid),
        )
    }
}

// --- 17.3 FFI ---

extern "C" {
    fn abs(input: i32) -> i32;
    fn strlen(s: *const i8) -> usize;
}

// You can also EXPORT Rust functions for C:
// #[no_mangle]
// pub extern "C" fn rust_function(x: i32) -> i32 {
//     x * 2
// }

// --- 17.4 Statics ---

static MAX_SIZE: u32 = 1000; // immutable static — safe
static mut COUNTER: u32 = 0; // mutable static — unsafe to access

// --- 17.5 Unsafe traits ---

// `Send`: safe to transfer between threads
// `Sync`: safe to reference from multiple threads
// Implementing these incorrectly can cause data races.

unsafe trait MyUnsafeTrait {
    fn do_something(&self);
}

struct MySync {
    data: i32,
}

unsafe impl MyUnsafeTrait for MySync {
    fn do_something(&self) {
        println!("MySync: {}", self.data);
    }
}

// --- 17.6 Union ---

#[repr(C)]
union IntOrFloat {
    i: i32,
    f: f32,
}

// --- 17.7 Safe Abstraction ---

struct FixedArray {
    data: [i32; 16],
    len: usize,
}

impl FixedArray {
    fn new() -> Self {
        FixedArray {
            data: [0; 16],
            len: 0,
        }
    }

    fn push(&mut self, value: i32) {
        assert!(self.len < 16, "FixedArray is full");
        self.data[self.len] = value;
        self.len += 1;
    }

    fn get(&self, index: usize) -> Option<i32> {
        if index < self.len {
            Some(self.data[index])
        } else {
            None
        }
    }

    fn as_slice(&self) -> &[i32] {
        &self.data[..self.len]
    }
}

// ============================================================
// EXERCISES:
// 1. Write a safe wrapper around a raw pointer that ensures
//    non-null and proper alignment.
// 2. Use FFI to call a C math function (e.g., sqrt, pow).
// 3. Implement a simple singly-linked list using raw pointers.
// 4. Create a safe API over an unsafe mutable static using
//    a mutex or atomic operations.
// ============================================================
