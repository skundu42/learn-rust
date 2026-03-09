// ============================================================
// STEP 21: DSA — Arrays & Strings
// ============================================================
// Run: rustc main.rs && ./main
//
// Classic array/string problems implemented in idiomatic Rust.
// Covers: two pointers, sliding window, prefix sums, and more.
// ============================================================

fn main() {
    // =========================
    // 21.1 Two Sum (HashMap approach)
    // =========================

    println!("--- Two Sum ---");
    // Given nums and target, find two indices that sum to target.
    assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
    assert_eq!(two_sum(&[3, 2, 4], 6), Some((1, 2)));
    assert_eq!(two_sum(&[1, 2, 3], 10), None);
    println!("  All tests passed!");

    // =========================
    // 21.2 Two Pointers — Sorted Array
    // =========================

    println!("\n--- Two Pointers ---");
    // Two sum on a SORTED array using two pointers (no extra space).
    assert_eq!(two_sum_sorted(&[1, 2, 3, 4, 6], 6), Some((1, 3)));
    assert_eq!(two_sum_sorted(&[2, 3, 5, 8], 11), Some((1, 3)));
    println!("  All tests passed!");

    // Remove duplicates in-place:
    let mut arr = vec![1, 1, 2, 2, 3, 4, 4, 5];
    let new_len = remove_duplicates(&mut arr);
    println!("  Unique elements: {:?} (len={})", &arr[..new_len], new_len);

    // =========================
    // 21.3 Sliding Window
    // =========================

    println!("\n--- Sliding Window ---");

    // Max sum of k consecutive elements:
    assert_eq!(max_sum_subarray(&[2, 1, 5, 1, 3, 2], 3), 9); // [5,1,3]
    assert_eq!(max_sum_subarray(&[2, 3, 4, 1, 5], 2), 7);     // [3,4]
    println!("  Max sum tests passed!");

    // Longest substring without repeating characters:
    assert_eq!(longest_unique_substring("abcabcbb"), 3); // "abc"
    assert_eq!(longest_unique_substring("bbbbb"), 1);
    assert_eq!(longest_unique_substring("pwwkew"), 3);   // "wke"
    println!("  Longest unique substring tests passed!");

    // Minimum window substring:
    assert_eq!(min_window("ADOBECODEBANC", "ABC"), "BANC");
    assert_eq!(min_window("a", "a"), "a");
    println!("  Min window substring tests passed!");

    // =========================
    // 21.4 Prefix Sum
    // =========================

    println!("\n--- Prefix Sum ---");

    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let prefix = PrefixSum::new(&nums);
    assert_eq!(prefix.range_sum(0, 4), 15);  // 1+2+3+4+5
    assert_eq!(prefix.range_sum(3, 6), 22);  // 4+5+6+7
    assert_eq!(prefix.range_sum(0, 9), 55);  // all
    println!("  Prefix sum tests passed!");

    // =========================
    // 21.5 String Problems
    // =========================

    println!("\n--- String Problems ---");

    // Valid palindrome (ignore non-alphanumeric):
    assert!(is_palindrome("A man, a plan, a canal: Panama"));
    assert!(!is_palindrome("race a car"));
    assert!(is_palindrome("Was it a car or a cat I saw?"));
    println!("  Palindrome tests passed!");

    // Valid anagram:
    assert!(is_anagram("anagram", "nagaram"));
    assert!(!is_anagram("rat", "car"));
    assert!(is_anagram("listen", "silent"));
    println!("  Anagram tests passed!");

    // Group anagrams:
    let words = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
    let groups = group_anagrams(&words);
    println!("  Anagram groups:");
    for group in &groups {
        println!("    {:?}", group);
    }

    // Longest common prefix:
    assert_eq!(longest_common_prefix(&["flower", "flow", "flight"]), "fl");
    assert_eq!(longest_common_prefix(&["dog", "racecar", "car"]), "");
    println!("  Longest common prefix tests passed!");

    // =========================
    // 21.6 Kadane's Algorithm (Maximum Subarray)
    // =========================

    println!("\n--- Kadane's Algorithm ---");
    assert_eq!(max_subarray(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6); // [4,-1,2,1]
    assert_eq!(max_subarray(&[1]), 1);
    assert_eq!(max_subarray(&[-1, -2, -3]), -1);
    println!("  Max subarray tests passed!");

    // =========================
    // 21.7 Dutch National Flag (3-way partition)
    // =========================

    println!("\n--- Dutch National Flag ---");
    let mut colors = vec![2, 0, 2, 1, 1, 0];
    sort_colors(&mut colors);
    assert_eq!(colors, vec![0, 0, 1, 1, 2, 2]);
    println!("  Sort colors: {:?} -- passed!", colors);

    // =========================
    // 21.8 Next Permutation
    // =========================

    println!("\n--- Next Permutation ---");
    let mut perm = vec![1, 2, 3];
    next_permutation(&mut perm);
    assert_eq!(perm, vec![1, 3, 2]);

    let mut perm = vec![3, 2, 1];
    next_permutation(&mut perm);
    assert_eq!(perm, vec![1, 2, 3]); // wraps around

    let mut perm = vec![1, 1, 5];
    next_permutation(&mut perm);
    assert_eq!(perm, vec![1, 5, 1]);
    println!("  Next permutation tests passed!");

    // =========================
    // 21.9 Rotate Array
    // =========================

    println!("\n--- Rotate Array ---");
    let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
    rotate_right(&mut arr, 3);
    assert_eq!(arr, vec![5, 6, 7, 1, 2, 3, 4]);
    println!("  Rotate right by 3: {:?} -- passed!", arr);

    // =========================
    // 21.10 Trapping Rain Water
    // =========================

    println!("\n--- Trapping Rain Water ---");
    assert_eq!(trap_water(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(trap_water(&[4, 2, 0, 3, 2, 5]), 9);
    println!("  Trapping rain water tests passed!");

    println!("\n--- Step 21 Complete! ---");
}

// ============================================================
// Implementations
// ============================================================

// --- 21.1 Two Sum ---

fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    use std::collections::HashMap;
    let mut seen: HashMap<i32, usize> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = seen.get(&complement) {
            return Some((j, i));
        }
        seen.insert(num, i);
    }
    None
}

// --- 21.2 Two Pointers ---

fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let (mut left, mut right) = (0, nums.len() - 1);

    while left < right {
        let sum = nums[left] + nums[right];
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    None
}

fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
    if nums.is_empty() {
        return 0;
    }
    let mut write = 1;
    for read in 1..nums.len() {
        if nums[read] != nums[write - 1] {
            nums[write] = nums[read];
            write += 1;
        }
    }
    write
}

