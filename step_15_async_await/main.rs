// ============================================================
// STEP 15: Async/Await
// ============================================================
// Run: rustc --edition 2021 main.rs && ./main
//
// NOTE: Real async Rust uses a runtime like `tokio` or `async-std`.
// This file demonstrates the CONCEPTS using a minimal manual approach.
// For production code, use: cargo add tokio --features full
//
// Key concepts:
// - `async fn` returns a Future (lazy — doesn't run until polled)
// - `.await` suspends until the future completes
// - An executor/runtime drives futures to completion
// ============================================================

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

fn main() {
    // =========================
    // 15.1 What is Async?
    // =========================

    println!("--- Async Concepts ---");
    println!("Synchronous: tasks run one after another.");
    println!("Asynchronous: tasks can be suspended and resumed,");
    println!("  allowing other tasks to make progress.\n");

    // =========================
    // 15.2 Futures (The Building Block)
    // =========================

    // A Future is a value that might not be ready yet.
    // It has a `poll` method that returns:
    //   Poll::Ready(value) — done!
    //   Poll::Pending — not ready yet, try again later

    println!("--- Manual Future ---");

    // Our simple countdown future:
    let countdown = Countdown::new(3);

    // Manually drive it (this is what a runtime does for you):
    let result = block_on(countdown);
    println!("Countdown result: {}\n", result);

    // =========================
    // 15.3 async fn and .await
    // =========================

    println!("--- async fn ---");

    // `async fn` automatically wraps the return value in a Future.
    // .await suspends until the future is ready.

    let result = block_on(async {
        let greeting = say_hello().await;
        println!("  {}", greeting);

        let sum = add_async(5, 3).await;
        println!("  5 + 3 = {}", sum);

        sum
    });
    println!("  Async block returned: {}\n", result);

    // =========================
    // 15.4 Sequential vs Concurrent
    // =========================

    println!("--- Sequential (one after another) ---");
    let start = Instant::now();
    block_on(async {
        let a = simulated_fetch("API A", 100).await;
        let b = simulated_fetch("API B", 100).await;
        println!("  Results: {}, {}", a, b);
    });
    println!("  Sequential took: {:?}\n", start.elapsed());

    // To run futures concurrently, you'd use tokio::join! or similar.
    // Our minimal executor doesn't support true concurrency, but here's
    // how it would look with tokio:
    //
    //   let (a, b) = tokio::join!(
    //       simulated_fetch("API A", 100),
    //       simulated_fetch("API B", 100),
    //   );
    //   // Both run concurrently — total time ~100ms, not ~200ms

    // =========================
    // 15.5 Async Closures and Blocks
    // =========================

    println!("--- Async Blocks ---");

    let future = async {
        let x = 42;
        let y = 58;
        x + y
    };
    let result = block_on(future);
    println!("  Async block: {}\n", result);

    // =========================
    // 15.6 Error Handling in Async
    // =========================

    println!("--- Async Error Handling ---");

    let result = block_on(async {
        match fetch_data(true).await {
            Ok(data) => println!("  Success: {}", data),
            Err(e) => println!("  Error: {}", e),
        }

        match fetch_data(false).await {
            Ok(data) => println!("  Success: {}", data),
            Err(e) => println!("  Error: {}", e),
        }
    });

    // =========================
    // 15.7 Async with Generics
    // =========================

    println!("\n--- Async Patterns ---");

    // Retry pattern:
    let result = block_on(retry(3, || async { fetch_data(true).await }));
    println!("  Retry result: {:?}", result);

    // =========================
    // 15.8 Tokio Example (Reference)
    // =========================

    println!("\n--- Tokio Reference ---");
    println!("In a real project with Cargo, you'd use tokio:");
    println!();
    println!("  # Cargo.toml");
    println!("  [dependencies]");
    println!("  tokio = {{ version = \"1\", features = [\"full\"] }}");
    println!();
    println!("  // main.rs");
    println!("  #[tokio::main]");
    println!("  async fn main() {{");
    println!("      let result = fetch_data().await;");
    println!("      ");
    println!("      // Run futures concurrently:");
    println!("      let (a, b) = tokio::join!(task_a(), task_b());");
    println!("      ");
    println!("      // Select first to complete:");
    println!("      tokio::select! {{");
    println!("          val = fast_task() => println!(\"fast: {{}}\", val),");
    println!("          val = slow_task() => println!(\"slow: {{}}\", val),");
    println!("      }}");
    println!("      ");
    println!("      // Spawn concurrent tasks:");
    println!("      let handle = tokio::spawn(async {{ heavy_work().await }});");
    println!("      let result = handle.await.unwrap();");
    println!("  }}");

    // =========================
    // 15.9 Pin and Unpin (Advanced)
    // =========================

    println!("\n--- Pin (Advanced) ---");
    println!("Pin<T> ensures a value won't be moved in memory.");
    println!("This is needed because async futures can be self-referential.");
    println!("Most of the time you won't need Pin directly — it's handled");
    println!("by the runtime and async/await syntax.\n");

    // When you DO encounter Pin:
    // - `Pin<Box<dyn Future>>` is a common return type for async trait methods
    // - `Pin<&mut T>` prevents moving the inner value
    // - Most types are `Unpin` (can be freely moved even when pinned)

    println!("--- Step 15 Complete! ---");
    println!("Next: step_16 — Macros");
}

