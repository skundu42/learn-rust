// ============================================================
// STEP 29: Advanced Iterators & Pipelines
// ============================================================
// Run: rustc --edition 2021 main.rs && ./main
//
// Step 11 introduced the iterator basics.
// This step goes further into:
// - custom iterators
// - IntoIterator for your own types
// - stateful adapters like scan
// - fallible pipelines with try_fold
// - custom iterator adapters
// ============================================================

fn main() {
    // =========================
    // 29.1 Custom Iterator
    // =========================

    println!("--- Custom Iterator: Fibonacci ---");

    let fib: Vec<u64> = Fibonacci::new().take(8).collect();
    assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13]);
    println!("  First 8 Fibonacci numbers: {:?}", fib);

    // =========================
    // 29.2 IntoIterator for Your Own Type
    // =========================

    println!("\n--- IntoIterator for Custom Collections ---");

    let board = ScoreBoard::from(vec![("Alice", 12), ("Bob", 18), ("Carol", 15)]);
    let names: Vec<&str> = (&board).into_iter().map(|entry| entry.name.as_str()).collect();
    let total_points: u32 = board.clone().into_iter().map(|entry| entry.points).sum();

    assert_eq!(names, vec!["Alice", "Bob", "Carol"]);
    assert_eq!(total_points, 45);
    println!("  Names: {:?}", names);
    println!("  Total points: {}", total_points);

    // =========================
    // 29.3 Stateful Adapters
    // =========================

    println!("\n--- Stateful Adapters: scan / map_while ---");

    let running_totals: Vec<i32> = [3, 1, 4, 1, 5]
        .into_iter()
        .scan(0, |sum, value| {
            *sum += value;
            Some(*sum)
        })
        .collect();
    assert_eq!(running_totals, vec![3, 4, 8, 9, 14]);
    println!("  Running totals: {:?}", running_totals);

    let prefix_before_negative: Vec<i32> = [2, 4, 6, -1, 8]
        .into_iter()
        .map_while(|&value| if value >= 0 { Some(value) } else { None })
        .collect();
    assert_eq!(prefix_before_negative, vec![2, 4, 6]);
    println!("  Prefix before negative: {:?}", prefix_before_negative);

    // =========================
    // 29.4 Peekable
    // =========================

    println!("\n--- Peekable: Run-Length Encoding ---");

    let runs = compress_runs("aaabbccccdaa");
    assert_eq!(runs, vec![('a', 3), ('b', 2), ('c', 4), ('d', 1), ('a', 2)]);
    println!("  Runs: {:?}", runs);

    // =========================
    // 29.5 Fallible Pipelines
    // =========================

    println!("\n--- try_fold: Fallible Pipelines ---");

    assert_eq!(checked_sum(["10", "20", "30"]), Some(60));
    assert_eq!(checked_sum(["10", "oops", "30"]), None);
    println!("  Checked sum tests passed.");

    // =========================
    // 29.6 flat_map / partition
    // =========================

    println!("\n--- flat_map / partition ---");

    let sentences = ["rust makes iterators ergonomic", "lazy pipelines stay composable"];
    let words: Vec<&str> = sentences
        .iter()
        .flat_map(|line| line.split_whitespace())
        .collect();
    let (short_words, long_words): (Vec<&str>, Vec<&str>) =
        words.into_iter().partition(|word| word.len() <= 4);

    println!("  Short words: {:?}", short_words);
    println!("  Long words:  {:?}", long_words);

    // =========================
    // 29.7 Custom Iterator Adapter
    // =========================

    println!("\n--- Custom Adapter: Adjacent Pairs ---");

    let nums = [10, 20, 30, 40];
    let diffs: Vec<i32> = AdjacentPairs::new(&nums).map(|(a, b)| b - a).collect();
    assert_eq!(diffs, vec![10, 10, 10]);

    let reverse_pairs: Vec<(i32, i32)> = AdjacentPairs::new(&nums)
        .rev()
        .map(|(a, b)| (*a, *b))
        .collect();
    assert_eq!(reverse_pairs, vec![(30, 40), (20, 30), (10, 20)]);

    println!("  Adjacent diffs: {:?}", diffs);
    println!("  Reverse pairs: {:?}", reverse_pairs);

    // =========================
    // 29.8 Mini Analytics Pipeline
    // =========================

    println!("\n--- Mini Analytics Pipeline ---");

    let logs = [
        LogLine {
            user: "alice",
            latency_ms: 120,
            status: 200,
        },
        LogLine {
            user: "bob",
            latency_ms: 70,
            status: 200,
        },
        LogLine {
            user: "carol",
            latency_ms: 180,
            status: 503,
        },
        LogLine {
            user: "dave",
            latency_ms: 95,
            status: 200,
        },
    ];

    let alerts: Vec<&str> = logs
        .iter()
        .filter(|log| log.status >= 500 || log.latency_ms >= 120)
        .map(|log| log.user)
        .collect();
    assert_eq!(alerts, vec!["alice", "carol"]);

    let average_latency = average_latency(&logs).unwrap();
    println!("  Alert users: {:?}", alerts);
    println!("  Average latency: {:.1}ms", average_latency);

    println!("\n--- Step 29 Complete! ---");
    println!("Next: step_30 — DSA Capstone");
}