// --- 21.3 Sliding Window ---

fn max_sum_subarray(nums: &[i32], k: usize) -> i32 {
    let mut window_sum: i32 = nums[..k].iter().sum();
    let mut max_sum = window_sum;

    for i in k..nums.len() {
        window_sum += nums[i] - nums[i - k];
        max_sum = max_sum.max(window_sum);
    }
    max_sum
}

fn longest_unique_substring(s: &str) -> usize {
    use std::collections::HashMap;
    let chars: Vec<char> = s.chars().collect();
    let mut last_seen: HashMap<char, usize> = HashMap::new();
    let mut max_len = 0;
    let mut start = 0;

    for (i, &c) in chars.iter().enumerate() {
        if let Some(&prev) = last_seen.get(&c) {
            start = start.max(prev + 1);
        }
        last_seen.insert(c, i);
        max_len = max_len.max(i - start + 1);
    }
    max_len
}

fn min_window(s: &str, t: &str) -> String {
    use std::collections::HashMap;

    let s_bytes = s.as_bytes();
    let mut need: HashMap<u8, i32> = HashMap::new();
    for &b in t.as_bytes() {
        *need.entry(b).or_insert(0) += 1;
    }

    let mut have: HashMap<u8, i32> = HashMap::new();
    let mut formed = 0;
    let required = need.len();
    let mut min_len = usize::MAX;
    let mut min_start = 0;
    let mut left = 0;

    for right in 0..s_bytes.len() {
        let c = s_bytes[right];
        *have.entry(c).or_insert(0) += 1;

        if need.contains_key(&c) && have[&c] == need[&c] {
            formed += 1;
        }

        while formed == required {
            let window_len = right - left + 1;
            if window_len < min_len {
                min_len = window_len;
                min_start = left;
            }
            let left_char = s_bytes[left];
            *have.get_mut(&left_char).unwrap() -= 1;
            if need.contains_key(&left_char) && have[&left_char] < need[&left_char] {
                formed -= 1;
            }
            left += 1;
        }
    }

    if min_len == usize::MAX {
        String::new()
    } else {
        s[min_start..min_start + min_len].to_string()
    }
}

