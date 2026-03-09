// ============================================================
// STEP 25: DSA — Graphs
// ============================================================
// Run: rustc --edition 2021 main.rs && ./main
//
// Graphs are one of the most common interview topics.
// Covers:
// - Adjacency lists
// - BFS / DFS
// - Connected components
// - Topological sort
// - Dijkstra's shortest path
// - Union-Find / Disjoint Set Union
// ============================================================

use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

fn main() {
    // =========================
    // 25.1 Graph Representation
    // =========================

    println!("--- Graph Representation ---");

    // Undirected graph:
    // 0 -- 1 -- 3
    // |    |
    // 2    4 -- 5
    let graph = vec![
        vec![1, 2],
        vec![0, 3, 4],
        vec![0],
        vec![1],
        vec![1, 5],
        vec![4],
    ];

    for (node, edges) in graph.iter().enumerate() {
        println!("  {} -> {:?}", node, edges);
    }

    // =========================
    // 25.2 BFS Traversal + Shortest Path
    // =========================

    println!("\n--- BFS Traversal + Shortest Path ---");

    let order = bfs_order(&graph, 0);
    assert_eq!(order, vec![0, 1, 2, 3, 4, 5]);
    println!("  BFS order from 0: {:?}", order);

    let (distance, path) = shortest_path_unweighted(&graph, 0, 5).unwrap();
    assert_eq!(distance, 3);
    assert_eq!(path, vec![0, 1, 4, 5]);
    println!("  Shortest path 0 -> 5: {:?} (distance {})", path, distance);

    // =========================
    // 25.3 DFS Traversal
    // =========================

    println!("\n--- DFS Traversal ---");

    let mut visited = vec![false; graph.len()];
    let mut recursive = Vec::new();
    dfs_recursive(&graph, 0, &mut visited, &mut recursive);

    let iterative = dfs_iterative(&graph, 0);
    assert_eq!(recursive, vec![0, 1, 3, 4, 5, 2]);
    assert_eq!(iterative, recursive);
    println!("  DFS order: {:?}", recursive);

    // =========================
    // 25.4 Connected Components
    // =========================

    println!("\n--- Connected Components ---");

    let disconnected = vec![vec![1], vec![0], vec![3], vec![2], vec![]];
    let components = connected_components(&disconnected);
    assert_eq!(components, vec![vec![0, 1], vec![2, 3], vec![4]]);
    println!("  Components: {:?}", components);

    // =========================
    // 25.5 Topological Sort
    // =========================

    println!("\n--- Topological Sort / Course Schedule ---");

    // Edge u -> v means u must come before v.
    let dag_edges = vec![(0, 1), (0, 2), (1, 3), (2, 3), (3, 4)];
    let topo = topological_sort(5, &dag_edges).unwrap();
    assert!(respects_edges(&topo, &dag_edges));
    println!("  Valid topological order: {:?}", topo);

    let cycle = vec![(0, 1), (1, 2), (2, 0)];
    assert!(topological_sort(3, &cycle).is_none());
    println!("  Cycle detection works.");

    // =========================
    // 25.6 Dijkstra's Algorithm
    // =========================

    println!("\n--- Dijkstra's Shortest Path ---");

    let weighted = vec![
        vec![(1, 4), (2, 1)],
        vec![(3, 1)],
        vec![(1, 2), (3, 5)],
        vec![(4, 3)],
        vec![],
    ];

    let (dist, prev) = dijkstra(&weighted, 0);
    assert_eq!(dist, vec![0, 3, 1, 4, 7]);

    let path = reconstruct_path(&prev, 0, 4).unwrap();
    assert_eq!(path, vec![0, 2, 1, 3, 4]);
    println!("  Distances from 0: {:?}", dist);
    println!("  Best path 0 -> 4: {:?}", path);

    // =========================
    // 25.7 Union-Find / Provinces
    // =========================

    println!("\n--- Union-Find / Provinces ---");

    let is_connected = vec![
        vec![1, 1, 0, 0],
        vec![1, 1, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 1],
    ];
    assert_eq!(count_provinces(&is_connected), 3);
    println!("  Province count test passed.");

    let mut uf = UnionFind::new(5);
    uf.union(0, 1);
    uf.union(1, 2);
    assert!(uf.connected(0, 2));
    assert!(!uf.connected(0, 4));
    println!("  Components after unions: {}", uf.components());

    println!("\nPopular graph interview patterns:");
    println!("  - BFS on grids: number of islands, rotten oranges");
    println!("  - DFS: connected components, flood fill, cycle detection");
    println!("  - Topological sort: course schedule, alien dictionary");
    println!("  - Dijkstra: weighted shortest path, network delay time");
    println!("  - Union-Find: redundant connection, provinces, MST");

    println!("\n--- Step 25 Complete! ---");
    println!("Next: step_26 — Sorting & Selection");
}

