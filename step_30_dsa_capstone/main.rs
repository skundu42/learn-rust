// ============================================================
// STEP 30: DSA Capstone — Interview Study Planner
// ============================================================
// Run: rustc --edition 2021 main.rs && ./main
//
// This capstone combines several patterns from the DSA track:
// - Graphs / topological sort for prerequisites
// - BFS for dependency chains
// - Heap-based ranking for next recommendations
// - Dynamic programming for revision planning
// - Trie-based autocomplete for topic lookup
// ============================================================

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    let planner = StudyPlanner::new(sample_topics(), sample_prerequisites());

    // =========================
    // 30.1 Catalog
    // =========================

    println!("--- Topic Catalog ---");
    planner.print_catalog();

    // =========================
    // 30.2 Learning Order
    // =========================

    println!("\n--- Learning Order (Topological Sort) ---");

    let order = planner.learning_order().unwrap();
    let names = planner.topic_names(&order);
    assert!(respects_edges(&order, &planner.prerequisites));
    println!("  {:?}", names);

    // =========================
    // 30.3 Ready Topics
    // =========================

    println!("\n--- Recommended Next Topics (Heap Ranking) ---");

    let completed = ["ownership", "borrowing", "collections"];
    let ready = planner.recommended_next_topics(&completed, 3);
    assert_eq!(ready, vec!["recursion", "heaps", "concurrency"]);
    println!("  After {:?}: {:?}", completed, ready);

    // =========================
    // 30.4 Dependency Chain
    // =========================

    println!("\n--- Shortest Dependency Chain (BFS) ---");

    let chain = planner
        .dependency_chain("ownership", "graphs")
        .unwrap();
    assert_eq!(chain, vec!["ownership", "collections", "trees", "graphs"]);
    println!("  ownership -> graphs: {:?}", chain);

    // =========================
    // 30.5 Revision Bundle
    // =========================

    println!("\n--- Best 8-Hour Revision Bundle (DP) ---");

    // Revision planning ignores prerequisite ordering and simply
    // maximizes value under a strict time budget.
    let (score, mut bundle) = planner.best_revision_plan(8);
    bundle.sort_unstable();
    assert_eq!(score, 27);
    assert_eq!(bundle, vec!["borrowing", "ownership", "trees"]);
    println!("  Score {} with topics {:?}", score, bundle);

    // =========================
    // 30.6 Autocomplete
    // =========================

    println!("\n--- Topic Autocomplete (Trie) ---");

    assert_eq!(
        planner.autocomplete("co", 5),
        vec!["collections".to_string(), "concurrency".to_string()]
    );
    assert_eq!(planner.autocomplete("gr", 5), vec!["graphs".to_string()]);
    println!("  \"co\" -> {:?}", planner.autocomplete("co", 5));
    println!("  \"gr\" -> {:?}", planner.autocomplete("gr", 5));

    // =========================
    // 30.7 Validation
    // =========================

    println!("\n--- Cycle Detection ---");

    let cycle = vec![(0, 1), (1, 2), (2, 0)];
    assert!(topological_sort(3, &cycle).is_none());
    println!("  Cyclic prerequisite graph rejected correctly.");

    println!("\nPatterns used in this capstone:");
    println!("  - Graph traversal");
    println!("  - Topological sort");
    println!("  - Priority queue ranking");
    println!("  - 0/1 knapsack style DP");
    println!("  - Trie prefix search");

    println!("\n--- Step 30 Complete! ---");
}

// ============================================================
// Domain Types
// ============================================================

#[derive(Debug, Clone)]
struct Topic {
    name: &'static str,
    hours: usize,
    value: usize,
}

struct StudyPlanner {
    topics: Vec<Topic>,
    prerequisites: Vec<(usize, usize)>,
    name_to_id: HashMap<&'static str, usize>,
    search_index: Trie,
}

impl StudyPlanner {
    fn new(topics: Vec<Topic>, prerequisites: Vec<(usize, usize)>) -> Self {
        let mut name_to_id = HashMap::new();
        let mut search_index = Trie::new();

        for (id, topic) in topics.iter().enumerate() {
            name_to_id.insert(topic.name, id);
            search_index.insert(topic.name);
        }

        StudyPlanner {
            topics,
            prerequisites,
            name_to_id,
            search_index,
        }
    }

    fn print_catalog(&self) {
        for topic in &self.topics {
            println!(
                "  {:<20} {:>2}h  importance {}",
                topic.name, topic.hours, topic.value
            );
        }
    }

