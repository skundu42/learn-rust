// ============================================================
// STEP 18: Capstone Project — CLI Task Manager
// ============================================================
// Run: rustc main.rs && ./main
//
// This project combines concepts from ALL previous steps:
// - Structs & Enums (step 05)
// - Pattern Matching (step 06)
// - Collections (step 07)
// - Error Handling (step 08)
// - Traits & Generics (step 09)
// - Closures & Iterators (step 11)
// - Modules (step 12)
//
// Features:
// - Add, complete, delete, list tasks
// - Filter by status, priority, or search text
// - Save/load from a JSON-like file
// - Statistics and reporting
// ============================================================

use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io::{self, Write};

// ============================================================
// Data Model
// ============================================================

#[derive(Debug, Clone, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    fn from_str(s: &str) -> Option<Priority> {
        match s.to_lowercase().as_str() {
            "low" | "l" => Some(Priority::Low),
            "medium" | "med" | "m" => Some(Priority::Medium),
            "high" | "h" => Some(Priority::High),
            "critical" | "crit" | "c" => Some(Priority::Critical),
            _ => None,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
            Priority::Critical => "Critical",
        }
    }

    fn emoji(&self) -> &str {
        match self {
            Priority::Low => " ",
            Priority::Medium => "*",
            Priority::High => "!",
            Priority::Critical => "!!",
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    fn from_str(s: &str) -> Option<Status> {
        match s.to_lowercase().as_str() {
            "todo" | "t" => Some(Status::Todo),
            "inprogress" | "in_progress" | "ip" | "wip" => Some(Status::InProgress),
            "done" | "d" => Some(Status::Done),
            _ => None,
        }
    }

    fn symbol(&self) -> &str {
        match self {
            Status::Todo => "[ ]",
            Status::InProgress => "[~]",
            Status::Done => "[x]",
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Todo => write!(f, "Todo"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Done => write!(f, "Done"),
        }
    }
}

#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    description: Option<String>,
    priority: Priority,
    status: Status,
    tags: Vec<String>,
}

impl Task {
    fn new(id: u32, title: String, priority: Priority) -> Self {
        Task {
            id,
            title,
            description: None,
            priority,
            status: Status::Todo,
            tags: Vec::new(),
        }
    }

    fn with_description(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }

    fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    // Serialize to a simple text format
    fn to_line(&self) -> String {
        let desc = self.description.as_deref().unwrap_or("");
        let tags = self.tags.join(",");
        format!(
            "{}|{}|{}|{}|{}|{}",
            self.id,
            self.title,
            desc,
            self.priority.as_str(),
            self.status,
            tags
        )
    }

    // Deserialize from a text line
    fn from_line(line: &str) -> Result<Task, String> {
        let parts: Vec<&str> = line.splitn(6, '|').collect();
        if parts.len() < 5 {
            return Err(format!("Invalid task line: {}", line));
        }

        let id = parts[0]
            .parse::<u32>()
            .map_err(|e| format!("Invalid id: {}", e))?;
        let title = parts[1].to_string();
        let description = if parts[2].is_empty() {
            None
        } else {
            Some(parts[2].to_string())
        };
        let priority =
            Priority::from_str(parts[3]).ok_or_else(|| format!("Invalid priority: {}", parts[3]))?;
        let status =
            Status::from_str(parts[4]).ok_or_else(|| format!("Invalid status: {}", parts[4]))?;
        let tags = if parts.len() > 5 && !parts[5].is_empty() {
            parts[5].split(',').map(String::from).collect()
        } else {
            Vec::new()
        };

        Ok(Task {
            id,
            title,
            description,
            priority,
            status,
            tags,
        })
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {:>3}. {} [{}] {}",
            self.status.symbol(),
            self.id,
            self.priority.emoji(),
            self.priority,
            self.title
        )?;
        if !self.tags.is_empty() {
            write!(f, " #{}", self.tags.join(" #"))?;
        }
        if let Some(desc) = &self.description {
            write!(f, "\n         {}", desc)?;
        }
        Ok(())
    }
}

// ============================================================
// Task Manager — Uses Traits, Generics, Iterators
// ============================================================

// Trait for anything that can filter tasks
trait TaskFilter {
    fn matches(&self, task: &Task) -> bool;
}

// Filter by status
struct StatusFilter(Status);
impl TaskFilter for StatusFilter {
    fn matches(&self, task: &Task) -> bool {
        task.status == self.0
    }
}

