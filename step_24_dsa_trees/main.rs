// ============================================================
// STEP 24: DSA — Trees (Binary Tree, BST)
// ============================================================
// Run: rustc main.rs && ./main
//
// Trees in Rust use Option<Box<TreeNode>> for child pointers.
// Covers: traversals, BST operations, and classic tree problems.
// ============================================================

use std::collections::VecDeque;

fn main() {
    // =========================
    // 24.1 Building a Binary Tree
    // =========================

    println!("--- Binary Tree ---");

    //        1
    //       / \
    //      2   3
    //     / \   \
    //    4   5   6
    let tree = node(
        1,
        node(2, leaf(4), leaf(5)),
        node(3, None, leaf(6)),
    );

    // =========================
    // 24.2 Tree Traversals
    // =========================

    println!("\n--- Traversals ---");

    let mut result = Vec::new();
    inorder(&tree, &mut result);
    println!("Inorder:    {:?}", result); // 4 2 5 1 3 6

    let mut result = Vec::new();
    preorder(&tree, &mut result);
    println!("Preorder:   {:?}", result); // 1 2 4 5 3 6

    let mut result = Vec::new();
    postorder(&tree, &mut result);
    println!("Postorder:  {:?}", result); // 4 5 2 6 3 1

    let result = level_order(&tree);
    println!("Level order: {:?}", result); // [[1], [2,3], [4,5,6]]

    // Iterative inorder:
    let result = inorder_iterative(&tree);
    println!("Inorder (iterative): {:?}", result);

    // =========================
    // 24.3 Tree Properties
    // =========================

    println!("\n--- Tree Properties ---");
    println!("Height: {}", height(&tree));
    println!("Node count: {}", count_nodes(&tree));
    println!("Is balanced? {}", is_balanced(&tree));

    // =========================
    // 24.4 Maximum/Minimum Depth
    // =========================

    println!("Max depth: {}", max_depth(&tree));
    println!("Min depth: {}", min_depth(&tree));

    // =========================
    // 24.5 Path Sum
    // =========================

    println!("\n--- Path Sum ---");
    //   5
    //  / \
    // 4   8
    // |  / \
    // 11 13 4
    // /\     \
    // 7 2     1
    let path_tree = node(
        5,
        node(4, node(11, leaf(7), leaf(2)), None),
        node(8, leaf(13), node(4, None, leaf(1))),
    );

    println!("Has path sum 22? {}", has_path_sum(&path_tree, 22)); // 5->4->11->2
    println!("Has path sum 26? {}", has_path_sum(&path_tree, 26)); // 5->8->13

    let paths = all_root_to_leaf_paths(&path_tree);
    println!("All paths:");
    for path in &paths {
        println!("  {:?} (sum={})", path, path.iter().sum::<i32>());
    }

    // =========================
    // 24.6 Binary Search Tree (BST)
    // =========================

    println!("\n--- BST ---");

    let mut bst = None;
    for &val in &[5, 3, 7, 1, 4, 6, 8, 2] {
        bst = bst_insert(bst, val);
    }

    let mut sorted = Vec::new();
    inorder(&bst, &mut sorted);
    println!("BST inorder (sorted): {:?}", sorted);

    println!("Search 4: {}", bst_search(&bst, 4));
    println!("Search 9: {}", bst_search(&bst, 9));

    println!("BST min: {:?}", bst_min(&bst));
    println!("BST max: {:?}", bst_max(&bst));

    // Delete a node
    let bst = bst_delete(bst, 5);
    let mut sorted = Vec::new();
    inorder(&bst, &mut sorted);
    println!("After deleting 5: {:?}", sorted);

    // =========================
    // 24.7 Validate BST
    // =========================

    println!("\n--- Validate BST ---");
    let valid_bst = node(2, leaf(1), leaf(3));
    let invalid_bst = node(5, leaf(1), node(4, leaf(3), leaf(6)));
    println!("Valid BST? {}", is_valid_bst(&valid_bst, i64::MIN, i64::MAX));
    println!("Invalid BST? {}", is_valid_bst(&invalid_bst, i64::MIN, i64::MAX));

    // =========================
    // 24.8 Lowest Common Ancestor (BST)
    // =========================

    println!("\n--- LCA ---");
    let mut bst = None;
    for &v in &[6, 2, 8, 0, 4, 7, 9, 3, 5] {
        bst = bst_insert(bst, v);
    }
    println!("LCA(2,8) = {:?}", bst_lca(&bst, 2, 8));   // 6
    println!("LCA(2,4) = {:?}", bst_lca(&bst, 2, 4));   // 2
    println!("LCA(3,5) = {:?}", bst_lca(&bst, 3, 5));   // 4

    // =========================
    // 24.9 Invert Binary Tree
    // =========================

    println!("\n--- Invert Tree ---");
    let tree = node(4, node(2, leaf(1), leaf(3)), node(7, leaf(6), leaf(9)));
    let mut before = Vec::new();
    level_order_flat(&tree, &mut before);
    println!("Before: {:?}", before);

    let inverted = invert_tree(tree);
    let mut after = Vec::new();
    level_order_flat(&inverted, &mut after);
    println!("After:  {:?}", after);

    // =========================
    // 24.10 Serialize / Deserialize
    // =========================

    println!("\n--- Serialize / Deserialize ---");
    let tree = node(1, node(2, leaf(4), leaf(5)), node(3, None, leaf(6)));
    let serialized = serialize(&tree);
    println!("Serialized: {}", serialized);

    let deserialized = deserialize(&serialized);
    let reserialized = serialize(&deserialized);
    println!("Round-trip: {}", reserialized);
    assert_eq!(serialized, reserialized);
    println!("Round-trip matches!");

    println!("\n--- Step 24 Complete! ---");
}

