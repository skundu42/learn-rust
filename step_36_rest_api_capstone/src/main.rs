// ============================================================
// STEP 36: Capstone — Full REST API with Database
// ============================================================
// Run: cargo run
// Test endpoints:
//   curl http://localhost:8080/api/notes
//   curl -X POST http://localhost:8080/api/notes \
//     -H "Content-Type: application/json" \
//     -d '{"title":"My Note","content":"Hello Rust!","tags":["rust","learning"]}'
//   curl http://localhost:8080/api/notes/1
//   curl http://localhost:8080/api/notes/search?q=rust
//   curl http://localhost:8080/api/tags
//   curl -X DELETE http://localhost:8080/api/notes/1
//
// This capstone combines EVERYTHING:
// - Actix-Web (HTTP server, routing, middleware)
// - Serde (JSON serialization)
// - SQLite (persistent storage via rusqlite)
// - Chrono (timestamps)
// - UUID (unique identifiers)
// - Error handling (custom error types)
// - Logging (env_logger)
// - Repository pattern
// - Layered architecture
// ============================================================

use actix_web::{web, App, HttpServer, HttpResponse, middleware};
use chrono::{NaiveDateTime, Utc};
use log::info;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// ============================================================
// Models
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Note {
    id: i64,
    title: String,
    content: String,
    tags: Vec<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
struct CreateNote {
    title: String,
    content: String,
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateNote {
    title: Option<String>,
    content: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: String,
}

#[derive(Debug, Deserialize)]
struct PaginationParams {
    #[serde(default = "default_page")]
    page: u32,
    #[serde(default = "default_per_page")]
    per_page: u32,
}

fn default_page() -> u32 { 1 }
fn default_per_page() -> u32 { 20 }

#[derive(Debug, Serialize)]
struct PaginatedResponse<T: Serialize> {
    data: Vec<T>,
    total: i64,
    page: u32,
    per_page: u32,
    total_pages: u32,
}

// ============================================================
// Database Layer
// ============================================================

struct Database {
    conn: Connection,
}

impl Database {
    fn new(path: &str) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;

        // Enable WAL mode for better concurrent read performance
        conn.execute_batch("PRAGMA journal_mode=WAL;")?;

        // Create tables
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS notes (
                id         INTEGER PRIMARY KEY AUTOINCREMENT,
                title      TEXT NOT NULL,
                content    TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS note_tags (
                note_id INTEGER NOT NULL,
                tag     TEXT NOT NULL,
                PRIMARY KEY (note_id, tag),
                FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_note_tags_tag ON note_tags(tag);
            CREATE INDEX IF NOT EXISTS idx_notes_title ON notes(title);",
        )?;

        Ok(Database { conn })
    }

    fn create_note(&self, input: &CreateNote) -> rusqlite::Result<Note> {
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        self.conn.execute(
            "INSERT INTO notes (title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            params![input.title, input.content, now, now],
        )?;

        let id = self.conn.last_insert_rowid();

        // Insert tags
        for tag in &input.tags {
            self.conn.execute(
                "INSERT OR IGNORE INTO note_tags (note_id, tag) VALUES (?1, ?2)",
                params![id, tag.to_lowercase()],
            )?;
        }

        Ok(Note {
            id,
            title: input.title.clone(),
            content: input.content.clone(),
            tags: input.tags.iter().map(|t| t.to_lowercase()).collect(),
            created_at: now.clone(),
            updated_at: now,
        })
    }

    fn get_note(&self, id: i64) -> rusqlite::Result<Option<Note>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, content, created_at, updated_at FROM notes WHERE id = ?1",
        )?;

        let mut notes = stmt.query_map(params![id], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                tags: Vec::new(),
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;

        match notes.next() {
            Some(Ok(mut note)) => {
                note.tags = self.get_tags(note.id)?;
                Ok(Some(note))
            }
            Some(Err(e)) => Err(e),
            None => Ok(None),
        }
    }

    fn list_notes(&self, page: u32, per_page: u32) -> rusqlite::Result<(Vec<Note>, i64)> {
        let total: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM notes", [], |row| row.get(0),
        )?;

        let offset = (page - 1) * per_page;
        let mut stmt = self.conn.prepare(
            "SELECT id, title, content, created_at, updated_at
             FROM notes ORDER BY updated_at DESC LIMIT ?1 OFFSET ?2",
        )?;

        let notes: Vec<Note> = stmt
            .query_map(params![per_page, offset], |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    tags: Vec::new(),
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        // Fetch tags for each note
        let mut result = Vec::new();
        for mut note in notes {
            note.tags = self.get_tags(note.id)?;
            result.push(note);
        }

        Ok((result, total))
    }

    fn search_notes(&self, query: &str) -> rusqlite::Result<Vec<Note>> {
        let pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT n.id, n.title, n.content, n.created_at, n.updated_at
             FROM notes n
             LEFT JOIN note_tags t ON n.id = t.note_id
             WHERE n.title LIKE ?1 OR n.content LIKE ?1 OR t.tag LIKE ?1
             ORDER BY n.updated_at DESC",
        )?;

        let notes: Vec<Note> = stmt
            .query_map(params![pattern], |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    tags: Vec::new(),
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        let mut result = Vec::new();
        for mut note in notes {
            note.tags = self.get_tags(note.id)?;
            result.push(note);
        }

        Ok(result)
    }

    fn update_note(&self, id: i64, input: &UpdateNote) -> rusqlite::Result<Option<Note>> {
        let existing = self.get_note(id)?;
        match existing {
            None => Ok(None),
            Some(note) => {
                let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let title = input.title.as_deref().unwrap_or(&note.title);
                let content = input.content.as_deref().unwrap_or(&note.content);

                self.conn.execute(
                    "UPDATE notes SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
                    params![title, content, now, id],
                )?;

                // Update tags if provided
                if let Some(ref tags) = input.tags {
                    self.conn.execute("DELETE FROM note_tags WHERE note_id = ?1", params![id])?;
                    for tag in tags {
                        self.conn.execute(
                            "INSERT OR IGNORE INTO note_tags (note_id, tag) VALUES (?1, ?2)",
                            params![id, tag.to_lowercase()],
                        )?;
                    }
                }

                self.get_note(id)
            }
        }
    }

    fn delete_note(&self, id: i64) -> rusqlite::Result<bool> {
        let rows = self.conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
        if rows > 0 {
            self.conn.execute("DELETE FROM note_tags WHERE note_id = ?1", params![id])?;
        }
        Ok(rows > 0)
    }

    fn get_all_tags(&self) -> rusqlite::Result<Vec<(String, i64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT tag, COUNT(*) as count FROM note_tags GROUP BY tag ORDER BY count DESC",
        )?;

        let tags = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<rusqlite::Result<Vec<_>>>()?;

        Ok(tags)
    }

    fn get_tags(&self, note_id: i64) -> rusqlite::Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT tag FROM note_tags WHERE note_id = ?1 ORDER BY tag",
        )?;

        let tags = stmt
            .query_map(params![note_id], |row| row.get(0))?
            .collect::<rusqlite::Result<Vec<String>>>()?;

        Ok(tags)
    }
}