// Filter by priority
struct PriorityFilter(Priority);
impl TaskFilter for PriorityFilter {
    fn matches(&self, task: &Task) -> bool {
        task.priority == self.0
    }
}

// Filter by search text
struct SearchFilter(String);
impl TaskFilter for SearchFilter {
    fn matches(&self, task: &Task) -> bool {
        let query = self.0.to_lowercase();
        task.title.to_lowercase().contains(&query)
            || task
                .description
                .as_ref()
                .map_or(false, |d| d.to_lowercase().contains(&query))
            || task.tags.iter().any(|t| t.to_lowercase().contains(&query))
    }
}

// The main task manager
struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
    file_path: String,
}

impl TaskManager {
    fn new(file_path: &str) -> Self {
        let mut manager = TaskManager {
            tasks: Vec::new(),
            next_id: 1,
            file_path: file_path.to_string(),
        };
        if let Err(e) = manager.load() {
            if e.kind() != io::ErrorKind::NotFound {
                eprintln!("Warning: Could not load tasks: {}", e);
            }
        }
        manager
    }

    fn add(&mut self, task: Task) -> u32 {
        let id = task.id;
        self.tasks.push(task);
        if id >= self.next_id {
            self.next_id = id + 1;
        }
        id
    }

    fn create_task(&mut self, title: String, priority: Priority) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let task = Task::new(id, title, priority);
        self.tasks.push(task);
        id
    }

    fn complete_task(&mut self, id: u32) -> Result<(), String> {
        self.tasks
            .iter_mut()
            .find(|t| t.id == id)
            .map(|t| t.status = Status::Done)
            .ok_or_else(|| format!("Task {} not found", id))
    }

    fn start_task(&mut self, id: u32) -> Result<(), String> {
        self.tasks
            .iter_mut()
            .find(|t| t.id == id)
            .map(|t| t.status = Status::InProgress)
            .ok_or_else(|| format!("Task {} not found", id))
    }

    fn delete_task(&mut self, id: u32) -> Result<Task, String> {
        let pos = self
            .tasks
            .iter()
            .position(|t| t.id == id)
            .ok_or_else(|| format!("Task {} not found", id))?;
        Ok(self.tasks.remove(pos))
    }

    fn get_task(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }

    // Generic filter using our trait
    fn filter(&self, filter: &dyn TaskFilter) -> Vec<&Task> {
        self.tasks.iter().filter(|t| filter.matches(t)).collect()
    }

    // Filter with closure (functional style)
    fn filter_by<F>(&self, predicate: F) -> Vec<&Task>
    where
        F: Fn(&Task) -> bool,
    {
        self.tasks.iter().filter(|t| predicate(t)).collect()
    }

    fn all_tasks(&self) -> &[Task] {
        &self.tasks
    }

    // Statistics using iterators
    fn stats(&self) -> Stats {
        let total = self.tasks.len();
        let done = self.tasks.iter().filter(|t| t.status == Status::Done).count();
        let in_progress = self
            .tasks
            .iter()
            .filter(|t| t.status == Status::InProgress)
            .count();
        let todo = total - done - in_progress;

        let by_priority: HashMap<String, usize> = {
            let mut map = HashMap::new();
            for task in &self.tasks {
                *map.entry(task.priority.as_str().to_string()).or_insert(0) += 1;
            }
            map
        };

        let completion_rate = if total > 0 {
            (done as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        Stats {
            total,
            todo,
            in_progress,
            done,
            completion_rate,
            by_priority,
        }
    }

    // Save to file
    fn save(&self) -> io::Result<()> {
        let content: String = self
            .tasks
            .iter()
            .map(|t| t.to_line())
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(&self.file_path, content)
    }

    // Load from file
    fn load(&mut self) -> io::Result<()> {
        let content = fs::read_to_string(&self.file_path)?;
        self.tasks.clear();
        self.next_id = 1;

        for line in content.lines() {
            if line.trim().is_empty() {
                continue;
            }
            match Task::from_line(line) {
                Ok(task) => {
                    if task.id >= self.next_id {
                        self.next_id = task.id + 1;
                    }
                    self.tasks.push(task);
                }
                Err(e) => eprintln!("Warning: Skipping invalid line: {}", e),
            }
        }
        Ok(())
    }
}

struct Stats {
    total: usize,
    todo: usize,
    in_progress: usize,
    done: usize,
    completion_rate: f64,
    by_priority: HashMap<String, usize>,
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Task Statistics ===")?;
        writeln!(f, "Total:       {}", self.total)?;
        writeln!(f, "Todo:        {}", self.todo)?;
        writeln!(f, "In Progress: {}", self.in_progress)?;
        writeln!(f, "Done:        {}", self.done)?;
        writeln!(f, "Completion:  {:.1}%", self.completion_rate)?;
        writeln!(f, "By Priority:")?;
        for (priority, count) in &self.by_priority {
            writeln!(f, "  {}: {}", priority, count)?;
        }
        Ok(())
    }
}

