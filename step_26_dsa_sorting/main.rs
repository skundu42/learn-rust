// ============================================================
// STEP 26: DSA — Sorting & Selection
// ============================================================
// Run: rustc --edition 2021 main.rs && ./main
//
// Sorting is everywhere in interview problems:
// - ordering data
// - merging intervals
// - selecting kth elements
// - preparing for binary search
// ============================================================

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    // =========================
    // 26.1 Insertion Sort
    // =========================

    println!("--- Insertion Sort ---");

    let mut nearly_sorted = vec![1, 2, 4, 3, 5, 6];
    insertion_sort(&mut nearly_sorted);
    assert_eq!(nearly_sorted, vec![1, 2, 3, 4, 5, 6]);
    println!("  Result: {:?}", nearly_sorted);

    // =========================
    // 26.2 Merge Sort
    // =========================

    println!("\n--- Merge Sort ---");

    let mut nums = vec![9, 3, 7, 1, 8, 2, 6, 5, 4];
    merge_sort(&mut nums);
    assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    println!("  Result: {:?}", nums);

    // =========================
    // 26.3 Quick Sort
    // =========================

    println!("\n--- Quick Sort ---");

    let mut nums = vec![10, -1, 3, 7, 7, 2, 0];
    quick_sort(&mut nums);
    assert_eq!(nums, vec![-1, 0, 2, 3, 7, 7, 10]);
    println!("  Result: {:?}", nums);

    // =========================
    // 26.4 Heap Sort
    // =========================

    println!("\n--- Heap Sort ---");

    let mut nums = vec![5, 1, 4, 2, 8, 0];
    heap_sort(&mut nums);
    assert_eq!(nums, vec![0, 1, 2, 4, 5, 8]);
    println!("  Result: {:?}", nums);

    // =========================
    // 26.5 Counting Sort
    // =========================

    println!("\n--- Counting Sort ---");

    let mut nums = vec![4usize, 2, 2, 8, 3, 3, 1];
    counting_sort(&mut nums);
    assert_eq!(nums, vec![1, 2, 2, 3, 3, 4, 8]);
    println!("  Result: {:?}", nums);

    // =========================
    // 26.6 Quickselect
    // =========================

    println!("\n--- Quickselect (kth smallest) ---");

    let mut nums = vec![7, 10, 4, 3, 20, 15];
    let third_smallest = quickselect(&mut nums, 2);
    assert_eq!(third_smallest, 7);
    println!("  3rd smallest element: {}", third_smallest);

    // =========================
    // 26.7 Merge Intervals
    // =========================

    println!("\n--- Merge Intervals ---");

    let mut intervals = vec![(1, 3), (2, 6), (8, 10), (15, 18)];
    let merged = merge_intervals(&mut intervals);
    assert_eq!(merged, vec![(1, 6), (8, 10), (15, 18)]);
    println!("  Merged: {:?}", merged);

    // =========================
    // 26.8 Rust's Built-in Sort APIs
    // =========================

    println!("\n--- Built-in Sort APIs ---");

    let mut students = vec![("Asha", 90), ("Ben", 80), ("Chloe", 90), ("Dinesh", 85)];
    students.sort_by_key(|(_, score)| Reverse(*score));
    println!("  Stable sort by descending score: {:?}", students);

    let mut fast_sort = vec![9, 1, 8, 2, 7, 3];
    fast_sort.sort_unstable();
    println!("  sort_unstable(): {:?}", fast_sort);

    println!("\nPopular sorting interview questions:");
    println!("  - Merge intervals");
    println!("  - Kth largest / kth smallest");
    println!("  - Sort characters or colors by category");
    println!("  - Meeting rooms / interval scheduling");
    println!("  - Top K using heap vs sort tradeoffs");

    println!("\n--- Step 26 Complete! ---");
    println!("Next: step_27 — Dynamic Programming");
}

// ============================================================
// Basic Sorting Algorithms
// ============================================================

fn insertion_sort(nums: &mut [i32]) {
    for i in 1..nums.len() {
        let key = nums[i];
        let mut j = i;
        while j > 0 && nums[j - 1] > key {
            nums[j] = nums[j - 1];
            j -= 1;
        }
        nums[j] = key;
    }
}

fn merge_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    let mid = nums.len() / 2;
    merge_sort(&mut nums[..mid]);
    merge_sort(&mut nums[mid..]);

    let mut merged = nums.to_vec();
    merge(&nums[..mid], &nums[mid..], &mut merged);
    nums.copy_from_slice(&merged);
}

fn merge(left: &[i32], right: &[i32], out: &mut [i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            out[k] = left[i];
            i += 1;
        } else {
            out[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        out[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        out[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn quick_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    let pivot = partition(nums);
    let (left, right) = nums.split_at_mut(pivot);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn partition(nums: &mut [i32]) -> usize {
    let pivot_index = nums.len() - 1;
    let pivot = nums[pivot_index];
    let mut store = 0;

    for i in 0..pivot_index {
        if nums[i] <= pivot {
            nums.swap(i, store);
            store += 1;
        }
    }

    nums.swap(store, pivot_index);
    store
}

fn heap_sort(nums: &mut [i32]) {
    let mut heap = BinaryHeap::from(nums.to_vec());
    for slot in nums.iter_mut().rev() {
        *slot = heap.pop().unwrap();
    }
}

fn counting_sort(nums: &mut [usize]) {
    if nums.is_empty() {
        return;
    }

    let max_value = *nums.iter().max().unwrap();
    let mut counts = vec![0usize; max_value + 1];

    for &value in nums.iter() {
        counts[value] += 1;
    }

    let mut write = 0;
    for (value, count) in counts.into_iter().enumerate() {
        for _ in 0..count {
            nums[write] = value;
            write += 1;
        }
    }
}

// ============================================================
// Selection
// ============================================================

fn quickselect(nums: &mut [i32], k: usize) -> i32 {
    assert!(k < nums.len(), "k out of bounds");

    let mut left = 0usize;
    let mut right = nums.len() - 1;

    loop {
        let pivot = partition_range(nums, left, right);
        if pivot == k {
            return nums[pivot];
        }
        if pivot < k {
            left = pivot + 1;
        } else if pivot == 0 {
            return nums[0];
        } else {
            right = pivot - 1;
        }
    }
}

fn partition_range(nums: &mut [i32], left: usize, right: usize) -> usize {
    let pivot = nums[right];
    let mut store = left;

    for i in left..right {
        if nums[i] <= pivot {
            nums.swap(i, store);
            store += 1;
        }
    }

    nums.swap(store, right);
    store
}

// ============================================================
// Interval Problems
// ============================================================

fn merge_intervals(intervals: &mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    if intervals.is_empty() {
        return Vec::new();
    }

    intervals.sort_unstable_by_key(|interval| interval.0);

    let mut merged = vec![intervals[0]];
    for &(start, end) in intervals.iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if start <= last.1 {
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }

    merged
}
