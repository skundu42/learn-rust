// ============================================================
// STEP 23: DSA — Stacks, Queues & Deques
// ============================================================
// Run: rustc main.rs && ./main
//
// Stacks (LIFO) and Queues (FIFO) are fundamental.
// Covers: classic problems, monotonic stacks, and priority queues.
// ============================================================

use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::cmp::Reverse;

fn main() {
    // =========================
    // 23.1 Stack (Vec-based)
    // =========================

    println!("--- Stack ---");

    let mut stack: Vec<i32> = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("Stack: {:?}", stack);
    println!("Top: {:?}", stack.last());
    println!("Pop: {:?}", stack.pop());
    println!("After: {:?}", stack);

    // =========================
    // 23.2 Valid Parentheses
    // =========================

    println!("\n--- Valid Parentheses ---");
    assert!(is_valid_parens("()[]{}"));
    assert!(is_valid_parens("({[]})"));
    assert!(!is_valid_parens("(]"));
    assert!(!is_valid_parens("([)]"));
    assert!(is_valid_parens(""));
    println!("  All tests passed!");

    // =========================
    // 23.3 Min Stack
    // =========================

    println!("\n--- Min Stack ---");

    let mut min_stack = MinStack::new();
    min_stack.push(5);
    min_stack.push(3);
    min_stack.push(7);
    min_stack.push(1);
    println!("Min: {}", min_stack.get_min().unwrap()); // 1
    min_stack.pop();
    println!("Min after pop: {}", min_stack.get_min().unwrap()); // 3
    min_stack.pop();
    println!("Min after pop: {}", min_stack.get_min().unwrap()); // 3

    // =========================
    // 23.4 Evaluate Reverse Polish Notation
    // =========================

    println!("\n--- Reverse Polish Notation ---");

    let tokens = vec!["2", "1", "+", "3", "*"];
    assert_eq!(eval_rpn(&tokens), 9); // (2+1)*3
    let tokens = vec!["4", "13", "5", "/", "+"];
    assert_eq!(eval_rpn(&tokens), 6); // 4+(13/5)
    println!("  All tests passed!");

    // =========================
    // 23.5 Monotonic Stack — Next Greater Element
    // =========================

    println!("\n--- Next Greater Element ---");

    let result = next_greater_element(&[4, 5, 2, 25]);
    println!("  [4,5,2,25] -> {:?}", result); // [5, 25, 25, -1]

    let result = next_greater_element(&[13, 7, 6, 12]);
    println!("  [13,7,6,12] -> {:?}", result); // [-1, 12, 12, -1]

    // =========================
    // 23.6 Daily Temperatures
    // =========================

    println!("\n--- Daily Temperatures ---");
    let temps = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let result = daily_temperatures(&temps);
    println!("  Temps:  {:?}", temps);
    println!("  Result: {:?}", result); // [1,1,4,2,1,1,0,0]

    // =========================
    // 23.7 Queue (VecDeque-based)
    // =========================

    println!("\n--- Queue ---");

    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);
    println!("Queue: {:?}", queue);
    println!("Front: {:?}", queue.front());
    println!("Dequeue: {:?}", queue.pop_front());
    println!("After: {:?}", queue);

    // =========================
    // 23.8 Implement Queue using Two Stacks
    // =========================

    println!("\n--- Queue from Two Stacks ---");

    let mut q = StackQueue::new();
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);
    println!("Dequeue: {}", q.dequeue().unwrap()); // 1
    q.enqueue(4);
    println!("Dequeue: {}", q.dequeue().unwrap()); // 2
    println!("Dequeue: {}", q.dequeue().unwrap()); // 3
    println!("Dequeue: {}", q.dequeue().unwrap()); // 4

    // =========================
    // 23.9 Sliding Window Maximum
    // =========================

    println!("\n--- Sliding Window Maximum ---");
    let result = max_sliding_window(&[1, 3, -1, -3, 5, 3, 6, 7], 3);
    println!("  Result: {:?}", result); // [3, 3, 5, 5, 6, 7]
    assert_eq!(result, vec![3, 3, 5, 5, 6, 7]);
    println!("  Test passed!");

    // =========================
    // 23.10 Priority Queue (BinaryHeap)
    // =========================

    println!("\n--- Priority Queue ---");

    // Rust's BinaryHeap is a MAX-heap
    let mut heap = BinaryHeap::new();
    heap.push(3);
    heap.push(1);
    heap.push(4);
    heap.push(1);
    heap.push(5);

    print!("Max-heap order: ");
    while let Some(val) = heap.pop() {
        print!("{} ", val); // 5 4 3 1 1
    }
    println!();

    // Min-heap using Reverse:
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(3));
    min_heap.push(Reverse(1));
    min_heap.push(Reverse(4));

    print!("Min-heap order: ");
    while let Some(Reverse(val)) = min_heap.pop() {
        print!("{} ", val); // 1 3 4
    }
    println!();

    // =========================
    // 23.11 Kth Largest Element
    // =========================

    println!("\n--- Kth Largest ---");
    assert_eq!(kth_largest(&[3, 2, 1, 5, 6, 4], 2), 5);
    assert_eq!(kth_largest(&[3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    println!("  All tests passed!");

    // =========================
    // 23.12 Top K Frequent Elements
    // =========================

    println!("\n--- Top K Frequent ---");
    let result = top_k_frequent(&[1, 1, 1, 2, 2, 3], 2);
    println!("  Top 2 frequent of [1,1,1,2,2,3]: {:?}", result);

    // =========================
    // 23.13 Largest Rectangle in Histogram
    // =========================

    println!("\n--- Largest Rectangle in Histogram ---");
    assert_eq!(largest_rectangle_histogram(&[2, 1, 5, 6, 2, 3]), 10);
    assert_eq!(largest_rectangle_histogram(&[2, 4]), 4);
    println!("  All tests passed!");

    println!("\n--- Step 23 Complete! ---");
}

// ============================================================
// Implementations
// ============================================================

// --- 23.2 Valid Parentheses ---

fn is_valid_parens(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

// --- 23.3 Min Stack ---

struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>, // tracks minimums
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        let min = self.min_stack.last().copied().unwrap_or(i32::MAX).min(val);
        self.min_stack.push(min);
    }

    fn pop(&mut self) -> Option<i32> {
        self.min_stack.pop();
        self.stack.pop()
    }

    fn get_min(&self) -> Option<i32> {
        self.min_stack.last().copied()
    }
}

// --- 23.4 Evaluate RPN ---

fn eval_rpn(tokens: &[&str]) -> i32 {
    let mut stack = Vec::new();

    for &token in tokens {
        match token {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            num => stack.push(num.parse().unwrap()),
        }
    }
    stack.pop().unwrap()
}

// --- 23.5 Next Greater Element ---

fn next_greater_element(nums: &[i32]) -> Vec<i32> {
    let mut result = vec![-1i32; nums.len()];
    let mut stack: Vec<usize> = Vec::new(); // stack of indices

    for i in 0..nums.len() {
        while !stack.is_empty() && nums[*stack.last().unwrap()] < nums[i] {
            let idx = stack.pop().unwrap();
            result[idx] = nums[i];
        }
        stack.push(i);
    }
    result
}

// --- 23.6 Daily Temperatures ---

fn daily_temperatures(temps: &[i32]) -> Vec<i32> {
    let mut result = vec![0; temps.len()];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..temps.len() {
        while !stack.is_empty() && temps[*stack.last().unwrap()] < temps[i] {
            let idx = stack.pop().unwrap();
            result[idx] = (i - idx) as i32;
        }
        stack.push(i);
    }
    result
}

// --- 23.8 Queue from Two Stacks ---

struct StackQueue<T> {
    inbox: Vec<T>,
    outbox: Vec<T>,
}

impl<T> StackQueue<T> {
    fn new() -> Self {
        StackQueue {
            inbox: Vec::new(),
            outbox: Vec::new(),
        }
    }

    fn enqueue(&mut self, val: T) {
        self.inbox.push(val);
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.outbox.is_empty() {
            while let Some(val) = self.inbox.pop() {
                self.outbox.push(val);
            }
        }
        self.outbox.pop()
    }
}

// --- 23.9 Sliding Window Maximum (Monotonic Deque) ---

fn max_sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
    let mut result = Vec::new();
    let mut deque: VecDeque<usize> = VecDeque::new(); // stores indices

    for i in 0..nums.len() {
        // Remove elements outside window
        while !deque.is_empty() && *deque.front().unwrap() + k <= i {
            deque.pop_front();
        }

        // Remove smaller elements from back
        while !deque.is_empty() && nums[*deque.back().unwrap()] <= nums[i] {
            deque.pop_back();
        }

        deque.push_back(i);

        if i >= k - 1 {
            result.push(nums[*deque.front().unwrap()]);
        }
    }
    result
}

