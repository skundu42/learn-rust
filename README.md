# Learn Rust: From Basics to Advanced

A hands-on, step-by-step Rust tutorial with 36 lessons covering fundamentals, advanced concepts, data structures & algorithms, and practical real-world projects.

## How to Use

1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Go through each step in order
3. Read the comments in each `.rs` file — they explain every concept
4. Try modifying the examples and re-running them

## Web App Setup

1. Install frontend dependencies: `pnpm install`
2. Copy `.env.example` to `.env.local`
3. Fill in:
   - `NEXT_PUBLIC_SUPABASE_URL`
   - `NEXT_PUBLIC_SUPABASE_ANON_KEY`
   - `NEXT_PUBLIC_SITE_URL` (`http://localhost:3000` in local development)
4. In Supabase SQL Editor, run:
   - `scripts/001_create_profiles.sql`
   - `scripts/002_profile_trigger.sql`
   - `scripts/003_create_progress.sql`
   - `scripts/004_upgrade_progress_tracking.sql` if you already created `lesson_progress`
5. In Supabase Auth URL settings, add `http://localhost:3000/auth/callback` as a redirect URL
6. Start the app with `pnpm dev`

The app now stores per-user lesson progress with `started_at`, `last_viewed_at`,
`completed_at`, `status`, and `visit_count` in `public.lesson_progress`.

### Running Steps 01–30 (Single Files)

```bash
rustc --edition 2021 step_XX_name/main.rs -o stepXX && ./stepXX
```

### Running Steps 31–36 (Cargo Projects)

```bash
cd step_31_serde_serialization && cargo run
cd step_32_web_api && cargo run            # starts server on :8080
cd step_33_cli_apps && cargo run -- --help  # CLI with subcommands
cd step_34_database && cargo run
cd step_35_testing && cargo test            # run the test suite
cd step_36_rest_api_capstone && cargo run   # starts server on :8080
```

## Steps

### Fundamentals (Steps 01–12)

| Step | Topic | Key Concepts |
|------|-------|-------------|
| 01 | Hello World & Basics | println!, formatting, comments, escape characters |
| 02 | Variables, Types & Mutability | let, mut, scalar types, compounds, type conversions |
| 03 | Functions & Control Flow | Functions, expressions vs statements, if/else, loops |
| 04 | Ownership & Borrowing | THE core concept: move semantics, references, slices |
| 05 | Structs & Enums | Struct definitions, methods, impl blocks, Option\<T\> |
| 06 | Pattern Matching | match, guards, @binding, if let, while let, let-else |
| 07 | Collections | Vec, String, HashMap, VecDeque, HashSet, BTreeMap |
| 08 | Error Handling | Result, panic!, ? operator, custom errors, From trait |
| 09 | Traits & Generics | Trait definitions, bounds, impl Trait, dyn Trait |
| 10 | Lifetimes | Annotations, elision rules, struct lifetimes, 'static |
| 11 | Closures & Iterators | Fn/FnMut/FnOnce, iterator adaptors, custom iterators |
| 12 | Modules & Crates | mod, pub, use, re-exports, prelude pattern |

### Advanced Rust (Steps 13–20)

| Step | Topic | Key Concepts |
|------|-------|-------------|
| 13 | Smart Pointers | Box, Deref, Drop, Rc, RefCell, Cow, Weak |
| 14 | Concurrency | Threads, channels (mpsc), Mutex, Arc, parallel patterns |
| 15 | Async/Await | Futures, async fn, .await, manual executor |
| 16 | Macros | macro_rules!, fragment types, repetition, DSL macros |
| 17 | Unsafe Rust | Raw pointers, FFI (C interop), mutable statics, unions |
| 18 | Capstone: CLI Task Manager | Interactive app combining all fundamentals |
| 19 | Advanced Traits | Operator overloading, blanket impls, object safety, disambiguation |
| 20 | Type System Patterns | Newtype, builder, typestate, phantom types, const generics |

### Data Structures & Algorithms (Steps 21–30)

| Step | Topic | Key Concepts |
|------|-------|-------------|
| 21 | DSA: Arrays & Strings | Two pointers, sliding window, prefix sum, Kadane's, anagrams |
| 22 | DSA: Linked Lists | Singly linked list, reverse, merge sorted, cycle detection |
| 23 | DSA: Stacks, Queues & Heaps | Valid parentheses, monotonic stack, BinaryHeap, top-k |
| 24 | DSA: Trees & BSTs | Traversals, balanced check, BST ops, serialize/deserialize |
| 25 | DSA: Graphs | BFS, DFS, Dijkstra, topological sort, connected components |
| 26 | DSA: Sorting & Selection | Merge sort, quick sort, heap sort, counting sort, kth element |
| 27 | DSA: Dynamic Programming | Knapsack, LCS, coin change, edit distance, DP patterns |
| 28 | DSA: Interview Patterns | Backtracking, binary search variants, two heaps, intervals |
| 29 | Advanced Iterators & Pipelines | Custom iterators, lazy evaluation, combinator chains |
| 30 | DSA Capstone | Interview study planner combining all DSA concepts |

### Practical Projects (Steps 31–36)

| Step | Topic | Key Concepts |
|------|-------|-------------|
| 31 | Serde Serialization | JSON, TOML, field attributes, enum tagging, custom serializers |
| 32 | Web API with Actix-Web | REST endpoints, CRUD, middleware, shared state, JSON responses |
| 33 | CLI Apps with Clap | Derive macros, subcommands, colored output, progress bars |
| 34 | Database with SQLite | rusqlite, repository pattern, transactions, parameterized queries |
| 35 | Testing in Rust | Unit tests, assertions, panic tests, parameterized tests, TDD |
| 36 | REST API Capstone | Full project: Actix-Web + SQLite + Serde + validation + logging |

## External Dependencies (Steps 31–36)

| Crate | Purpose | Used In |
|-------|---------|---------|
| `serde` / `serde_json` | Serialization framework | Steps 31–36 |
| `toml` | TOML config parsing | Step 31 |
| `actix-web` | Web framework | Steps 32, 36 |
| `clap` | CLI argument parsing | Step 33 |
| `colored` | Terminal colors | Step 33 |
| `indicatif` | Progress bars & spinners | Step 33 |
| `rusqlite` | SQLite database | Steps 34, 36 |
| `uuid` | Unique identifiers | Steps 32, 36 |
| `chrono` | Date/time handling | Step 36 |
| `tokio` | Async runtime | Steps 32, 36 |
| `env_logger` / `log` | Structured logging | Step 36 |