// ============================================================
// Async Functions
// ============================================================

async fn say_hello() -> String {
    String::from("Hello from async!")
}

async fn add_async(a: i32, b: i32) -> i32 {
    a + b
}

async fn simulated_fetch(name: &str, millis: u64) -> String {
    // In real code this would be an actual async operation.
    // Here we just simulate a delay.
    std::thread::sleep(Duration::from_millis(millis));
    format!("{}: data", name)
}

async fn fetch_data(success: bool) -> Result<String, String> {
    if success {
        Ok(String::from("important data"))
    } else {
        Err(String::from("connection failed"))
    }
}

// Retry pattern using async
async fn retry<F, Fut, T, E>(max_attempts: u32, f: F) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut last_err = None;
    for attempt in 1..=max_attempts {
        match f().await {
            Ok(val) => return Ok(val),
            Err(e) => {
                println!("  Attempt {} failed: {:?}", attempt, e);
                last_err = Some(e);
            }
        }
    }
    Err(last_err.unwrap())
}

// ============================================================
// Minimal Future Implementation & Executor
// ============================================================

// A simple future that counts down
struct Countdown {
    remaining: u32,
}

impl Countdown {
    fn new(count: u32) -> Self {
        Countdown { remaining: count }
    }
}

impl Future for Countdown {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.remaining == 0 {
            Poll::Ready(String::from("Countdown complete!"))
        } else {
            println!("  Countdown: {}...", self.remaining);
            self.remaining -= 1;
            cx.waker().wake_by_ref(); // schedule to be polled again
            Poll::Pending
        }
    }
}

// Minimal single-threaded executor (simplified version of what tokio does)
fn block_on<F: Future>(mut future: F) -> F::Output {
    use std::task::{RawWaker, RawWakerVTable, Waker};

    // Create a no-op waker (simplest possible)
    fn dummy_raw_waker() -> RawWaker {
        fn no_op(_: *const ()) {}
        fn clone(data: *const ()) -> RawWaker {
            RawWaker::new(data, &VTABLE)
        }
        const VTABLE: RawWakerVTable =
            RawWakerVTable::new(clone, no_op, no_op, no_op);
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    let waker = unsafe { Waker::from_raw(dummy_raw_waker()) };
    let mut cx = Context::from_waker(&waker);

    // SAFETY: we never move the future after pinning
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => {
                // In a real executor, this would park the thread
                // and wait for the waker to be called.
                // We just busy-loop for simplicity.
            }
        }
    }
}

// ============================================================
// EXERCISES:
// 1. Create a project with `cargo new async-demo` and add tokio.
//    Write a main function that fetches data from two async sources
//    concurrently using tokio::join!
// 2. Implement an async function that retries up to 3 times with
//    exponential backoff.
// 3. Use tokio::spawn to run multiple tasks concurrently and
//    collect their results.
// 4. Implement a simple async timeout using tokio::select!
// ============================================================
