// ============================================================
// STEP 22: DSA — Linked Lists in Rust
// ============================================================
// Run: rustc main.rs && ./main
//
// Linked lists are HARD in Rust due to ownership rules.
// This is intentional — they teach you to think about memory.
// We'll implement singly and doubly linked lists, plus classic problems.
// ============================================================

use std::fmt;

fn main() {
    // =========================
    // 22.1 Singly Linked List
    // =========================

    println!("--- Singly Linked List ---");

    let mut list = SinglyLinkedList::new();
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    list.push_back(4);
    list.push_back(5);
    println!("List: {}", list);
    println!("Length: {}", list.len());

    println!("Pop front: {:?}", list.pop_front());
    println!("After pop: {}", list);

    println!("Contains 3? {}", list.contains(&3));
    println!("Contains 9? {}", list.contains(&9));

    // Iterate:
    print!("Iter: ");
    for val in list.iter() {
        print!("{} ", val);
    }
    println!();

    // =========================
    // 22.2 Reverse a Linked List
    // =========================

    println!("\n--- Reverse ---");

    let mut list = SinglyLinkedList::new();
    for i in 1..=5 {
        list.push_back(i);
    }
    println!("Before: {}", list);
    list.reverse();
    println!("After:  {}", list);

    // =========================
    // 22.3 Detect Cycle (Floyd's Algorithm)
    // =========================

    println!("\n--- Cycle Detection ---");
    // We'll use a vec-based simulation since creating actual cycles
    // in safe Rust is very difficult (and that's the point!)
    println!("  Floyd's Tortoise and Hare:");
    assert!(has_cycle(&[1, 2, 3, 4, 2]));  // 4 -> 2 (cycle)
    assert!(!has_cycle(&[1, 2, 3, 4, 0])); // no cycle (0 = null)
    println!("  Cycle detection tests passed!");

    // =========================
    // 22.4 Merge Two Sorted Lists
    // =========================

    println!("\n--- Merge Sorted Lists ---");

    let mut l1 = SinglyLinkedList::new();
    let mut l2 = SinglyLinkedList::new();
    for &v in &[1, 3, 5, 7] {
        l1.push_back(v);
    }
    for &v in &[2, 4, 6, 8] {
        l2.push_back(v);
    }
    println!("L1: {}", l1);
    println!("L2: {}", l2);

    let merged = SinglyLinkedList::merge_sorted(l1, l2);
    println!("Merged: {}", merged);

    // =========================
    // 22.5 Find Middle Element
    // =========================

    println!("\n--- Middle Element ---");

    let mut list = SinglyLinkedList::new();
    for i in 1..=5 {
        list.push_back(i);
    }
    println!("List: {}", list);
    println!("Middle: {:?}", list.middle());

    let mut list = SinglyLinkedList::new();
    for i in 1..=6 {
        list.push_back(i);
    }
    println!("List: {}", list);
    println!("Middle: {:?}", list.middle());

    // =========================
    // 22.6 Remove Nth from End
    // =========================

    println!("\n--- Remove Nth from End ---");

    let mut list = SinglyLinkedList::new();
    for i in 1..=5 {
        list.push_back(i);
    }
    println!("Before: {}", list);
    list.remove_nth_from_end(2);
    println!("Remove 2nd from end: {}", list); // removes 4

    // =========================
    // 22.7 Check Palindrome
    // =========================

    println!("\n--- Palindrome Check ---");

    let cases = vec![
        vec![1, 2, 3, 2, 1],
        vec![1, 2, 2, 1],
        vec![1, 2, 3],
    ];

    for case in cases {
        let mut list = SinglyLinkedList::new();
        for &v in &case {
            list.push_back(v);
        }
        println!("  {} -> palindrome? {}", list, list.is_palindrome());
    }

    // =========================
    // 22.8 Stack-Allocated List (Array-Based)
    // =========================

    println!("\n--- Array-Based Linked List ---");
    println!("  For competitive programming, array-based lists avoid");
    println!("  heap allocation overhead and ownership complexity.");

    let mut pool = ArrayLinkedList::new();
    pool.push_front(3);
    pool.push_front(2);
    pool.push_front(1);
    pool.push_back(4);
    pool.push_back(5);
    print!("  Array list: ");
    pool.print();

    println!("\n--- Step 22 Complete! ---");
}

// ============================================================
// Singly Linked List Implementation
// ============================================================

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

pub struct SinglyLinkedList<T> {
    head: Link<T>,
    len: usize,
}