// --- 21.4 Prefix Sum ---

struct PrefixSum {
    prefix: Vec<i64>,
}

impl PrefixSum {
    fn new(nums: &[i32]) -> Self {
        let mut prefix = vec![0i64; nums.len() + 1];
        for (i, &num) in nums.iter().enumerate() {
            prefix[i + 1] = prefix[i] + num as i64;
        }
        PrefixSum { prefix }
    }

    // Sum of nums[left..=right]
    fn range_sum(&self, left: usize, right: usize) -> i64 {
        self.prefix[right + 1] - self.prefix[left]
    }
}

// --- 21.5 String Problems ---

fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i].to_lowercase().next() != chars[len - 1 - i].to_lowercase().next() {
            return false;
        }
    }
    true
}

fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut counts = [0i32; 26];
    for (a, b) in s.bytes().zip(t.bytes()) {
        counts[(a - b'a') as usize] += 1;
        counts[(b - b'a') as usize] -= 1;
    }
    counts.iter().all(|&c| c == 0)
}

fn group_anagrams(words: &[&str]) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for &word in words {
        let mut key: Vec<u8> = word.bytes().collect();
        key.sort();
        let key = String::from_utf8(key).unwrap();
        groups.entry(key).or_default().push(word.to_string());
    }

    groups.into_values().collect()
}

fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let first = strs[0].as_bytes();
    let mut len = first.len();

    for s in &strs[1..] {
        len = len.min(s.len());
        for i in 0..len {
            if s.as_bytes()[i] != first[i] {
                len = i;
                break;
            }
        }
    }
    strs[0][..len].to_string()
}

// --- 21.6 Kadane's Algorithm ---

fn max_subarray(nums: &[i32]) -> i32 {
    let mut max_sum = nums[0];
    let mut current = nums[0];

    for &num in &nums[1..] {
        current = num.max(current + num);
        max_sum = max_sum.max(current);
    }
    max_sum
}

// --- 21.7 Dutch National Flag ---

fn sort_colors(nums: &mut Vec<i32>) {
    let mut low = 0;
    let mut mid = 0;
    let mut high = nums.len() as i32 - 1;

    while mid <= high {
        match nums[mid as usize] {
            0 => {
                nums.swap(low, mid as usize);
                low += 1;
                mid += 1;
            }
            1 => {
                mid += 1;
            }
            2 => {
                nums.swap(mid as usize, high as usize);
                high -= 1;
            }
            _ => unreachable!(),
        }
    }
}

// --- 21.8 Next Permutation ---

fn next_permutation(nums: &mut Vec<i32>) {
    let n = nums.len();
    if n <= 1 {
        return;
    }

    // Find first decreasing element from right
    let mut i = n - 2;
    while i < n && nums[i] >= nums[i + 1] {
        if i == 0 {
            nums.reverse();
            return;
        }
        i -= 1;
    }

    // Find smallest element greater than nums[i] from right
    let mut j = n - 1;
    while nums[j] <= nums[i] {
        j -= 1;
    }

    nums.swap(i, j);
    nums[i + 1..].reverse();
}

// --- 21.9 Rotate Array ---

fn rotate_right(nums: &mut Vec<i32>, k: usize) {
    let n = nums.len();
    let k = k % n;
    nums.reverse();
    nums[..k].reverse();
    nums[k..].reverse();
}

// --- 21.10 Trapping Rain Water ---

fn trap_water(height: &[i32]) -> i32 {
    let n = height.len();
    if n < 3 {
        return 0;
    }

    let (mut left, mut right) = (0, n - 1);
    let (mut left_max, mut right_max) = (0, 0);
    let mut water = 0;

    while left < right {
        if height[left] < height[right] {
            left_max = left_max.max(height[left]);
            water += left_max - height[left];
            left += 1;
        } else {
            right_max = right_max.max(height[right]);
            water += right_max - height[right];
            right -= 1;
        }
    }
    water
}