// --- 23.11 Kth Largest ---

fn kth_largest(nums: &[i32], k: usize) -> i32 {
    // Min-heap of size k
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for &num in nums {
        heap.push(Reverse(num));
        if heap.len() > k {
            heap.pop();
        }
    }
    heap.peek().unwrap().0
}

// --- 23.12 Top K Frequent ---

fn top_k_frequent(nums: &[i32], k: usize) -> Vec<i32> {
    let mut freq: HashMap<i32, usize> = HashMap::new();
    for &num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut heap: BinaryHeap<(usize, i32)> = BinaryHeap::new();
    for (num, count) in freq {
        heap.push((count, num));
    }

    let mut result = Vec::new();
    for _ in 0..k {
        if let Some((_, num)) = heap.pop() {
            result.push(num);
        }
    }
    result
}

// --- 23.13 Largest Rectangle in Histogram ---

fn largest_rectangle_histogram(heights: &[i32]) -> i32 {
    let mut stack: Vec<usize> = Vec::new();
    let mut max_area = 0;
    let n = heights.len();

    for i in 0..=n {
        let current_height = if i == n { 0 } else { heights[i] };

        while !stack.is_empty() && heights[*stack.last().unwrap()] > current_height {
            let height = heights[stack.pop().unwrap()];
            let width = if stack.is_empty() {
                i
            } else {
                i - stack.last().unwrap() - 1
            };
            max_area = max_area.max(height * width as i32);
        }
        stack.push(i);
    }
    max_area
}