impl<T: fmt::Display + Clone + PartialEq> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList { head: None, len: 0 }
    }

    pub fn push_front(&mut self, val: T) {
        let new_node = Box::new(Node {
            val,
            next: self.head.take(), // take ownership of current head
        });
        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn push_back(&mut self, val: T) {
        let new_node = Box::new(Node { val, next: None });

        // Navigate to the end
        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            current = &mut node.next;
        }
        *current = Some(new_node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.val
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn contains(&self, target: &T) -> bool {
        let mut current = &self.head;
        while let Some(ref node) = current {
            if &node.val == target {
                return true;
            }
            current = &node.next;
        }
        false
    }

    pub fn iter(&self) -> ListIter<T> {
        ListIter {
            current: self.head.as_deref(),
        }
    }

    // --- 22.2 Reverse ---
    pub fn reverse(&mut self) {
        let mut prev: Link<T> = None;
        let mut current = self.head.take();

        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }
        self.head = prev;
    }

    // --- 22.4 Merge two sorted lists ---
    pub fn merge_sorted(mut l1: Self, mut l2: Self) -> Self {
        let mut result = SinglyLinkedList::new();
        let mut tail: *mut Link<T> = &mut result.head;

        let mut a = l1.head.take();
        let mut b = l2.head.take();

        unsafe {
            while a.is_some() && b.is_some() {
                let a_val = &a.as_ref().unwrap().val;
                let b_val = &b.as_ref().unwrap().val;

                if a_val.to_string() <= b_val.to_string() {
                    let mut node = a.unwrap();
                    a = node.next.take();
                    *tail = Some(node);
                    tail = &mut (*tail).as_mut().unwrap().next;
                } else {
                    let mut node = b.unwrap();
                    b = node.next.take();
                    *tail = Some(node);
                    tail = &mut (*tail).as_mut().unwrap().next;
                }
                result.len += 1;
            }

            // Append remaining
            if a.is_some() {
                *tail = a;
            } else {
                *tail = b;
            }
        }

        // Count remaining
        let mut cur = unsafe { &*tail };
        while let Some(ref node) = cur {
            result.len += 1;
            cur = &node.next;
        }

        result
    }

    // --- 22.5 Middle element (slow/fast pointer) ---
    pub fn middle(&self) -> Option<&T> {
        let mut slow = &self.head;
        let mut fast = &self.head;

        while let Some(ref fast_node) = fast {
            if let Some(ref next) = fast_node.next {
                slow = &slow.as_ref().unwrap().next;
                fast = &next.next;
            } else {
                break;
            }
        }
        slow.as_ref().map(|n| &n.val)
    }

    // --- 22.6 Remove Nth from end ---
    pub fn remove_nth_from_end(&mut self, n: usize) {
        let len = self.len;
        if n > len {
            return;
        }
        let target = len - n;

        if target == 0 {
            self.pop_front();
            return;
        }

        let mut current = &mut self.head;
        for _ in 0..target - 1 {
            current = &mut current.as_mut().unwrap().next;
        }

        let next_next = current.as_mut().unwrap().next.as_mut().and_then(|n| n.next.take());
        current.as_mut().unwrap().next = next_next;
        self.len -= 1;
    }

    // --- 22.7 Palindrome check ---
    pub fn is_palindrome(&self) -> bool {
        let vals: Vec<T> = self.iter().cloned().collect();
        let n = vals.len();
        for i in 0..n / 2 {
            if vals[i] != vals[n - 1 - i] {
                return false;
            }
        }
        true
    }
}

// Iterator for the list
pub struct ListIter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.val
        })
    }
}

impl<T: fmt::Display + Clone + PartialEq> fmt::Display for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;
        let mut first = true;
        while let Some(ref node) = current {
            if !first {
                write!(f, " -> ")?;
            }
            write!(f, "{}", node.val)?;
            first = false;
            current = &node.next;
        }
        Ok(())
    }
}

// --- 22.3 Cycle Detection (simulated with vec) ---

fn has_cycle(next_indices: &[usize]) -> bool {
    // next_indices[i] = index of next node. 0 means null/end.
    if next_indices.is_empty() {
        return false;
    }
    let mut slow = 0;
    let mut fast = 0;

    loop {
        // Move slow by 1
        slow = next_indices[slow];
        if slow == 0 {
            return false;
        }

        // Move fast by 2
        fast = next_indices[fast];
        if fast == 0 {
            return false;
        }
        fast = next_indices[fast];
        if fast == 0 {
            return false;
        }

        if slow == fast {
            return true;
        }
    }
}

// --- 22.8 Array-Based Linked List ---

const MAX_NODES: usize = 1024;

struct ArrayLinkedList {
    values: [i32; MAX_NODES],
    next: [i32; MAX_NODES], // -1 = null
    head: i32,
    free: usize, // next free slot
}

impl ArrayLinkedList {
    fn new() -> Self {
        ArrayLinkedList {
            values: [0; MAX_NODES],
            next: [-1; MAX_NODES],
            head: -1,
            free: 0,
        }
    }

    fn alloc(&mut self) -> usize {
        let idx = self.free;
        self.free += 1;
        idx
    }

    fn push_front(&mut self, val: i32) {
        let idx = self.alloc();
        self.values[idx] = val;
        self.next[idx] = self.head;
        self.head = idx as i32;
    }

    fn push_back(&mut self, val: i32) {
        let idx = self.alloc();
        self.values[idx] = val;
        self.next[idx] = -1;

        if self.head == -1 {
            self.head = idx as i32;
            return;
        }

        let mut cur = self.head as usize;
        while self.next[cur] != -1 {
            cur = self.next[cur] as usize;
        }
        self.next[cur] = idx as i32;
    }

    fn print(&self) {
        let mut cur = self.head;
        while cur != -1 {
            print!("{}", self.values[cur as usize]);
            cur = self.next[cur as usize];
            if cur != -1 {
                print!(" -> ");
            }
        }
        println!();
    }
}
