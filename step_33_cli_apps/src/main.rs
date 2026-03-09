// ============================================================
// STEP 33: CLI Applications with Clap & Friends
// ============================================================
// Run: cargo run -- --help
//      cargo run -- add "Buy groceries" --priority high
//      cargo run -- list
//      cargo run -- list --status pending
//      cargo run -- done 1
//      cargo run -- remove 1
//      cargo run -- stats
//
// Uses:
// - clap: argument parsing with derive macros
// - colored: terminal colors
// - indicatif: progress bars
// - serde_json: data persistence
// ============================================================

use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

// ============================================================
// CLI Definition (clap derive)
// ============================================================

/// A beautiful CLI task manager built with Rust
#[derive(Parser)]
#[command(name = "tasks", version = "1.0", about = "CLI Task Manager")]
struct Cli {
    /// Path to the data file
    #[arg(long, default_value = "/tmp/rust_cli_tasks.json")]
    data: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// Task description
        description: String,

        /// Task priority
        #[arg(short, long, value_enum, default_value_t = PriorityCli::Medium)]
        priority: PriorityCli,

        /// Tags (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,
    },

    /// List all tasks
    List {
        /// Filter by status
        #[arg(short, long, value_enum)]
        status: Option<StatusFilter>,

        /// Filter by priority
        #[arg(short, long, value_enum)]
        priority: Option<PriorityCli>,
    },

    /// Mark a task as done
    Done {
        /// Task ID
        id: u32,
    },

    /// Remove a task
    Remove {
        /// Task ID
        id: u32,
    },

    /// Show statistics
    Stats,

    /// Clear all completed tasks
    Clean,

    /// Demo mode: shows progress bars and colors
    Demo,
}

#[derive(Clone, ValueEnum, Debug, Serialize, Deserialize, PartialEq)]
enum PriorityCli {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, ValueEnum)]
enum StatusFilter {
    Pending,
    Done,
    All,
}

// ============================================================
// Data Model
// ============================================================

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    priority: PriorityCli,
    done: bool,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct TaskStore {
    next_id: u32,
    tasks: Vec<Task>,
}

impl TaskStore {
    fn load(path: &PathBuf) -> Self {
        match fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => TaskStore {
                next_id: 1,
                tasks: Vec::new(),
            },
        }
    }

    fn save(&self, path: &PathBuf) {
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write(path, json).expect("Failed to save tasks");
    }

    fn add(&mut self, description: String, priority: PriorityCli, tags: Vec<String>) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.push(Task {
            id,
            description,
            priority,
            done: false,
            tags,
        });
        id
    }

    fn complete(&mut self, id: u32) -> Option<&Task> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.done = true;
            Some(task)
        } else {
            None
        }
    }

    fn remove(&mut self, id: u32) -> bool {
        let len = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        self.tasks.len() < len
    }

    fn clean_completed(&mut self) -> usize {
        let len = self.tasks.len();
        self.tasks.retain(|t| !t.done);
        len - self.tasks.len()
    }
}

// ============================================================
// Display Helpers
// ============================================================

fn priority_colored(p: &PriorityCli) -> ColoredString {
    match p {
        PriorityCli::Low => "LOW".dimmed(),
        PriorityCli::Medium => "MED".yellow(),
        PriorityCli::High => "HIGH".red(),
        PriorityCli::Critical => "CRIT".red().bold().underline(),
    }
}

fn print_task(task: &Task) {
    let check = if task.done {
        "✓".green().bold()
    } else {
        "○".dimmed()
    };
    let desc = if task.done {
        task.description.dimmed().strikethrough()
    } else {
        task.description.normal()
    };
    let tags = if task.tags.is_empty() {
        String::new()
    } else {
        format!(" {}", task.tags.iter().map(|t| format!("#{}", t).cyan().to_string()).collect::<Vec<_>>().join(" "))
    };

    println!(
        "  {} {:>3} [{}] {}{}",
        check,
        task.id.to_string().bold(),
        priority_colored(&task.priority),
        desc,
        tags,
    );
}

// ============================================================
// Main
// ============================================================

