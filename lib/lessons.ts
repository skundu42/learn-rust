export type Track = "fundamentals" | "advanced" | "dsa" | "projects";

export interface Lesson {
  id: number;
  slug: string;
  title: string;
  track: Track;
  description: string;
  concepts: string[];
  difficulty: "beginner" | "intermediate" | "advanced";
  /** Relative path from project root to the .rs source file. */
  starterFile: string;
  /** Populated at request time by lessons-server.ts – not inlined in the bundle. */
  starterCode: string;
  solutionHint: string;
}

export interface TrackMeta {
  id: Track;
  label: string;
  description: string;
  color: string;
  range: [number, number];
}

export const TRACKS: TrackMeta[] = [
  {
    id: "fundamentals",
    label: "Fundamentals",
    description: "Core Rust concepts every developer needs",
    color: "#e75a2b",
    range: [1, 12],
  },
  {
    id: "advanced",
    label: "Advanced Rust",
    description: "Deep dives into the power of Rust",
    color: "#3b82f6",
    range: [13, 20],
  },
  {
    id: "dsa",
    label: "Data Structures & Algorithms",
    description: "DSA patterns for technical interviews",
    color: "#a855f7",
    range: [21, 30],
  },
  {
    id: "projects",
    label: "Real-World Projects",
    description: "Build production-grade Rust applications",
    color: "#22c55e",
    range: [31, 36],
  },
];