// ============================================================
// Custom Iterator
// ============================================================

struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current;
        self.current = self.next;
        self.next += value;
        Some(value)
    }
}

// ============================================================
// IntoIterator for Custom Types
// ============================================================

#[derive(Debug, Clone)]
struct Score {
    name: String,
    points: u32,
}

#[derive(Debug, Clone)]
struct ScoreBoard {
    entries: Vec<Score>,
}

impl From<Vec<(&str, u32)>> for ScoreBoard {
    fn from(entries: Vec<(&str, u32)>) -> Self {
        ScoreBoard {
            entries: entries
                .into_iter()
                .map(|(name, points)| Score {
                    name: name.to_string(),
                    points,
                })
                .collect(),
        }
    }
}

impl<'a> IntoIterator for &'a ScoreBoard {
    type Item = &'a Score;
    type IntoIter = std::slice::Iter<'a, Score>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

impl IntoIterator for ScoreBoard {
    type Item = Score;
    type IntoIter = std::vec::IntoIter<Score>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

// ============================================================
// Peekable / try_fold Helpers
// ============================================================

fn compress_runs(input: &str) -> Vec<(char, usize)> {
    let mut chars = input.chars().peekable();
    let mut runs = Vec::new();

    while let Some(ch) = chars.next() {
        let mut count = 1;
        while chars.peek() == Some(&ch) {
            chars.next();
            count += 1;
        }
        runs.push((ch, count));
    }

    runs
}

fn checked_sum<I>(items: I) -> Option<i32>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    items.into_iter().try_fold(0i32, |acc, item| {
        let value: i32 = item.as_ref().parse().ok()?;
        acc.checked_add(value)
    })
}

// ============================================================
// Custom Iterator Adapter
// ============================================================

struct AdjacentPairs<'a, T> {
    slice: &'a [T],
    front: usize,
    back: usize,
}

impl<'a, T> AdjacentPairs<'a, T> {
    fn new(slice: &'a [T]) -> Self {
        if slice.len() < 2 {
            AdjacentPairs {
                slice,
                front: 1,
                back: 0,
            }
        } else {
            AdjacentPairs {
                slice,
                front: 0,
                back: slice.len() - 2,
            }
        }
    }
}

impl<'a, T> Iterator for AdjacentPairs<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.front > self.back {
            return None;
        }

        let index = self.front;
        self.front += 1;
        Some((&self.slice[index], &self.slice[index + 1]))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, T> DoubleEndedIterator for AdjacentPairs<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front > self.back {
            return None;
        }

        let index = self.back;
        if self.back == 0 {
            self.front = 1;
        } else {
            self.back -= 1;
        }
        Some((&self.slice[index], &self.slice[index + 1]))
    }
}

impl<'a, T> ExactSizeIterator for AdjacentPairs<'a, T> {
    fn len(&self) -> usize {
        if self.front > self.back {
            0
        } else {
            self.back - self.front + 1
        }
    }
}

// ============================================================
// Mini Analytics Example
// ============================================================

struct LogLine {
    user: &'static str,
    latency_ms: u32,
    status: u16,
}

fn average_latency(logs: &[LogLine]) -> Option<f64> {
    if logs.is_empty() {
        return None;
    }

    let total: u32 = logs.iter().map(|log| log.latency_ms).sum();
    Some(total as f64 / logs.len() as f64)
}
