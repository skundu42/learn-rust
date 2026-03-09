// ============================================================
// STEP 27: DSA — Dynamic Programming
// ============================================================
// Run: rustc --edition 2021 main.rs && ./main
//
// DP is about solving overlapping subproblems efficiently.
// Common recipe:
// 1. Define the state
// 2. Write the transition
// 3. Set the base cases
// 4. Choose memoization or tabulation
// ============================================================

fn main() {
    // =========================
    // 27.1 Memoization vs Tabulation
    // =========================

    println!("--- Fibonacci: Memoization vs Tabulation ---");

    let n = 10;
    let mut memo = vec![None; n + 1];
    let fib_a = fib_memo(n, &mut memo);
    let fib_b = fib_tab(n);
    assert_eq!(fib_a, 55);
    assert_eq!(fib_b, 55);
    println!("  fib({}) = {} (memoized), {} (tabulated)", n, fib_a, fib_b);

    // =========================
    // 27.2 Climbing Stairs
    // =========================

    println!("\n--- Climbing Stairs ---");

    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(5), 8);
    println!("  Tests passed.");

    // =========================
    // 27.3 House Robber
    // =========================

    println!("\n--- House Robber ---");

    assert_eq!(house_robber(&[1, 2, 3, 1]), 4);
    assert_eq!(house_robber(&[2, 7, 9, 3, 1]), 12);
    println!("  Tests passed.");

    // =========================
    // 27.4 Coin Change
    // =========================

    println!("\n--- Coin Change ---");

    assert_eq!(coin_change(&[1, 2, 5], 11), 3);
    assert_eq!(coin_change(&[2], 3), -1);
    println!("  Tests passed.");

    // =========================
    // 27.5 Longest Increasing Subsequence
    // =========================

    println!("\n--- Longest Increasing Subsequence ---");

    assert_eq!(lis_length(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(lis_length(&[0, 1, 0, 3, 2, 3]), 4);
    println!("  Tests passed.");

    // =========================
    // 27.6 0/1 Knapsack
    // =========================

    println!("\n--- 0/1 Knapsack ---");

    let weights = [1, 3, 4, 5];
    let values = [1, 4, 5, 7];
    assert_eq!(knapsack_01(&weights, &values, 7), 9);
    println!("  Test passed.");

    // =========================
    // 27.7 Edit Distance
    // =========================

    println!("\n--- Edit Distance ---");

    assert_eq!(edit_distance("horse", "ros"), 3);
    assert_eq!(edit_distance("intention", "execution"), 5);
    println!("  Tests passed.");

    println!("\nPopular DP interview questions:");
    println!("  - House Robber");
    println!("  - Coin Change");
    println!("  - Longest Increasing Subsequence");
    println!("  - Knapsack");
    println!("  - Edit Distance / LCS");

    println!("\n--- Step 27 Complete! ---");
    println!("Next: step_28 — Interview Patterns");
}

// ============================================================
// Memoization / Tabulation
// ============================================================

fn fib_memo(n: usize, memo: &mut [Option<u64>]) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    if let Some(value) = memo[n] {
        return value;
    }

    let value = fib_memo(n - 1, memo) + fib_memo(n - 2, memo);
    memo[n] = Some(value);
    value
}

fn fib_tab(n: usize) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    let mut prev2 = 0u64;
    let mut prev1 = 1u64;

    for _ in 2..=n {
        let next = prev1 + prev2;
        prev2 = prev1;
        prev1 = next;
    }

    prev1
}

// ============================================================
// Classic Problems
// ============================================================

fn climb_stairs(n: usize) -> i32 {
    if n <= 2 {
        return n as i32;
    }

    let mut prev2 = 1;
    let mut prev1 = 2;
    for _ in 3..=n {
        let next = prev1 + prev2;
        prev2 = prev1;
        prev1 = next;
    }
    prev1
}

fn house_robber(nums: &[i32]) -> i32 {
    let mut include = 0;
    let mut exclude = 0;

    for &value in nums {
        let next_include = exclude + value;
        exclude = exclude.max(include);
        include = next_include;
    }

    include.max(exclude)
}

fn coin_change(coins: &[i32], amount: i32) -> i32 {
    let amount = amount as usize;
    let mut dp = vec![amount as i32 + 1; amount + 1];
    dp[0] = 0;

    for current in 1..=amount {
        for &coin in coins {
            let coin = coin as usize;
            if coin <= current {
                dp[current] = dp[current].min(dp[current - coin] + 1);
            }
        }
    }

    if dp[amount] > amount as i32 {
        -1
    } else {
        dp[amount]
    }
}

fn lis_length(nums: &[i32]) -> usize {
    let mut tails = Vec::new();

    for &num in nums {
        match tails.binary_search(&num) {
            Ok(index) => tails[index] = num,
            Err(index) => {
                if index == tails.len() {
                    tails.push(num);
                } else {
                    tails[index] = num;
                }
            }
        }
    }

    tails.len()
}

fn knapsack_01(weights: &[usize], values: &[i32], capacity: usize) -> i32 {
    let mut dp = vec![0i32; capacity + 1];

    for (&weight, &value) in weights.iter().zip(values.iter()) {
        for cap in (weight..=capacity).rev() {
            dp[cap] = dp[cap].max(dp[cap - weight] + value);
        }
    }

    dp[capacity]
}

fn edit_distance(a: &str, b: &str) -> usize {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut dp = vec![vec![0usize; b.len() + 1]; a.len() + 1];

    for (i, row) in dp.iter_mut().enumerate() {
        row[0] = i;
    }
    for j in 0..=b.len() {
        dp[0][j] = j;
    }

    for i in 1..=a.len() {
        for j in 1..=b.len() {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1]
                    .min(dp[i - 1][j])
                    .min(dp[i][j - 1]);
            }
        }
    }

    dp[a.len()][b.len()]
}