export const LESSONS: Lesson[] = [
  // ── FUNDAMENTALS ─────────────────────────────────────────────────────────
  {
    id: 1,
    slug: "hello-world",
    title: "Hello World & Basics",
    track: "fundamentals",
    description:
      "Start your Rust journey. Learn about println!, string formatting, comments, and escape characters — the foundation of every Rust program.",
    concepts: ["println!", "string formatting", "comments", "escape characters"],
    difficulty: "beginner",
    starterFile: "step_01_hello_world/main.rs",
    starterCode: "",
    solutionHint: 'Use println!("Hello, {}!", name) with placeholders.',
  },
  {
    id: 2,
    slug: "variables-and-types",
    title: "Variables, Types & Mutability",
    track: "fundamentals",
    description:
      "Rust variables are immutable by default. Learn about let, mut, constants, shadowing, scalar types, tuples, arrays, and type conversions.",
    concepts: ["let", "mut", "const", "shadowing", "scalar types", "tuples", "arrays"],
    difficulty: "beginner",
    starterFile: "step_02_variables_and_types/main.rs",
    starterCode: "",
    solutionHint: "Use `let mut` for mutable variables. Shadow with `let x = x * 2;`",
  },
  {
    id: 3,
    slug: "functions-and-control-flow",
    title: "Functions & Control Flow",
    track: "fundamentals",
    description:
      "Define functions, understand expressions vs statements, and master if/else, loop, while, and for constructs.",
    concepts: ["fn", "expressions", "statements", "if/else", "loop", "while", "for"],
    difficulty: "beginner",
    starterFile: "step_03_functions_and_control_flow/main.rs",
    starterCode: "",
    solutionHint: "Use `fn name(param: Type) -> ReturnType { ... }` syntax.",
  },
  {
    id: 4,
    slug: "ownership-and-borrowing",
    title: "Ownership & Borrowing",
    track: "fundamentals",
    description:
      "THE most important Rust concept. Understand move semantics, references, mutable borrows, and string slices — how Rust achieves memory safety without GC.",
    concepts: ["ownership", "move semantics", "references", "mutable borrows", "slices"],
    difficulty: "beginner",
    starterFile: "step_04_ownership_and_borrowing/main.rs",
    starterCode: "",
    solutionHint: "Pass `&value` to borrow, `&mut value` to mutably borrow.",
  },
  {
    id: 5,
    slug: "structs-and-enums",
    title: "Structs & Enums",
    track: "fundamentals",
    description:
      "Model your data with structs and enums. Implement methods with impl blocks, use Option<T> to handle nullable values.",
    concepts: ["struct", "enum", "impl", "methods", "Option<T>"],
    difficulty: "beginner",
    starterFile: "step_05_structs_and_enums/main.rs",
    starterCode: "",
    solutionHint: "Use `(self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()`",
  },
  {
    id: 6,
    slug: "pattern-matching",
    title: "Pattern Matching",
    track: "fundamentals",
    description:
      "Master match expressions, guards, @bindings, if let, while let, and let-else for elegant control flow.",
    concepts: ["match", "guards", "@binding", "if let", "while let", "let-else"],
    difficulty: "beginner",
    starterFile: "step_06_pattern_matching/main.rs",
    starterCode: "",
    solutionHint: "Match on all enum variants. Use `Coin::Quarter(state)` to bind the state.",
  },
  {
    id: 7,
    slug: "collections",
    title: "Collections",
    track: "fundamentals",
    description:
      "Work with Vec, String, HashMap, VecDeque, HashSet, and BTreeMap — Rust's essential data collections.",
    concepts: ["Vec", "String", "HashMap", "HashSet", "BTreeMap", "VecDeque"],
    difficulty: "beginner",
    starterFile: "step_07_collections/main.rs",
    starterCode: "",
    solutionHint: "Use `*counts.entry(word).or_insert(0) += 1;`",
  },
  {
    id: 8,
    slug: "error-handling",
    title: "Error Handling",
    track: "fundamentals",
    description:
      "Handle errors gracefully with Result<T,E>, the ? operator, panic!, and custom error types.",
    concepts: ["Result", "panic!", "? operator", "custom errors", "From trait"],
    difficulty: "intermediate",
    starterFile: "step_08_error_handling/main.rs",
    starterCode: "",
    solutionHint: "The ? operator automatically converts and returns Err. Implement From for type conversion.",
  },
  {
    id: 9,
    slug: "traits-and-generics",
    title: "Traits & Generics",
    track: "fundamentals",
    description:
      "Define shared behavior with traits, write generic functions with bounds, and use impl Trait and dyn Trait.",
    concepts: ["trait", "generics", "bounds", "impl Trait", "dyn Trait"],
    difficulty: "intermediate",
    starterFile: "step_09_traits_and_generics/main.rs",
    starterCode: "",
    solutionHint: "Use `impl TraitName for StructName { fn method(&self) -> ReturnType { ... } }`",
  },
  {
    id: 10,
    slug: "lifetimes",
    title: "Lifetimes",
    track: "fundamentals",
    description:
      "Understand lifetime annotations, elision rules, struct lifetimes, and the 'static lifetime.",
    concepts: ["lifetime annotations", "elision", "struct lifetimes", "'static"],
    difficulty: "intermediate",
    starterFile: "step_10_lifetimes/main.rs",
    starterCode: "",
    solutionHint: "Add `<'a>` after the function name: `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`",
  },
  {
    id: 11,
    slug: "closures-and-iterators",
    title: "Closures & Iterators",
    track: "fundamentals",
    description:
      "Write expressive code with closures (Fn, FnMut, FnOnce), iterator adaptors, and custom iterators.",
    concepts: ["closures", "Fn/FnMut/FnOnce", "map", "filter", "fold", "custom iterators"],
    difficulty: "intermediate",
    starterFile: "step_11_closures_and_iterators/main.rs",
    starterCode: "",
    solutionHint: "Chain `.filter(|&&x| x % 2 == 0).map(|&x| x * x).sum()`",
  },
  {
    id: 12,
    slug: "modules-and-crates",
    title: "Modules & Crates",
    track: "fundamentals",
    description:
      "Organize code with mod, pub, use, re-exports, and the prelude pattern.",
    concepts: ["mod", "pub", "use", "re-exports", "prelude pattern"],
    difficulty: "intermediate",
    starterFile: "step_12_modules_and_crates/main.rs",
    starterCode: "",
    solutionHint: "Use `impl Shape for Circle { fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius } }`",
  },

  // ── ADVANCED ─────────────────────────────────────────────────────────────
  {
    id: 13,
    slug: "smart-pointers",
    title: "Smart Pointers",
    track: "advanced",
    description:
      "Use Box for heap allocation, Rc for shared ownership, RefCell for interior mutability, and Cow for efficient cloning.",
    concepts: ["Box", "Rc", "RefCell", "Cow", "Weak", "Deref", "Drop"],
    difficulty: "advanced",
    starterFile: "step_13_smart_pointers/main.rs",
    starterCode: "",
    solutionHint: "Use `Rc::new(RefCell::new(vec![]))` and `rc.borrow_mut().push(...)`",
  },
  {
    id: 14,
    slug: "concurrency",
    title: "Concurrency",
    track: "advanced",
    description:
      "Spawn threads, communicate with channels (mpsc), share state with Mutex and Arc.",
    concepts: ["threads", "mpsc channels", "Mutex", "Arc", "parallel patterns"],
    difficulty: "advanced",
    starterFile: "step_14_concurrency/main.rs",
    starterCode: "",
    solutionHint: "Use `Arc::clone(&data)` before moving into the thread closure.",
  },
  {
    id: 15,
    slug: "async-await",
    title: "Async/Await",
    track: "advanced",
    description:
      "Write asynchronous Rust with async fn, .await, Futures, and a manual executor.",
    concepts: ["async fn", ".await", "Future", "executor", "async runtime"],
    difficulty: "advanced",
    starterFile: "step_15_async_await/main.rs",
    starterCode: "",
    solutionHint: "Mark functions with `async fn` and call them with `.await`.",
  },
  {
    id: 16,
    slug: "macros",
    title: "Macros",
    track: "advanced",
    description:
      "Write powerful macros with macro_rules!, understand fragment types, repetition, and build DSL macros.",
    concepts: ["macro_rules!", "fragment types", "repetition", "DSL macros"],
    difficulty: "advanced",
    starterFile: "step_16_macros/main.rs",
    starterCode: "",
    solutionHint: "Use `$(,)?` for optional trailing comma. Use `stringify!($val)` to get variable name as string.",
  },
  {
    id: 17,
    slug: "unsafe-rust",
    title: "Unsafe Rust",
    track: "advanced",
    description:
      "Understand when and how to use unsafe: raw pointers, FFI C interop, mutable statics, and unions.",
    concepts: ["raw pointers", "unsafe blocks", "FFI", "mutable statics", "unions"],
    difficulty: "advanced",
    starterFile: "step_17_unsafe_rust/main.rs",
    starterCode: "",
    solutionHint: "Wrap unsafe code in `unsafe { }` blocks. Use `std::ptr::read/write` for pointer operations.",
  },
  {
    id: 18,
    slug: "capstone-cli-task-manager",
    title: "Capstone: CLI Task Manager",
    track: "advanced",
    description:
      "Combine everything from steps 1-17 to build a full CLI task manager with structs, traits, iterators, error handling, and file persistence.",
    concepts: ["structs", "enums", "traits", "iterators", "error handling", "file I/O"],
    difficulty: "advanced",
    starterFile: "step_18_capstone_project/main.rs",
    starterCode: "",
    solutionHint: "This is a complete implementation. Extend it with delete, filter, and search methods.",
  },
  {
    id: 19,
    slug: "advanced-traits",
    title: "Advanced Traits",
    track: "advanced",
    description:
      "Operator overloading, blanket implementations, object safety, and disambiguation with fully qualified syntax.",
    concepts: ["operator overloading", "blanket impls", "object safety", "disambiguation"],
    difficulty: "advanced",
    starterFile: "step_19_advanced_traits/main.rs",
    starterCode: "",
    solutionHint: "Implement `impl Add for Vec2 { type Output = Vec2; fn add(self, rhs: Vec2) -> Vec2 { ... } }`",
  },
  {
    id: 20,
    slug: "type-system-patterns",
    title: "Type System Patterns",
    track: "advanced",
    description:
      "Newtype pattern, builder pattern, typestate pattern, phantom types, and const generics.",
    concepts: ["newtype", "builder pattern", "typestate", "phantom types", "const generics"],
    difficulty: "advanced",
    starterFile: "step_20_type_patterns/main.rs",
    starterCode: "",
    solutionHint: "Builder methods take `self` (not `&mut self`) and return `Self` to enable chaining.",
  },

  // ── DSA ──────────────────────────────────────────────────────────────────
  {
    id: 21,
    slug: "dsa-arrays-strings",
    title: "DSA: Arrays & Strings",
    track: "dsa",
    description:
      "Two pointers, sliding window, prefix sum, Kadane's algorithm, and anagram detection.",
    concepts: ["two pointers", "sliding window", "prefix sum", "Kadane's", "anagrams"],
    difficulty: "intermediate",
    starterFile: "step_21_dsa_arrays_strings/main.rs",
    starterCode: "",
    solutionHint: "Kadane's: `cur = n.max(cur + n); max_sum = max_sum.max(cur);`",
  },
  {
    id: 22,
    slug: "dsa-linked-lists",
    title: "DSA: Linked Lists",
    track: "dsa",
    description:
      "Build a singly linked list, reverse it, merge sorted lists, and detect cycles.",
    concepts: ["linked list", "reverse", "merge sorted", "cycle detection", "two pointers"],
    difficulty: "intermediate",
    starterFile: "step_22_dsa_linked_lists/main.rs",
    starterCode: "",
    solutionHint: "Reverse iteratively: keep `prev` and `current`, rewire next pointers.",
  },
  {
    id: 23,
    slug: "dsa-stacks-queues",
    title: "DSA: Stacks, Queues & Heaps",
    track: "dsa",
    description:
      "Valid parentheses, monotonic stack, BinaryHeap usage, and top-k problems.",
    concepts: ["stack", "queue", "BinaryHeap", "monotonic stack", "top-k"],
    difficulty: "intermediate",
    starterFile: "step_23_dsa_stacks_queues/main.rs",
    starterCode: "",
    solutionHint: "Use `BinaryHeap<Reverse<...>>` for a min-heap. Maintain size k.",
  },
  {
    id: 24,
    slug: "dsa-trees",
    title: "DSA: Trees & BSTs",
    track: "dsa",
    description:
      "In-order, pre-order, post-order traversals, balanced tree checks, BST operations, and serialize/deserialize.",
    concepts: ["BST", "traversals", "balanced check", "serialize", "recursion"],
    difficulty: "intermediate",
    starterFile: "step_24_dsa_trees/main.rs",
    starterCode: "",
    solutionHint: "Inorder: left → root → right. Use recursive height(-1) sentinel for unbalanced detection.",
  },
  {
    id: 25,
    slug: "dsa-graphs",
    title: "DSA: Graphs",
    track: "dsa",
    description:
      "BFS, DFS, Dijkstra's shortest path, topological sort, and connected components.",
    concepts: ["BFS", "DFS", "Dijkstra", "topological sort", "adjacency list"],
    difficulty: "advanced",
    starterFile: "step_25_dsa_graphs/main.rs",
    starterCode: "",
    solutionHint: "Dijkstra uses a min-heap (BinaryHeap with Reverse). Always skip stale entries.",
  },
  {
    id: 26,
    slug: "dsa-sorting",
    title: "DSA: Sorting & Selection",
    track: "dsa",
    description:
      "Implement merge sort, quick sort, heap sort, counting sort, and the kth smallest element.",
    concepts: ["merge sort", "quick sort", "heap sort", "counting sort", "kth element"],
    difficulty: "intermediate",
    starterFile: "step_26_dsa_sorting/main.rs",
    starterCode: "",
    solutionHint: "Counting sort: count occurrences, then reconstruct the sorted array from counts.",
  },
  {
    id: 27,
    slug: "dsa-dynamic-programming",
    title: "DSA: Dynamic Programming",
    track: "dsa",
    description:
      "Knapsack, LCS, coin change, edit distance, and common DP patterns.",
    concepts: ["knapsack", "LCS", "coin change", "edit distance", "memoization"],
    difficulty: "advanced",
    starterFile: "step_27_dsa_dynamic_programming/main.rs",
    starterCode: "",
    solutionHint: "Bottom-up DP: fill a 2D table where dp[i][j] depends on dp[i-1][j-1], dp[i-1][j], dp[i][j-1].",
  },
  {
    id: 28,
    slug: "dsa-interview-patterns",
    title: "DSA: Interview Patterns",
    track: "dsa",
    description:
      "Backtracking, binary search variants, two heaps pattern, and interval merging.",
    concepts: ["backtracking", "binary search", "two heaps", "intervals", "sliding window"],
    difficulty: "advanced",
    starterFile: "step_28_dsa_interview_patterns/main.rs",
    starterCode: "",
    solutionHint: "Backtracking: try each option, recurse, then undo (restore state).",
  },
  {
    id: 29,
    slug: "advanced-iterators",
    title: "Advanced Iterators & Pipelines",
    track: "dsa",
    description:
      "Custom iterators, lazy evaluation, combinator chains, and writing your own iterator types.",
    concepts: ["custom Iterator", "lazy evaluation", "combinators", "zip", "chain", "scan"],
    difficulty: "advanced",
    starterFile: "step_29_advanced_iterators/main.rs",
    starterCode: "",
    solutionHint: "Implement `Iterator` trait with `type Item` and `fn next(&mut self) -> Option<Self::Item>`.",
  },
  {
    id: 30,
    slug: "dsa-capstone",
    title: "DSA Capstone: Interview Study Planner",
    track: "dsa",
    description:
      "Combine topological sort, BFS, heap ranking, DP, and Trie prefix search into a study planner.",
    concepts: ["topological sort", "BFS", "priority queue", "0/1 knapsack", "Trie"],
    difficulty: "advanced",
    starterFile: "step_30_dsa_capstone/main.rs",
    starterCode: "",
    solutionHint: "Topological sort uses Kahn's algorithm (indegree + BFS). DP uses 0/1 knapsack table.",
  },

  // ── PROJECTS ─────────────────────────────────────────────────────────────
  {
    id: 31,
    slug: "serde-serialization",
    title: "Serde Serialization",
    track: "projects",
    description:
      "Serialize and deserialize JSON, TOML, with field attributes, enum tagging, and custom serializers using Serde.",
    concepts: ["serde", "serde_json", "derive macros", "field attributes", "enum tagging"],
    difficulty: "intermediate",
    starterFile: "step_31_serde_serialization/src/main.rs",
    starterCode: "",
    solutionHint: "Add `#[derive(Serialize, Deserialize)]` and use `serde_json::to_string(&value)?`",
  },
  {
    id: 32,
    slug: "web-api-actix",
    title: "Web API with Actix-Web",
    track: "projects",
    description:
      "Build REST endpoints, CRUD operations, middleware, shared state, and JSON responses with Actix-Web.",
    concepts: ["actix-web", "REST", "CRUD", "middleware", "shared state", "JSON"],
    difficulty: "advanced",
    starterFile: "step_32_web_api/src/main.rs",
    starterCode: "",
    solutionHint: "In production, use `HttpResponse::Ok().json(data)` and `web::Data<Mutex<State>>` for shared state.",
  },
  {
    id: 33,
    slug: "cli-apps-clap",
    title: "CLI Apps with Clap",
    track: "projects",
    description:
      "Build professional CLI applications with Clap's derive macros, subcommands, colored output, and progress bars.",
    concepts: ["clap", "CLI", "subcommands", "derive macros", "colored output"],
    difficulty: "intermediate",
    starterFile: "step_33_cli_apps/src/main.rs",
    starterCode: "",
    solutionHint: "With Clap derive: `#[derive(Parser)]` on your struct, `#[derive(Subcommand)]` on enum.",
  },
  {
    id: 34,
    slug: "database-sqlite",
    title: "Database with SQLite",
    track: "projects",
    description:
      "Use rusqlite for SQLite database access, repository pattern, transactions, and parameterized queries.",
    concepts: ["rusqlite", "SQLite", "repository pattern", "transactions", "prepared statements"],
    difficulty: "advanced",
    starterFile: "step_34_database/src/main.rs",
    starterCode: "",
    solutionHint: "Use `params![val1, val2]` for parameterized queries to prevent SQL injection.",
  },
  {
    id: 35,
    slug: "testing-rust",
    title: "Testing in Rust",
    track: "projects",
    description:
      "Unit tests, assertion macros, panic tests, parameterized tests, and TDD in Rust.",
    concepts: ["#[test]", "assert!", "assert_eq!", "should_panic", "test modules", "TDD"],
    difficulty: "intermediate",
    starterFile: "step_35_testing/src/main.rs",
    starterCode: "",
    solutionHint: "Use `#[cfg(test)] mod tests { use super::*; }` to organize tests. Run with `cargo test`.",
  },
  {
    id: 36,
    slug: "rest-api-capstone",
    title: "REST API Capstone",
    track: "projects",
    description:
      "Build a full REST API combining Actix-Web, SQLite, Serde, validation, error handling, and testing.",
    concepts: ["actix-web", "rusqlite", "serde", "validation", "error handling", "integration tests"],
    difficulty: "advanced",
    starterFile: "step_36_rest_api_capstone/src/main.rs",
    starterCode: "",
    solutionHint: "Structure: models → repository → handlers → server. Use shared `web::Data<Mutex<State>>`.",
  },
];

export function getLessonBySlug(slug: string): Lesson | undefined {
  return LESSONS.find((l) => l.slug === slug);
}

export function getLessonById(id: number): Lesson | undefined {
  return LESSONS.find((l) => l.id === id);
}

export function getLessonsByTrack(track: Track): Lesson[] {
  return LESSONS.filter((l) => l.track === track);
}

export function getTrackProgress(track: Track, completedIds: number[]): number {
  const lessons = getLessonsByTrack(track);
  if (lessons.length === 0) return 0;
  const done = lessons.filter((l) => completedIds.includes(l.id)).length;
  return Math.round((done / lessons.length) * 100);
}

export function getNextLesson(currentId: number): Lesson | undefined {
  const idx = LESSONS.findIndex((l) => l.id === currentId);
  return LESSONS[idx + 1];
}

export function getPrevLesson(currentId: number): Lesson | undefined {
  const idx = LESSONS.findIndex((l) => l.id === currentId);
  return idx > 0 ? LESSONS[idx - 1] : undefined;
}
