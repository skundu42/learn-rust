// ============================================================
// STEP 32: Building a Web API with Actix-Web
// ============================================================
// Run: cargo run
// Then test with:
//   curl http://localhost:8080/health
//   curl http://localhost:8080/api/todos
//   curl -X POST http://localhost:8080/api/todos -H "Content-Type: application/json" -d '{"title":"Learn Rust"}'
//   curl http://localhost:8080/api/todos/{id}
//   curl -X PUT http://localhost:8080/api/todos/{id} -H "Content-Type: application/json" -d '{"title":"Learn Rust","completed":true}'
//   curl -X DELETE http://localhost:8080/api/todos/{id}
//
// This builds a complete CRUD REST API with:
// - JSON request/response bodies
// - Path parameters and query strings
// - Shared mutable state (Mutex)
// - Error handling with proper HTTP status codes
// - Middleware (logging)
// ============================================================

use actix_web::{web, App, HttpServer, HttpResponse, middleware};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

// ============================================================
// Data Models
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: String,
    title: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct QueryParams {
    completed: Option<bool>,
    search: Option<String>,
}

// Shared application state
struct AppState {
    todos: Mutex<Vec<Todo>>,
}

// ============================================================
// API Handlers
// ============================================================

// GET /health
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "version": "1.0.0"
    }))
}

// GET /api/todos?completed=true&search=rust
async fn get_todos(
    data: web::Data<AppState>,
    query: web::Query<QueryParams>,
) -> HttpResponse {
    let todos = data.todos.lock().unwrap();

    let filtered: Vec<&Todo> = todos
        .iter()
        .filter(|t| {
            if let Some(completed) = query.completed {
                if t.completed != completed {
                    return false;
                }
            }
            if let Some(ref search) = query.search {
                if !t.title.to_lowercase().contains(&search.to_lowercase()) {
                    return false;
                }
            }
            true
        })
        .collect();

    HttpResponse::Ok().json(serde_json::json!({
        "count": filtered.len(),
        "todos": filtered
    }))
}

// GET /api/todos/{id}
async fn get_todo(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = path.into_inner();
    let todos = data.todos.lock().unwrap();

    match todos.iter().find(|t| t.id == id) {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Todo not found",
            "id": id
        })),
    }
}

// POST /api/todos
async fn create_todo(
    data: web::Data<AppState>,
    body: web::Json<CreateTodo>,
) -> HttpResponse {
    let todo = Todo {
        id: Uuid::new_v4().to_string(),
        title: body.title.clone(),
        completed: false,
    };

    let mut todos = data.todos.lock().unwrap();
    todos.push(todo.clone());

    HttpResponse::Created().json(todo)
}

// PUT /api/todos/{id}
async fn update_todo(
    data: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateTodo>,
) -> HttpResponse {
    let id = path.into_inner();
    let mut todos = data.todos.lock().unwrap();

    match todos.iter_mut().find(|t| t.id == id) {
        Some(todo) => {
            if let Some(ref title) = body.title {
                todo.title = title.clone();
            }
            if let Some(completed) = body.completed {
                todo.completed = completed;
            }
            HttpResponse::Ok().json(todo.clone())
        }
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Todo not found",
            "id": id
        })),
    }
}

// DELETE /api/todos/{id}
async fn delete_todo(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> HttpResponse {
    let id = path.into_inner();
    let mut todos = data.todos.lock().unwrap();
    let len_before = todos.len();
    todos.retain(|t| t.id != id);

    if todos.len() < len_before {
        HttpResponse::Ok().json(serde_json::json!({
            "message": "Todo deleted",
            "id": id
        }))
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "error": "Todo not found",
            "id": id
        }))
    }
}

// GET /api/stats
async fn get_stats(data: web::Data<AppState>) -> HttpResponse {
    let todos = data.todos.lock().unwrap();
    let total = todos.len();
    let completed = todos.iter().filter(|t| t.completed).count();
    let pending = total - completed;

    HttpResponse::Ok().json(serde_json::json!({
        "total": total,
        "completed": completed,
        "pending": pending,
        "completion_rate": if total > 0 {
            (completed as f64 / total as f64 * 100.0).round()
        } else {
            0.0
        }
    }))
}

// ============================================================
// Server Setup
// ============================================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Seed with sample data
    let initial_todos = vec![
        Todo {
            id: Uuid::new_v4().to_string(),
            title: "Learn Rust basics".into(),
            completed: true,
        },
        Todo {
            id: Uuid::new_v4().to_string(),
            title: "Build a web API".into(),
            completed: false,
        },
        Todo {
            id: Uuid::new_v4().to_string(),
            title: "Deploy to production".into(),
            completed: false,
        },
    ];

    let data = web::Data::new(AppState {
        todos: Mutex::new(initial_todos),
    });

    println!("========================================");
    println!("  Todo API Server starting!");
    println!("  http://localhost:8080");
    println!("========================================");
    println!("\nEndpoints:");
    println!("  GET    /health");
    println!("  GET    /api/todos?completed=bool&search=text");
    println!("  GET    /api/todos/{{id}}");
    println!("  POST   /api/todos          {{\"title\": \"...\"}}");
    println!("  PUT    /api/todos/{{id}}     {{\"title\": \"...\", \"completed\": true}}");
    println!("  DELETE /api/todos/{{id}}");
    println!("  GET    /api/stats");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            // Logging middleware
            .wrap(middleware::Logger::default())
            // Routes
            .route("/health", web::get().to(health_check))
            .service(
                web::scope("/api")
                    .route("/todos", web::get().to(get_todos))
                    .route("/todos", web::post().to(create_todo))
                    .route("/todos/{id}", web::get().to(get_todo))
                    .route("/todos/{id}", web::put().to(update_todo))
                    .route("/todos/{id}", web::delete().to(delete_todo))
                    .route("/stats", web::get().to(get_stats)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// ============================================================
// WHAT YOU LEARNED:
// - Setting up an Actix-Web HTTP server
// - Defining routes with path params & query strings
// - JSON request/response with serde
// - Shared mutable state with web::Data<Mutex<T>>
// - Proper HTTP status codes (200, 201, 404)
// - Middleware (logging)
//
// NEXT STEPS:
// - Add authentication (JWT tokens)
// - Use a real database instead of in-memory Vec
// - Add request validation
// - Add CORS middleware
// - Add pagination
// - Write integration tests
// ============================================================
