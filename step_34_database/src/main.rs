// ============================================================
// STEP 34: Database Access with SQLite (rusqlite)
// ============================================================
// Run: cargo run
//
// Uses rusqlite with the "bundled" feature so no external
// SQLite installation is needed. Covers:
// - Creating tables and running migrations
// - CRUD operations with parameterized queries
// - Transactions
// - The repository pattern
// - Error handling
// ============================================================

use rusqlite::{params, Connection, Result as SqlResult};
use serde::Serialize;

fn main() -> SqlResult<()> {
    println!("=== Step 34: Database with SQLite ===\n");

    // Use a temp file (or ":memory:" for in-memory)
    let db_path = "/tmp/rust_tutorial_step34.db";
    let conn = Connection::open(db_path)?;

    // Create our repository
    let repo = UserRepository::new(&conn)?;

    // =========================
    // 34.1 Insert Records
    // =========================

    println!("--- INSERT ---");

    let alice_id = repo.create(&NewUser {
        name: "Alice".into(),
        email: "alice@example.com".into(),
        age: 30,
    })?;
    println!("Created Alice with id={}", alice_id);

    let bob_id = repo.create(&NewUser {
        name: "Bob".into(),
        email: "bob@example.com".into(),
        age: 25,
    })?;
    println!("Created Bob with id={}", bob_id);

    repo.create(&NewUser {
        name: "Charlie".into(),
        email: "charlie@example.com".into(),
        age: 35,
    })?;

    repo.create(&NewUser {
        name: "Diana".into(),
        email: "diana@example.com".into(),
        age: 28,
    })?;

    println!("Created 4 users total.");

    // =========================
    // 34.2 Query Records
    // =========================

    println!("\n--- SELECT ---");

    // Get by ID
    if let Some(user) = repo.find_by_id(alice_id)? {
        println!("Found: {:?}", user);
    }

    // Get all
    let all_users = repo.find_all()?;
    println!("\nAll users ({}):", all_users.len());
    for user in &all_users {
        println!("  [{}] {} <{}> age={}", user.id, user.name, user.email, user.age);
    }

    // Search by name
    let results = repo.search_by_name("li")?;
    println!("\nSearch 'li': {} results", results.len());
    for user in &results {
        println!("  {} ({})", user.name, user.email);
    }

    // Find by age range
    let young = repo.find_by_age_range(20, 30)?;
    println!("\nAge 20-30: {} users", young.len());
    for user in &young {
        println!("  {} age {}", user.name, user.age);
    }

    // =========================
    // 34.3 Update Records
    // =========================

    println!("\n--- UPDATE ---");

    let updated = repo.update(alice_id, &UpdateUser {
        name: Some("Alice Smith".into()),
        email: None,
        age: Some(31),
    })?;
    println!("Updated {} row(s)", updated);

    if let Some(user) = repo.find_by_id(alice_id)? {
        println!("Alice is now: {:?}", user);
    }

    // =========================
    // 34.4 Delete Records
    // =========================

    println!("\n--- DELETE ---");

    let deleted = repo.delete(bob_id)?;
    println!("Deleted {} row(s) (Bob)", deleted);

    let remaining = repo.count()?;
    println!("Remaining users: {}", remaining);

    // =========================
    // 34.5 Transactions
    // =========================

    println!("\n--- TRANSACTIONS ---");

    // Successful transaction
    {
        let tx = conn.unchecked_transaction()?;
        let repo = UserRepository::new_with_conn(&tx);

        repo.create_raw(&tx, "Eve", "eve@example.com", 22)?;
        repo.create_raw(&tx, "Frank", "frank@example.com", 40)?;
        tx.commit()?;
        println!("Transaction committed: added Eve and Frank");
    }

    // Failed transaction (rolled back)
    {
        let tx = conn.unchecked_transaction()?;
        let repo = UserRepository::new_with_conn(&tx);

        repo.create_raw(&tx, "Ghost", "ghost@example.com", 0)?;
        // Simulate a failure — just drop tx without committing
        drop(tx);
        println!("Transaction rolled back: Ghost NOT added");
    }

    let final_count = repo.count()?;
    println!("Final user count: {}", final_count);

    // =========================
    // 34.6 Aggregation Queries
    // =========================

    println!("\n--- AGGREGATION ---");

    let stats = repo.stats()?;
    println!("Stats: {:?}", stats);

    // =========================
    // 34.7 Prepared Statements (Performance)
    // =========================

    println!("\n--- BATCH INSERT (prepared) ---");

    let count_before = repo.count()?;

    {
        let tx = conn.unchecked_transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO users (name, email, age) VALUES (?1, ?2, ?3)"
            )?;

            for i in 0..100 {
                stmt.execute(params![
                    format!("BatchUser{}", i),
                    format!("batch{}@example.com", i),
                    20 + (i % 50)
                ])?;
            }
        }
        tx.commit()?;
    }

    let count_after = repo.count()?;
    println!("Inserted {} users in a batch", count_after - count_before);

    // =========================
    // 34.8 Error Handling
    // =========================

    println!("\n--- ERROR HANDLING ---");

    // Duplicate email (if we had a unique constraint):
    match repo.create(&NewUser {
        name: "Alice Duplicate".into(),
        email: "alice@example.com".into(), // already exists
        age: 30,
    }) {
        Ok(id) => println!("Created (no unique constraint): id={}", id),
        Err(e) => println!("Error: {}", e),
    }

    // Query for non-existent user
    match repo.find_by_id(99999)? {
        Some(user) => println!("Found: {:?}", user),
        None => println!("User 99999 not found (as expected)"),
    }

    // Cleanup
    std::fs::remove_file(db_path).ok();
    println!("\n=== Step 34 Complete! ===");

    Ok(())
}