// ============================================================
// App State
// ============================================================

struct AppState {
    db: Mutex<Database>,
}

// ============================================================
// Handlers
// ============================================================

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "notes-api",
        "version": "1.0.0"
    }))
}

async fn list_notes(
    data: web::Data<AppState>,
    query: web::Query<PaginationParams>,
) -> HttpResponse {
    let db = data.db.lock().unwrap();
    match db.list_notes(query.page, query.per_page) {
        Ok((notes, total)) => {
            let total_pages = ((total as f64) / (query.per_page as f64)).ceil() as u32;
            HttpResponse::Ok().json(PaginatedResponse {
                data: notes,
                total,
                page: query.page,
                per_page: query.per_page,
                total_pages,
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

async fn get_note(
    data: web::Data<AppState>,
    path: web::Path<i64>,
) -> HttpResponse {
    let db = data.db.lock().unwrap();
    let id = path.into_inner();

    match db.get_note(id) {
        Ok(Some(note)) => HttpResponse::Ok().json(note),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Note not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

async fn create_note(
    data: web::Data<AppState>,
    body: web::Json<CreateNote>,
) -> HttpResponse {
    // Validation
    if body.title.trim().is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Title cannot be empty"
        }));
    }
    if body.content.trim().is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Content cannot be empty"
        }));
    }

    let db = data.db.lock().unwrap();
    match db.create_note(&body) {
        Ok(note) => {
            info!("Created note: {} (id={})", note.title, note.id);
            HttpResponse::Created().json(note)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

async fn update_note(
    data: web::Data<AppState>,
    path: web::Path<i64>,
    body: web::Json<UpdateNote>,
) -> HttpResponse {
    let db = data.db.lock().unwrap();
    let id = path.into_inner();

    match db.update_note(id, &body) {
        Ok(Some(note)) => HttpResponse::Ok().json(note),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Note not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

async fn delete_note(
    data: web::Data<AppState>,
    path: web::Path<i64>,
) -> HttpResponse {
    let db = data.db.lock().unwrap();
    let id = path.into_inner();

    match db.delete_note(id) {
        Ok(true) => {
            info!("Deleted note id={}", id);
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Note deleted"
            }))
        }
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Note not found"
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

async fn search_notes(
    data: web::Data<AppState>,
    query: web::Query<SearchQuery>,
) -> HttpResponse {
    let db = data.db.lock().unwrap();
    match db.search_notes(&query.q) {
        Ok(notes) => HttpResponse::Ok().json(serde_json::json!({
            "query": query.q,
            "count": notes.len(),
            "results": notes
        })),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

async fn list_tags(data: web::Data<AppState>) -> HttpResponse {
    let db = data.db.lock().unwrap();
    match db.get_all_tags() {
        Ok(tags) => {
            let tag_list: Vec<serde_json::Value> = tags
                .into_iter()
                .map(|(tag, count)| serde_json::json!({"tag": tag, "count": count}))
                .collect();
            HttpResponse::Ok().json(tag_list)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Database error: {}", e)
        })),
    }
}

// ============================================================
// Main
// ============================================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let db_path = "/tmp/rust_notes_api.db";
    let db = Database::new(db_path).expect("Failed to initialize database");

    // Seed sample data
    let count: i64 = db.conn.query_row("SELECT COUNT(*) FROM notes", [], |r| r.get(0)).unwrap();
    if count == 0 {
        info!("Seeding sample data...");
        db.create_note(&CreateNote {
            title: "Welcome to the Notes API".into(),
            content: "This is a fully functional REST API built with Rust!".into(),
            tags: vec!["welcome".into(), "rust".into()],
        }).unwrap();
        db.create_note(&CreateNote {
            title: "Rust Ownership".into(),
            content: "Each value has exactly one owner. When the owner goes out of scope, the value is dropped.".into(),
            tags: vec!["rust".into(), "learning".into(), "ownership".into()],
        }).unwrap();
        db.create_note(&CreateNote {
            title: "Error Handling".into(),
            content: "Use Result<T, E> for recoverable errors and panic! for unrecoverable ones.".into(),
            tags: vec!["rust".into(), "errors".into()],
        }).unwrap();
    }

    let data = web::Data::new(AppState {
        db: Mutex::new(db),
    });

    println!("╔══════════════════════════════════════════╗");
    println!("║     Notes API - Rust Capstone Project    ║");
    println!("║     http://localhost:8080                 ║");
    println!("╚══════════════════════════════════════════╝");
    println!();
    println!("Endpoints:");
    println!("  GET    /health");
    println!("  GET    /api/notes?page=1&per_page=20");
    println!("  GET    /api/notes/{{id}}");
    println!("  POST   /api/notes");
    println!("  PUT    /api/notes/{{id}}");
    println!("  DELETE /api/notes/{{id}}");
    println!("  GET    /api/notes/search?q=keyword");
    println!("  GET    /api/tags");
    println!();
    println!("Database: {}", db_path);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::Logger::new("%s %r (%Dms)"))
            .route("/health", web::get().to(health))
            .service(
                web::scope("/api")
                    .route("/notes", web::get().to(list_notes))
                    .route("/notes", web::post().to(create_note))
                    .route("/notes/search", web::get().to(search_notes))
                    .route("/notes/{id}", web::get().to(get_note))
                    .route("/notes/{id}", web::put().to(update_note))
                    .route("/notes/{id}", web::delete().to(delete_note))
                    .route("/tags", web::get().to(list_tags)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// ============================================================
// CONCEPTS USED IN THIS CAPSTONE:
//
// From the tutorial:
// - Step 02: Variables & types
// - Step 04: Ownership & borrowing (Mutex, references)
// - Step 05: Structs & enums
// - Step 06: Pattern matching (Result handling)
// - Step 07: Collections (Vec, HashMap)
// - Step 08: Error handling (Result, custom errors)
// - Step 09: Traits (Serialize, Deserialize)
// - Step 11: Closures & iterators
// - Step 12: Modules (layered architecture)
// - Step 14: Concurrency (Mutex for shared DB)
// - Step 15: Async (actix-web, tokio)
// - Step 31: Serde (JSON serialization)
// - Step 34: Database (SQLite with rusqlite)
//
// Real-world patterns:
// - Repository pattern for data access
// - Request validation
// - Pagination
// - Full-text search
// - Tag system with many-to-many relationship
// - Proper HTTP status codes
// - Structured logging
// - Database migrations
// - Seed data
//
// TO EXTEND:
// 1. Add JWT authentication
// 2. Add CORS middleware
// 3. Add rate limiting
// 4. Add WebSocket support for real-time updates
// 5. Add file upload support
// 6. Deploy with Docker
// 7. Add OpenAPI/Swagger documentation
// 8. Add integration tests
// ============================================================