// ============================================================
// Tree Node Definition
// ============================================================

type Tree = Option<Box<TreeNode>>;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Tree,
    right: Tree,
}

// Helper constructors
fn leaf(val: i32) -> Tree {
    Some(Box::new(TreeNode {
        val,
        left: None,
        right: None,
    }))
}

fn node(val: i32, left: Tree, right: Tree) -> Tree {
    Some(Box::new(TreeNode { val, left, right }))
}

// ============================================================
// Traversals
// ============================================================

fn inorder(root: &Tree, result: &mut Vec<i32>) {
    if let Some(n) = root {
        inorder(&n.left, result);
        result.push(n.val);
        inorder(&n.right, result);
    }
}

fn preorder(root: &Tree, result: &mut Vec<i32>) {
    if let Some(n) = root {
        result.push(n.val);
        preorder(&n.left, result);
        preorder(&n.right, result);
    }
}

fn postorder(root: &Tree, result: &mut Vec<i32>) {
    if let Some(n) = root {
        postorder(&n.left, result);
        postorder(&n.right, result);
        result.push(n.val);
    }
}

fn level_order(root: &Tree) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    if root.is_none() {
        return result;
    }

    let mut queue: VecDeque<&Box<TreeNode>> = VecDeque::new();
    if let Some(n) = root {
        queue.push_back(n);
    }

    while !queue.is_empty() {
        let mut level = Vec::new();
        let size = queue.len();
        for _ in 0..size {
            let node = queue.pop_front().unwrap();
            level.push(node.val);
            if let Some(ref left) = node.left {
                queue.push_back(left);
            }
            if let Some(ref right) = node.right {
                queue.push_back(right);
            }
        }
        result.push(level);
    }
    result
}

fn level_order_flat(root: &Tree, result: &mut Vec<i32>) {
    for level in level_order(root) {
        result.extend(level);
    }
}

fn inorder_iterative(root: &Tree) -> Vec<i32> {
    let mut result = Vec::new();
    let mut stack: Vec<&Box<TreeNode>> = Vec::new();
    let mut current = root;

    loop {
        while let Some(node) = current {
            stack.push(node);
            current = &node.left;
        }
        if let Some(node) = stack.pop() {
            result.push(node.val);
            current = &node.right;
        } else {
            break;
        }
    }
    result
}

// ============================================================
// Tree Properties
// ============================================================

fn height(root: &Tree) -> i32 {
    match root {
        None => 0,
        Some(n) => 1 + height(&n.left).max(height(&n.right)),
    }
}

fn max_depth(root: &Tree) -> i32 {
    height(root)
}

fn min_depth(root: &Tree) -> i32 {
    match root {
        None => 0,
        Some(n) => {
            let left = min_depth(&n.left);
            let right = min_depth(&n.right);
            if left == 0 {
                right + 1
            } else if right == 0 {
                left + 1
            } else {
                left.min(right) + 1
            }
        }
    }
}

fn count_nodes(root: &Tree) -> i32 {
    match root {
        None => 0,
        Some(n) => 1 + count_nodes(&n.left) + count_nodes(&n.right),
    }
}

fn is_balanced(root: &Tree) -> bool {
    check_balance(root) != -1
}

fn check_balance(root: &Tree) -> i32 {
    match root {
        None => 0,
        Some(n) => {
            let left = check_balance(&n.left);
            let right = check_balance(&n.right);
            if left == -1 || right == -1 || (left - right).abs() > 1 {
                -1
            } else {
                1 + left.max(right)
            }
        }
    }
}

// ============================================================
// Path Problems
// ============================================================