// ============================================================
// Repository Pattern
// ============================================================

#[derive(Debug, Clone, Serialize)]
struct User {
    id: i64,
    name: String,
    email: String,
    age: i32,
}

struct NewUser {
    name: String,
    email: String,
    age: i32,
}

struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
    age: Option<i32>,
}

#[derive(Debug, Serialize)]
struct UserStats {
    total: i64,
    avg_age: f64,
    min_age: i32,
    max_age: i32,
}

struct UserRepository<'a> {
    conn: &'a Connection,
}

impl<'a> UserRepository<'a> {
    fn new(conn: &'a Connection) -> SqlResult<Self> {
        // Run migrations
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS users (
                id    INTEGER PRIMARY KEY AUTOINCREMENT,
                name  TEXT NOT NULL,
                email TEXT NOT NULL,
                age   INTEGER NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
            CREATE INDEX IF NOT EXISTS idx_users_name ON users(name);",
        )?;
        Ok(UserRepository { conn })
    }

    fn new_with_conn(conn: &'a Connection) -> Self {
        UserRepository { conn }
    }

    fn create(&self, user: &NewUser) -> SqlResult<i64> {
        self.conn.execute(
            "INSERT INTO users (name, email, age) VALUES (?1, ?2, ?3)",
            params![user.name, user.email, user.age],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    fn create_raw(&self, conn: &Connection, name: &str, email: &str, age: i32) -> SqlResult<i64> {
        conn.execute(
            "INSERT INTO users (name, email, age) VALUES (?1, ?2, ?3)",
            params![name, email, age],
        )?;
        Ok(conn.last_insert_rowid())
    }

    fn find_by_id(&self, id: i64) -> SqlResult<Option<User>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, email, age FROM users WHERE id = ?1"
        )?;

        let mut rows = stmt.query_map(params![id], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
                age: row.get(3)?,
            })
        })?;

        match rows.next() {
            Some(Ok(user)) => Ok(Some(user)),
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }

    fn find_all(&self) -> SqlResult<Vec<User>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, email, age FROM users ORDER BY id"
        )?;

        let users = stmt
            .query_map([], |row| {
                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                    age: row.get(3)?,
                })
            })?
            .collect::<SqlResult<Vec<_>>>()?;

        Ok(users)
    }

    fn search_by_name(&self, query: &str) -> SqlResult<Vec<User>> {
        let pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT id, name, email, age FROM users WHERE name LIKE ?1"
        )?;

        let users = stmt
            .query_map(params![pattern], |row| {
                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                    age: row.get(3)?,
                })
            })?
            .collect::<SqlResult<Vec<_>>>()?;

        Ok(users)
    }

    fn find_by_age_range(&self, min: i32, max: i32) -> SqlResult<Vec<User>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, email, age FROM users WHERE age BETWEEN ?1 AND ?2 ORDER BY age"
        )?;

        let users = stmt
            .query_map(params![min, max], |row| {
                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                    age: row.get(3)?,
                })
            })?
            .collect::<SqlResult<Vec<_>>>()?;

        Ok(users)
    }

    fn update(&self, id: i64, data: &UpdateUser) -> SqlResult<usize> {
        let current = self.find_by_id(id)?;
        match current {
            Some(user) => {
                let name = data.name.as_deref().unwrap_or(&user.name);
                let email = data.email.as_deref().unwrap_or(&user.email);
                let age = data.age.unwrap_or(user.age);

                self.conn.execute(
                    "UPDATE users SET name = ?1, email = ?2, age = ?3 WHERE id = ?4",
                    params![name, email, age, id],
                )
            }
            None => Ok(0),
        }
    }

    fn delete(&self, id: i64) -> SqlResult<usize> {
        self.conn.execute("DELETE FROM users WHERE id = ?1", params![id])
    }

    fn count(&self) -> SqlResult<i64> {
        self.conn.query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))
    }

    fn stats(&self) -> SqlResult<UserStats> {
        self.conn.query_row(
            "SELECT COUNT(*), AVG(age), MIN(age), MAX(age) FROM users",
            [],
            |row| {
                Ok(UserStats {
                    total: row.get(0)?,
                    avg_age: row.get(1)?,
                    min_age: row.get(2)?,
                    max_age: row.get(3)?,
                })
            },
        )
    }
}

// ============================================================
// WHAT YOU LEARNED:
// - SQLite with rusqlite (bundled, zero-setup)
// - Creating tables and indices
// - CRUD with parameterized queries (SQL injection safe!)
// - Repository pattern for clean data access
// - Transactions (commit and rollback)
// - Batch inserts with prepared statements
// - Aggregation queries
// - Error handling with rusqlite::Result
//
// NEXT STEPS:
// - Add migration versioning
// - Use connection pools (r2d2)
// - Try async with sqlx or sea-orm
// - Add full-text search with SQLite FTS5
// ============================================================
