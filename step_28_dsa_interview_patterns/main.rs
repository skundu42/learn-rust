// ============================================================
// STEP 28: DSA — Interview Patterns
// ============================================================
// Run: rustc --edition 2021 main.rs && ./main
//
// This step focuses on high-frequency interview patterns that show
// up across many problems, even when the surface story changes.
// ============================================================

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap};

fn main() {
    // =========================
    // 28.1 Prefix Sum + HashMap
    // =========================

    println!("--- Prefix Sum + HashMap ---");

    assert_eq!(subarray_sum_equals_k(&[1, 1, 1], 2), 2);
    assert_eq!(subarray_sum_equals_k(&[1, 2, 3], 3), 2);
    println!("  Tests passed.");

    // =========================
    // 28.2 Binary Search on Answer
    // =========================

    println!("\n--- Binary Search on Answer ---");

    assert_eq!(min_eating_speed(&[3, 6, 7, 11], 8), 4);
    assert_eq!(min_eating_speed(&[30, 11, 23, 4, 20], 6), 23);
    println!("  Tests passed.");

    // =========================
    // 28.3 Interval Scheduling
    // =========================

    println!("\n--- Interval Scheduling / Meeting Rooms ---");

    let meetings = vec![(0, 30), (5, 10), (15, 20)];
    assert_eq!(min_meeting_rooms(&meetings), 2);

    let meetings = vec![(7, 10), (2, 4)];
    assert_eq!(min_meeting_rooms(&meetings), 1);
    println!("  Tests passed.");

    // =========================
    // 28.4 Backtracking
    // =========================

    println!("\n--- Backtracking: Combination Sum ---");

    let combos = combination_sum(&[2, 3, 6, 7], 7);
    assert_eq!(combos, vec![vec![2, 2, 3], vec![7]]);
    println!("  Combinations: {:?}", combos);

    // =========================
    // 28.5 Greedy
    // =========================

    println!("\n--- Greedy: Jump Game ---");

    assert!(can_jump(&[2, 3, 1, 1, 4]));
    assert!(!can_jump(&[3, 2, 1, 0, 4]));
    assert_eq!(min_jumps(&[2, 3, 1, 1, 4]), 2);
    println!("  Tests passed.");

    // =========================
    // 28.6 Trie / Prefix Search
    // =========================

    println!("\n--- Trie / Prefix Search ---");

    let mut trie = Trie::new();
    for word in ["rust", "rule", "runner", "graph", "grape"] {
        trie.insert(word);
    }

    assert!(trie.search("rust"));
    assert!(!trie.search("run"));
    assert!(trie.starts_with("ru"));
    assert_eq!(
        trie.suggestions("gr", 3),
        vec!["grape".to_string(), "graph".to_string()]
    );
    println!("  Trie tests passed.");

    println!("\nPopular interview patterns to recognize quickly:");
    println!("  - Prefix sum + hash map: subarray count problems");
    println!("  - Binary search on answer: optimize a monotonic condition");
    println!("  - Backtracking: subsets, combinations, word search");
    println!("  - Greedy: jump game, interval selection, task scheduling");
    println!("  - Trie: autocomplete, dictionary search, word break variants");

    println!("\n--- Step 28 Complete! ---");
    println!("Next: step_29 — Advanced Iterators");
}

// ============================================================
// Prefix Sum + HashMap
// ============================================================

fn subarray_sum_equals_k(nums: &[i32], k: i32) -> i32 {
    let mut prefix = 0;
    let mut count = 0;
    let mut seen: HashMap<i32, i32> = HashMap::new();
    seen.insert(0, 1);

    for &num in nums {
        prefix += num;
        if let Some(&matches) = seen.get(&(prefix - k)) {
            count += matches;
        }
        *seen.entry(prefix).or_insert(0) += 1;
    }

    count
}

// ============================================================
// Binary Search on Answer
// ============================================================