// ============================================================
// CLI Interface
// ============================================================

fn print_help() {
    println!("Commands:");
    println!("  add <title>          Add a new task");
    println!("  done <id>            Mark task as done");
    println!("  start <id>           Mark task as in progress");
    println!("  delete <id>          Delete a task");
    println!("  list                 List all tasks");
    println!("  list todo            List todo tasks");
    println!("  list done            List completed tasks");
    println!("  list wip             List in-progress tasks");
    println!("  search <query>       Search tasks");
    println!("  stats                Show statistics");
    println!("  help                 Show this help");
    println!("  quit                 Save and exit");
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn run_interactive(manager: &mut TaskManager) {
    println!("=== Task Manager ===");
    println!("Type 'help' for commands.\n");

    loop {
        let input = prompt("> ");
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0].to_lowercase();
        let args = if parts.len() > 1 { parts[1] } else { "" };

        match command.as_str() {
            "add" | "a" => {
                if args.is_empty() {
                    println!("Usage: add <title>");
                    continue;
                }
                let priority_str = prompt("Priority (low/medium/high/critical) [medium]: ");
                let priority = if priority_str.is_empty() {
                    Priority::Medium
                } else {
                    match Priority::from_str(&priority_str) {
                        Some(p) => p,
                        None => {
                            println!("Invalid priority, using Medium");
                            Priority::Medium
                        }
                    }
                };

                let desc = prompt("Description (optional): ");
                let tags_str = prompt("Tags (comma-separated, optional): ");

                let mut task = Task::new(manager.next_id, args.to_string(), priority);
                manager.next_id += 1;

                if !desc.is_empty() {
                    task = task.with_description(desc);
                }
                if !tags_str.is_empty() {
                    let tags: Vec<String> = tags_str.split(',').map(|s| s.trim().to_string()).collect();
                    task = task.with_tags(tags);
                }

                let id = manager.add(task);
                println!("Created task #{}", id);
            }

            "done" | "complete" => {
                if let Ok(id) = args.parse::<u32>() {
                    match manager.complete_task(id) {
                        Ok(()) => println!("Task #{} marked as done", id),
                        Err(e) => println!("Error: {}", e),
                    }
                } else {
                    println!("Usage: done <id>");
                }
            }

            "start" | "wip" => {
                if let Ok(id) = args.parse::<u32>() {
                    match manager.start_task(id) {
                        Ok(()) => println!("Task #{} started", id),
                        Err(e) => println!("Error: {}", e),
                    }
                } else {
                    println!("Usage: start <id>");
                }
            }

            "delete" | "rm" => {
                if let Ok(id) = args.parse::<u32>() {
                    match manager.delete_task(id) {
                        Ok(task) => println!("Deleted: {}", task.title),
                        Err(e) => println!("Error: {}", e),
                    }
                } else {
                    println!("Usage: delete <id>");
                }
            }

            "list" | "ls" => {
                let tasks: Vec<&Task> = match args.to_lowercase().as_str() {
                    "todo" | "t" => manager.filter(&StatusFilter(Status::Todo)),
                    "done" | "d" => manager.filter(&StatusFilter(Status::Done)),
                    "wip" | "ip" | "inprogress" => {
                        manager.filter(&StatusFilter(Status::InProgress))
                    }
                    "high" | "h" => manager.filter(&PriorityFilter(Priority::High)),
                    "critical" | "crit" => manager.filter(&PriorityFilter(Priority::Critical)),
                    _ => manager.all_tasks().iter().collect(),
                };

                if tasks.is_empty() {
                    println!("No tasks found.");
                } else {
                    for task in &tasks {
                        println!("{}", task);
                    }
                    println!("\n({} tasks)", tasks.len());
                }
            }

            "search" | "find" => {
                if args.is_empty() {
                    println!("Usage: search <query>");
                    continue;
                }
                let results = manager.filter(&SearchFilter(args.to_string()));
                if results.is_empty() {
                    println!("No tasks matching '{}'", args);
                } else {
                    for task in &results {
                        println!("{}", task);
                    }
                    println!("\n({} results)", results.len());
                }
            }

            "show" | "view" => {
                if let Ok(id) = args.parse::<u32>() {
                    match manager.get_task(id) {
                        Some(task) => {
                            println!("{}", task);
                        }
                        None => println!("Task #{} not found", id),
                    }
                } else {
                    println!("Usage: show <id>");
                }
            }

            "stats" => {
                println!("{}", manager.stats());
            }

            "save" => {
                match manager.save() {
                    Ok(()) => println!("Tasks saved to {}", manager.file_path),
                    Err(e) => println!("Error saving: {}", e),
                }
            }

            "help" | "h" | "?" => print_help(),

            "quit" | "exit" | "q" => {
                match manager.save() {
                    Ok(()) => println!("Tasks saved. Goodbye!"),
                    Err(e) => println!("Warning: Could not save: {}", e),
                }
                break;
            }

            "" => continue,

            _ => println!("Unknown command: '{}'. Type 'help' for commands.", command),
        }
    }
}