// ============================================================
// BFS / DFS
// ============================================================

fn bfs_order(graph: &[Vec<usize>], start: usize) -> Vec<usize> {
    let mut order = Vec::new();
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &next in &graph[node] {
            if !visited[next] {
                visited[next] = true;
                queue.push_back(next);
            }
        }
    }

    order
}

fn shortest_path_unweighted(
    graph: &[Vec<usize>],
    start: usize,
    goal: usize,
) -> Option<(usize, Vec<usize>)> {
    let mut distance = vec![usize::MAX; graph.len()];
    let mut previous = vec![None; graph.len()];
    let mut queue = VecDeque::new();

    distance[start] = 0;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if node == goal {
            break;
        }
        for &next in &graph[node] {
            if distance[next] == usize::MAX {
                distance[next] = distance[node] + 1;
                previous[next] = Some(node);
                queue.push_back(next);
            }
        }
    }

    if distance[goal] == usize::MAX {
        None
    } else {
        Some((distance[goal], reconstruct_path(&previous, start, goal)?))
    }
}

fn dfs_recursive(
    graph: &[Vec<usize>],
    node: usize,
    visited: &mut [bool],
    order: &mut Vec<usize>,
) {
    visited[node] = true;
    order.push(node);

    for &next in &graph[node] {
        if !visited[next] {
            dfs_recursive(graph, next, visited, order);
        }
    }
}

fn dfs_iterative(graph: &[Vec<usize>], start: usize) -> Vec<usize> {
    let mut order = Vec::new();
    let mut visited = vec![false; graph.len()];
    let mut stack = vec![start];

    while let Some(node) = stack.pop() {
        if visited[node] {
            continue;
        }
        visited[node] = true;
        order.push(node);

        for &next in graph[node].iter().rev() {
            if !visited[next] {
                stack.push(next);
            }
        }
    }

    order
}

fn connected_components(graph: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut visited = vec![false; graph.len()];
    let mut components = Vec::new();

    for start in 0..graph.len() {
        if visited[start] {
            continue;
        }

        let mut component = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);
        visited[start] = true;

        while let Some(node) = queue.pop_front() {
            component.push(node);
            for &next in &graph[node] {
                if !visited[next] {
                    visited[next] = true;
                    queue.push_back(next);
                }
            }
        }

        components.push(component);
    }

    components
}

// ============================================================
// Topological Sort
// ============================================================

fn topological_sort(num_nodes: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut graph = vec![Vec::new(); num_nodes];
    let mut indegree = vec![0usize; num_nodes];
    let mut queue = VecDeque::new();

    for &(from, to) in edges {
        graph[from].push(to);
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
        for &next in &graph[node] {
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

// ============================================================
// Dijkstra
// ============================================================

fn dijkstra(graph: &[Vec<(usize, i32)>], start: usize) -> (Vec<i32>, Vec<Option<usize>>) {
    let mut dist = vec![i32::MAX; graph.len()];
    let mut prev = vec![None; graph.len()];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push((Reverse(0), start));

    while let Some((Reverse(cost), node)) = heap.pop() {
        if cost != dist[node] {
            continue;
        }

        for &(next, weight) in &graph[node] {
            let next_cost = cost + weight;
            if next_cost < dist[next] {
                dist[next] = next_cost;
                prev[next] = Some(node);
                heap.push((Reverse(next_cost), next));
            }
        }
    }

    (dist, prev)
}

fn reconstruct_path(prev: &[Option<usize>], start: usize, goal: usize) -> Option<Vec<usize>> {
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
// Union-Find
// ============================================================

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    components: usize,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
            components: size,
        }
    }

    fn find(&mut self, node: usize) -> usize {
        if self.parent[node] != node {
            let root = self.find(self.parent[node]);
            self.parent[node] = root;
        }
        self.parent[node]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return false;
        }

        if self.rank[root_a] < self.rank[root_b] {
            self.parent[root_a] = root_b;
        } else if self.rank[root_a] > self.rank[root_b] {
            self.parent[root_b] = root_a;
        } else {
            self.parent[root_b] = root_a;
            self.rank[root_a] += 1;
        }

        self.components -= 1;
        true
    }

    fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn components(&self) -> usize {
        self.components
    }
}

fn count_provinces(matrix: &[Vec<i32>]) -> usize {
    let n = matrix.len();
    let mut uf = UnionFind::new(n);

    for i in 0..n {
        for j in i + 1..n {
            if matrix[i][j] == 1 {
                uf.union(i, j);
            }
        }
    }

    uf.components()
}