fn min_eating_speed(piles: &[i32], h: i32) -> i32 {
    let mut left = 1;
    let mut right = *piles.iter().max().unwrap();

    while left < right {
        let mid = left + (right - left) / 2;
        if hours_needed(piles, mid) <= h as i64 {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn hours_needed(piles: &[i32], speed: i32) -> i64 {
    piles
        .iter()
        .map(|&pile| ((pile as i64) + speed as i64 - 1) / speed as i64)
        .sum()
}

// ============================================================
// Interval Scheduling
// ============================================================

fn min_meeting_rooms(intervals: &[(i32, i32)]) -> usize {
    if intervals.is_empty() {
        return 0;
    }

    let mut intervals = intervals.to_vec();
    intervals.sort_unstable_by_key(|interval| interval.0);

    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut max_rooms = 0;

    for (start, end) in intervals {
        while let Some(&Reverse(earliest_end)) = heap.peek() {
            if earliest_end <= start {
                heap.pop();
            } else {
                break;
            }
        }

        heap.push(Reverse(end));
        max_rooms = max_rooms.max(heap.len());
    }

    max_rooms
}

// ============================================================
// Backtracking
// ============================================================

fn combination_sum(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates.to_vec();
    candidates.sort_unstable();

    let mut result = Vec::new();
    let mut current = Vec::new();
    combination_sum_dfs(&candidates, target, 0, &mut current, &mut result);
    result
}

fn combination_sum_dfs(
    candidates: &[i32],
    target: i32,
    start: usize,
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if target == 0 {
        result.push(current.clone());
        return;
    }

    for i in start..candidates.len() {
        let value = candidates[i];
        if value > target {
            break;
        }

        current.push(value);
        combination_sum_dfs(candidates, target - value, i, current, result);
        current.pop();
    }
}

// ============================================================
// Greedy
// ============================================================

fn can_jump(nums: &[i32]) -> bool {
    let mut farthest = 0usize;

    for (i, &jump) in nums.iter().enumerate() {
        if i > farthest {
            return false;
        }
        farthest = farthest.max(i + jump as usize);
    }

    true
}

fn min_jumps(nums: &[i32]) -> i32 {
    if nums.len() <= 1 {
        return 0;
    }

    let mut jumps = 0;
    let mut current_end = 0usize;
    let mut farthest = 0usize;

    for i in 0..nums.len() - 1 {
        farthest = farthest.max(i + nums[i] as usize);
        if i == current_end {
            jumps += 1;
            current_end = farthest;
        }
    }

    jumps
}

// ============================================================
// Trie
// ============================================================

#[derive(Default)]
struct TrieNode {
    children: BTreeMap<char, TrieNode>,
    is_end: bool,
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_default();
        }
        node.is_end = true;
    }

    fn search(&self, word: &str) -> bool {
        self.find_node(word).is_some_and(|node| node.is_end)
    }

    fn starts_with(&self, prefix: &str) -> bool {
        self.find_node(prefix).is_some()
    }

    fn suggestions(&self, prefix: &str, limit: usize) -> Vec<String> {
        let mut results = Vec::new();
        if let Some(node) = self.find_node(prefix) {
            let mut current = prefix.to_string();
            collect_words(node, &mut current, limit, &mut results);
        }
        results
    }

    fn find_node<'a>(&'a self, text: &str) -> Option<&'a TrieNode> {
        let mut node = &self.root;
        for ch in text.chars() {
            node = node.children.get(&ch)?;
        }
        Some(node)
    }
}

fn collect_words(node: &TrieNode, current: &mut String, limit: usize, out: &mut Vec<String>) {
    if out.len() >= limit {
        return;
    }

    if node.is_end {
        out.push(current.clone());
    }

    for (ch, child) in &node.children {
        current.push(*ch);
        collect_words(child, current, limit, out);
        current.pop();
        if out.len() >= limit {
            return;
        }
    }
}