// ============================================================
// Main — Demo Mode + Interactive Mode
// ============================================================

fn main() {
    println!("============================================");
    println!("  CAPSTONE PROJECT: CLI Task Manager");
    println!("  Combining all Rust concepts learned!");
    println!("============================================\n");

    // --- Demo: show features programmatically ---

    let file_path = "/tmp/rust_tasks.txt";
    let mut manager = TaskManager::new(file_path);

    // Add some demo tasks
    let id1 = manager.create_task("Learn Rust basics".into(), Priority::High);
    let id2 = manager.create_task("Build a CLI app".into(), Priority::Medium);
    let _id3 = manager.create_task("Write tests".into(), Priority::Medium);
    let _id4 = manager.create_task("Deploy to production".into(), Priority::Critical);
    let id5 = manager.add(
        Task::new(manager.next_id, "Read Rust book".into(), Priority::Low)
            .with_description("The official Rust Programming Language book".into())
            .with_tags(vec!["reading".into(), "learning".into()]),
    );
    manager.next_id += 1;

    // Complete and start some tasks
    manager.complete_task(id1).unwrap();
    manager.start_task(id2).unwrap();

    // Display all tasks
    println!("--- All Tasks ---");
    for task in manager.all_tasks() {
        println!("{}", task);
    }

    // Filter examples
    println!("\n--- Todo Tasks ---");
    for task in manager.filter(&StatusFilter(Status::Todo)) {
        println!("{}", task);
    }

    println!("\n--- High Priority ---");
    for task in manager.filter(&PriorityFilter(Priority::High)) {
        println!("{}", task);
    }

    // Search
    println!("\n--- Search 'rust' ---");
    for task in manager.filter(&SearchFilter("rust".into())) {
        println!("{}", task);
    }

    // Closure-based filter
    println!("\n--- Custom Filter (not done + has tags) ---");
    for task in manager.filter_by(|t| t.status != Status::Done && !t.tags.is_empty()) {
        println!("{}", task);
    }

    // Stats
    println!("\n{}", manager.stats());

    // Save
    manager.save().expect("Failed to save");
    println!("Tasks saved to {}\n", file_path);

    // --- Interactive Mode ---

    println!("============================================");
    println!("  Entering Interactive Mode");
    println!("============================================\n");

    run_interactive(&mut manager);

    // Cleanup
    let _ = fs::remove_file(file_path);
}

// ============================================================
// CONCEPTS USED IN THIS PROJECT:
//
// Step 02: Variables & types (id, title, status fields)
// Step 03: Functions & control flow (match on commands, loops)
// Step 04: Ownership & borrowing (&self, &mut self, references)
// Step 05: Structs & enums (Task, Priority, Status)
// Step 06: Pattern matching (command parsing, status matching)
// Step 07: Collections (Vec, HashMap for tasks and stats)
// Step 08: Error handling (Result, Option throughout)
// Step 09: Traits & generics (TaskFilter trait, generic filter_by)
// Step 10: Lifetimes (implicit in references and slices)
// Step 11: Closures & iterators (filter, map, collect chains)
// Step 12: Modules (organized in sections within this file)
//
// FURTHER IMPROVEMENTS TO TRY:
// 1. Add due dates using chrono crate
// 2. Add colored output using colored crate
// 3. Use serde for JSON serialization
// 4. Add subtasks (tree structure)
// 5. Add undo/redo with a command history
// 6. Make it a proper Cargo project with multiple files
// 7. Add unit tests and integration tests
// 8. Use clap for command-line argument parsing
// ============================================================