    fn topic_names(&self, ids: &[usize]) -> Vec<&'static str> {
        ids.iter().map(|&id| self.topics[id].name).collect()
    }

    fn learning_order(&self) -> Option<Vec<usize>> {
        topological_sort(self.topics.len(), &self.prerequisites)
    }

    fn recommended_next_topics(&self, completed: &[&str], limit: usize) -> Vec<&'static str> {
        let completed: HashSet<usize> = completed
            .iter()
            .filter_map(|name| self.name_to_id.get(name).copied())
            .collect();

        let incoming = build_incoming(self.topics.len(), &self.prerequisites);
        let mut heap = BinaryHeap::new();

        for (id, topic) in self.topics.iter().enumerate() {
            if completed.contains(&id) {
                continue;
            }

            if incoming[id]
                .iter()
                .all(|prereq| completed.contains(prereq))
            {
                let ratio = topic.value * 100 / topic.hours;
                heap.push((ratio, topic.value, Reverse(topic.hours), topic.name));
            }
        }

        let mut result = Vec::new();
        while let Some((_, _, _, name)) = heap.pop() {
            result.push(name);
            if result.len() == limit {
                break;
            }
        }

        result
    }

    fn dependency_chain(&self, start: &str, goal: &str) -> Option<Vec<&'static str>> {
        let start = *self.name_to_id.get(start)?;
        let goal = *self.name_to_id.get(goal)?;
        let adjacency = build_adjacency(self.topics.len(), &self.prerequisites);
        let ids = shortest_path(&adjacency, start, goal)?;
        Some(self.topic_names(&ids))
    }

    fn best_revision_plan(&self, hours_budget: usize) -> (usize, Vec<&'static str>) {
        let n = self.topics.len();
        let mut dp = vec![vec![0usize; hours_budget + 1]; n + 1];
        let mut take = vec![vec![false; hours_budget + 1]; n + 1];

        for i in 1..=n {
            let topic = &self.topics[i - 1];
            for cap in 0..=hours_budget {
                dp[i][cap] = dp[i - 1][cap];
                if topic.hours <= cap {
                    let candidate = dp[i - 1][cap - topic.hours] + topic.value;
                    if candidate > dp[i][cap] {
                        dp[i][cap] = candidate;
                        take[i][cap] = true;
                    }
                }
            }
        }

        let mut chosen = Vec::new();
        let mut cap = hours_budget;
        for i in (1..=n).rev() {
            if take[i][cap] {
                let topic = &self.topics[i - 1];
                chosen.push(topic.name);
                cap -= topic.hours;
            }
        }

        (dp[n][hours_budget], chosen)
    }

    fn autocomplete(&self, prefix: &str, limit: usize) -> Vec<String> {
        self.search_index.suggestions(prefix, limit)
    }
}

// ============================================================
// Graph Helpers
// ============================================================

fn topological_sort(num_nodes: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let adjacency = build_adjacency(num_nodes, edges);
    let mut indegree = vec![0usize; num_nodes];
    let mut queue = VecDeque::new();

    for &(_, to) in edges {
        indegree[to] += 1;
    }

    for (node, &degree) in indegree.iter().enumerate() {
        if degree == 0 {
            queue.push_back(node);
        }
    }

    let mut order = Vec::new();
    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &next in &adjacency[node] {
            indegree[next] -= 1;
            if indegree[next] == 0 {
                queue.push_back(next);
            }
        }
    }

    if order.len() == num_nodes {
        Some(order)
    } else {
        None
    }
}

fn respects_edges(order: &[usize], edges: &[(usize, usize)]) -> bool {
    let mut position = vec![0usize; order.len()];
    for (index, &node) in order.iter().enumerate() {
        position[node] = index;
    }

    edges
        .iter()
        .all(|&(from, to)| position[from] < position[to])
}

fn build_adjacency(num_nodes: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adjacency = vec![Vec::new(); num_nodes];
    for &(from, to) in edges {
        adjacency[from].push(to);
    }
    adjacency
}

fn build_incoming(num_nodes: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut incoming = vec![Vec::new(); num_nodes];
    for &(from, to) in edges {
        incoming[to].push(from);
    }
    incoming
}

fn shortest_path(graph: &[Vec<usize>], start: usize, goal: usize) -> Option<Vec<usize>> {
    let mut prev = vec![None; graph.len()];
    let mut seen = vec![false; graph.len()];
    let mut queue = VecDeque::new();

    seen[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if node == goal {
            break;
        }

        for &next in &graph[node] {
            if !seen[next] {
                seen[next] = true;
                prev[next] = Some(node);
                queue.push_back(next);
            }
        }
    }

    if !seen[goal] {
        return None;
    }

    let mut path = Vec::new();
    let mut current = goal;
    loop {
        path.push(current);
        if current == start {
            path.reverse();
            return Some(path);
        }
        current = prev[current]?;
    }
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

// ============================================================
// Sample Data
// ============================================================

fn sample_topics() -> Vec<Topic> {
    vec![
        Topic {
            name: "ownership",
            hours: 2,
            value: 9,
        },
        Topic {
            name: "borrowing",
            hours: 2,
            value: 8,
        },
        Topic {
            name: "collections",
            hours: 3,
            value: 8,
        },
        Topic {
            name: "recursion",
            hours: 2,
            value: 7,
        },
        Topic {
            name: "trees",
            hours: 4,
            value: 10,
        },
        Topic {
            name: "graphs",
            hours: 5,
            value: 10,
        },
        Topic {
            name: "heaps",
            hours: 3,
            value: 8,
        },
        Topic {
            name: "dynamic-programming",
            hours: 5,
            value: 10,
        },
        Topic {
            name: "backtracking",
            hours: 4,
            value: 9,
        },
        Topic {
            name: "concurrency",
            hours: 4,
            value: 7,
        },
        Topic {
            name: "async",
            hours: 3,
            value: 7,
        },
        Topic {
            name: "tries",
            hours: 3,
            value: 8,
        },
    ]
}

fn sample_prerequisites() -> Vec<(usize, usize)> {
    vec![
        (0, 1),  // ownership -> borrowing
        (0, 2),  // ownership -> collections
        (1, 3),  // borrowing -> recursion
        (2, 4),  // collections -> trees
        (3, 4),  // recursion -> trees
        (4, 5),  // trees -> graphs
        (2, 6),  // collections -> heaps
        (3, 7),  // recursion -> dynamic-programming
        (2, 7),  // collections -> dynamic-programming
        (3, 8),  // recursion -> backtracking
        (0, 9),  // ownership -> concurrency
        (9, 10), // concurrency -> async
        (4, 11), // trees -> tries
    ]
}