fn main() {
    let cli = Cli::parse();
    let mut store = TaskStore::load(&cli.data);

    match cli.command {
        Commands::Add {
            description,
            priority,
            tags,
        } => {
            let tag_list: Vec<String> = tags
                .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default();

            let id = store.add(description.clone(), priority, tag_list);
            store.save(&cli.data);
            println!("{} Task #{} created: {}", "✓".green().bold(), id, description);
        }

        Commands::List { status, priority } => {
            let filtered: Vec<&Task> = store
                .tasks
                .iter()
                .filter(|t| match &status {
                    Some(StatusFilter::Pending) => !t.done,
                    Some(StatusFilter::Done) => t.done,
                    _ => true,
                })
                .filter(|t| match &priority {
                    Some(p) => t.priority == *p,
                    None => true,
                })
                .collect();

            if filtered.is_empty() {
                println!("{}", "No tasks found.".dimmed());
            } else {
                println!("{}", "Tasks:".bold().underline());
                for task in &filtered {
                    print_task(task);
                }
                println!("\n  {} total", filtered.len().to_string().bold());
            }
        }

        Commands::Done { id } => {
            if let Some(task) = store.complete(id) {
                println!("{} Completed: {}", "✓".green().bold(), task.description);
                store.save(&cli.data);
            } else {
                eprintln!("{} Task #{} not found", "✗".red().bold(), id);
            }
        }

        Commands::Remove { id } => {
            if store.remove(id) {
                println!("{} Task #{} removed", "✓".green().bold(), id);
                store.save(&cli.data);
            } else {
                eprintln!("{} Task #{} not found", "✗".red().bold(), id);
            }
        }

        Commands::Stats => {
            let total = store.tasks.len();
            let done = store.tasks.iter().filter(|t| t.done).count();
            let pending = total - done;
            let rate = if total > 0 {
                done as f64 / total as f64 * 100.0
            } else {
                0.0
            };

            println!("{}", "Task Statistics".bold().underline());
            println!("  Total:      {}", total.to_string().bold());
            println!("  Completed:  {}", done.to_string().green().bold());
            println!("  Pending:    {}", pending.to_string().yellow().bold());
            println!("  Rate:       {:.0}%", rate);

            // Priority breakdown
            println!("\n{}", "By Priority:".bold());
            for p in &[PriorityCli::Critical, PriorityCli::High, PriorityCli::Medium, PriorityCli::Low] {
                let count = store.tasks.iter().filter(|t| t.priority == *p && !t.done).count();
                if count > 0 {
                    println!("  [{}] {} pending", priority_colored(p), count);
                }
            }
        }

        Commands::Clean => {
            let removed = store.clean_completed();
            store.save(&cli.data);
            println!("{} Removed {} completed tasks", "✓".green().bold(), removed);
        }

        Commands::Demo => {
            println!("{}", "\n🦀 Rust CLI Demo".bold().cyan());
            println!("{}", "=================".cyan());

            // Color showcase
            println!("\n{}", "Color Support:".bold());
            println!("  {} {} {} {} {} {} {}",
                "Red".red(), "Green".green(), "Blue".blue(),
                "Yellow".yellow(), "Magenta".magenta(),
                "Cyan".cyan(), "Bold".bold());

            // Progress bar demo
            println!("\n{}", "Progress Bar:".bold());
            let pb = ProgressBar::new(50);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("  [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                    .unwrap()
                    .progress_chars("█▓░"),
            );
            for i in 0..50 {
                pb.set_position(i + 1);
                pb.set_message(format!("Processing item {}...", i + 1));
                thread::sleep(Duration::from_millis(30));
            }
            pb.finish_with_message("Done!");

            // Spinner demo
            println!("\n{}", "Spinner:".bold());
            let sp = ProgressBar::new_spinner();
            sp.set_style(
                ProgressStyle::default_spinner()
                    .template("  {spinner:.green} {msg}")
                    .unwrap(),
            );
            for i in 0..20 {
                sp.set_message(format!("Loading step {}...", i + 1));
                sp.tick();
                thread::sleep(Duration::from_millis(100));
            }
            sp.finish_with_message("Complete!");

            println!("\n{}", "Demo finished! Try the other commands:".bold());
            println!("  cargo run -- add \"My task\" --priority high");
            println!("  cargo run -- list");
            println!("  cargo run -- stats");

            // Cleanup temp file on demo
            let _ = fs::remove_file(&cli.data);
        }
    }
}

// ============================================================
// WHAT YOU LEARNED:
// - clap derive macros for argument parsing
// - Subcommands, flags, and positional arguments
// - colored for terminal output formatting
// - indicatif for progress bars and spinners
// - Persistent file-based storage with serde_json
// - Practical CLI app architecture
//
// EXERCISES:
// 1. Add a "search" subcommand that finds tasks by keyword
// 2. Add due dates and sort by urgency
// 3. Add an "export" subcommand for CSV output
// 4. Add shell completions with clap_complete
// ============================================================