fn has_path_sum(root: &Tree, target: i32) -> bool {
    match root {
        None => false,
        Some(n) => {
            let remaining = target - n.val;
            if n.left.is_none() && n.right.is_none() {
                return remaining == 0;
            }
            has_path_sum(&n.left, remaining) || has_path_sum(&n.right, remaining)
        }
    }
}

fn all_root_to_leaf_paths(root: &Tree) -> Vec<Vec<i32>> {
    let mut paths = Vec::new();
    let mut current_path = Vec::new();
    collect_paths(root, &mut current_path, &mut paths);
    paths
}

fn collect_paths(root: &Tree, path: &mut Vec<i32>, paths: &mut Vec<Vec<i32>>) {
    if let Some(n) = root {
        path.push(n.val);
        if n.left.is_none() && n.right.is_none() {
            paths.push(path.clone());
        } else {
            collect_paths(&n.left, path, paths);
            collect_paths(&n.right, path, paths);
        }
        path.pop();
    }
}

// ============================================================
// BST Operations
// ============================================================

fn bst_insert(root: Tree, val: i32) -> Tree {
    match root {
        None => leaf(val),
        Some(mut n) => {
            if val < n.val {
                n.left = bst_insert(n.left, val);
            } else if val > n.val {
                n.right = bst_insert(n.right, val);
            }
            Some(n)
        }
    }
}

fn bst_search(root: &Tree, val: i32) -> bool {
    match root {
        None => false,
        Some(n) => {
            if val == n.val {
                true
            } else if val < n.val {
                bst_search(&n.left, val)
            } else {
                bst_search(&n.right, val)
            }
        }
    }
}

fn bst_min(root: &Tree) -> Option<i32> {
    match root {
        None => None,
        Some(n) => {
            if n.left.is_none() {
                Some(n.val)
            } else {
                bst_min(&n.left)
            }
        }
    }
}

fn bst_max(root: &Tree) -> Option<i32> {
    match root {
        None => None,
        Some(n) => {
            if n.right.is_none() {
                Some(n.val)
            } else {
                bst_max(&n.right)
            }
        }
    }
}

fn bst_delete(root: Tree, val: i32) -> Tree {
    match root {
        None => None,
        Some(mut n) => {
            if val < n.val {
                n.left = bst_delete(n.left, val);
                Some(n)
            } else if val > n.val {
                n.right = bst_delete(n.right, val);
                Some(n)
            } else {
                // Found the node to delete
                if n.left.is_none() {
                    return n.right;
                }
                if n.right.is_none() {
                    return n.left;
                }
                // Two children: replace with inorder successor
                let successor_val = bst_min(&n.right).unwrap();
                n.val = successor_val;
                n.right = bst_delete(n.right, successor_val);
                Some(n)
            }
        }
    }
}

fn is_valid_bst(root: &Tree, min: i64, max: i64) -> bool {
    match root {
        None => true,
        Some(n) => {
            let val = n.val as i64;
            if val <= min || val >= max {
                return false;
            }
            is_valid_bst(&n.left, min, val) && is_valid_bst(&n.right, val, max)
        }
    }
}

fn bst_lca(root: &Tree, p: i32, q: i32) -> Option<i32> {
    match root {
        None => None,
        Some(n) => {
            if p < n.val && q < n.val {
                bst_lca(&n.left, p, q)
            } else if p > n.val && q > n.val {
                bst_lca(&n.right, p, q)
            } else {
                Some(n.val)
            }
        }
    }
}

fn invert_tree(root: Tree) -> Tree {
    match root {
        None => None,
        Some(mut n) => {
            let left = invert_tree(n.left);
            let right = invert_tree(n.right);
            n.left = right;
            n.right = left;
            Some(n)
        }
    }
}

// ============================================================
// Serialize / Deserialize
// ============================================================

fn serialize(root: &Tree) -> String {
    let mut result = Vec::new();
    serialize_helper(root, &mut result);
    result.join(",")
}

fn serialize_helper(root: &Tree, result: &mut Vec<String>) {
    match root {
        None => result.push("null".to_string()),
        Some(n) => {
            result.push(n.val.to_string());
            serialize_helper(&n.left, result);
            serialize_helper(&n.right, result);
        }
    }
}

fn deserialize(data: &str) -> Tree {
    let tokens: Vec<&str> = data.split(',').collect();
    let mut idx = 0;
    deserialize_helper(&tokens, &mut idx)
}

fn deserialize_helper(tokens: &[&str], idx: &mut usize) -> Tree {
    if *idx >= tokens.len() || tokens[*idx] == "null" {
        *idx += 1;
        return None;
    }
    let val: i32 = tokens[*idx].parse().unwrap();
    *idx += 1;
    let left = deserialize_helper(tokens, idx);
    let right = deserialize_helper(tokens, idx);
    node(val, left, right)
}
