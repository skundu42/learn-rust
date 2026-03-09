// ============================================================
// STEP 31: Serde — Serialization & Deserialization
// ============================================================
// Run: cargo run
//
// Serde is THE Rust serialization framework. It works with JSON,
// TOML, YAML, MessagePack, and dozens more formats.
// This step covers the most common real-world patterns.
// ============================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main() {
    // =========================
    // 31.1 Basic JSON Serialization
    // =========================

    println!("=== 31.1 Basic JSON ===\n");

    #[derive(Debug, Serialize, Deserialize)]
    struct User {
        name: String,
        age: u32,
        email: String,
    }

    let user = User {
        name: "Alice".into(),
        age: 30,
        email: "alice@example.com".into(),
    };

    // Serialize to JSON string
    let json = serde_json::to_string(&user).unwrap();
    println!("Compact: {}", json);

    // Pretty-print JSON
    let pretty = serde_json::to_string_pretty(&user).unwrap();
    println!("Pretty:\n{}", pretty);

    // Deserialize back
    let parsed: User = serde_json::from_str(&json).unwrap();
    println!("Parsed back: {:?}\n", parsed);

    // =========================
    // 31.2 Renaming & Skipping Fields
    // =========================

    println!("=== 31.2 Field Attributes ===\n");

    #[derive(Debug, Serialize, Deserialize)]
    struct ApiResponse {
        #[serde(rename = "statusCode")]
        status_code: u16,

        #[serde(rename = "errorMessage")]
        error_message: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        debug_info: Option<String>,

        #[serde(default)]
        retry_count: u32,

        #[serde(skip)]
        internal_id: u64,
    }

    let response = ApiResponse {
        status_code: 200,
        error_message: None,
        debug_info: None,
        retry_count: 0,
        internal_id: 999, // will be skipped
    };

    let json = serde_json::to_string_pretty(&response).unwrap();
    println!("Response JSON:\n{}", json);

    // Deserialize with missing optional fields
    let input = r#"{"statusCode": 404, "errorMessage": "Not Found"}"#;
    let parsed: ApiResponse = serde_json::from_str(input).unwrap();
    println!("\nParsed 404: status={}, error={:?}, retry={}",
        parsed.status_code, parsed.error_message, parsed.retry_count);

    // =========================
    // 31.3 Enums in JSON
    // =========================

    println!("\n=== 31.3 Enum Serialization ===\n");

    // Default enum representation: externally tagged
    #[derive(Debug, Serialize, Deserialize)]
    enum Shape {
        Circle { radius: f64 },
        Rectangle { width: f64, height: f64 },
        Triangle { base: f64, height: f64 },
    }

    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 10.0, height: 3.0 },
        Shape::Triangle { base: 6.0, height: 4.0 },
    ];

    let json = serde_json::to_string_pretty(&shapes).unwrap();
    println!("Shapes (externally tagged):\n{}", json);

    // Internally tagged enum (common for API responses)
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(tag = "type")]
    enum Event {
        #[serde(rename = "user_login")]
        UserLogin { user_id: u64, ip: String },
        #[serde(rename = "purchase")]
        Purchase { user_id: u64, amount: f64, item: String },
        #[serde(rename = "logout")]
        Logout { user_id: u64 },
    }

    let events = vec![
        Event::UserLogin { user_id: 1, ip: "192.168.1.1".into() },
        Event::Purchase { user_id: 1, amount: 29.99, item: "Rust Book".into() },
        Event::Logout { user_id: 1 },
    ];

    let json = serde_json::to_string_pretty(&events).unwrap();
    println!("\nEvents (internally tagged):\n{}", json);

    // Round-trip test
    let parsed: Vec<Event> = serde_json::from_str(&json).unwrap();
    println!("Parsed {} events back", parsed.len());

    // =========================
    // 31.4 Nested Structures
    // =========================

    println!("\n=== 31.4 Nested Structures ===\n");

    #[derive(Debug, Serialize, Deserialize)]
    struct Address {
        street: String,
        city: String,
        country: String,
        #[serde(rename = "zipCode")]
        zip_code: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Company {
        name: String,
        founded: u32,
        address: Address,
        employees: Vec<Employee>,
        #[serde(default)]
        metadata: HashMap<String, String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Employee {
        name: String,
        role: String,
        #[serde(rename = "yearsExp")]
        years_exp: u32,
    }

    let company = Company {
        name: "Rustacean Corp".into(),
        founded: 2015,
        address: Address {
            street: "123 Crab Lane".into(),
            city: "Ferris Town".into(),
            country: "US".into(),
            zip_code: "12345".into(),
        },
        employees: vec![
            Employee { name: "Alice".into(), role: "CTO".into(), years_exp: 10 },
            Employee { name: "Bob".into(), role: "Engineer".into(), years_exp: 5 },
        ],
        metadata: {
            let mut m = HashMap::new();
            m.insert("industry".into(), "Technology".into());
            m
        },
    };

    let json = serde_json::to_string_pretty(&company).unwrap();
    println!("Company:\n{}", json);

    // =========================
    // 31.5 Dynamic JSON (serde_json::Value)
    // =========================

    println!("\n=== 31.5 Dynamic JSON (Value) ===\n");

    let raw = r#"{
        "name": "Unknown API",
        "version": 2,
        "features": ["auth", "logging"],
        "config": { "timeout": 30, "retries": 3 }
    }"#;

    let value: serde_json::Value = serde_json::from_str(raw).unwrap();
    println!("Name: {}", value["name"]);
    println!("Version: {}", value["version"]);
    println!("First feature: {}", value["features"][0]);
    println!("Timeout: {}", value["config"]["timeout"]);

    if let Some(name) = value["name"].as_str() {
        println!("Name is a string: {}", name);
    }

    // Build JSON dynamically with json! macro:
    let dynamic = serde_json::json!({
        "status": "ok",
        "count": 42,
        "items": ["a", "b", "c"],
        "nested": { "key": "value" }
    });
    println!("\nDynamic JSON:\n{}", serde_json::to_string_pretty(&dynamic).unwrap());

    // =========================
    // 31.6 TOML (Config Files)
    // =========================

    println!("\n=== 31.6 TOML ===\n");

    #[derive(Debug, Serialize, Deserialize)]
    struct AppConfig {
        app_name: String,
        version: String,
        debug: bool,
        server: ServerConfig,
        database: DatabaseConfig,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct ServerConfig {
        host: String,
        port: u16,
        workers: u32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct DatabaseConfig {
        url: String,
        max_connections: u32,
    }

    let config = AppConfig {
        app_name: "MyApp".into(),
        version: "1.0.0".into(),
        debug: true,
        server: ServerConfig {
            host: "0.0.0.0".into(),
            port: 8080,
            workers: 4,
        },
        database: DatabaseConfig {
            url: "postgres://localhost/mydb".into(),
            max_connections: 10,
        },
    };

    let toml_str = toml::to_string_pretty(&config).unwrap();
    println!("TOML config:\n{}", toml_str);

    let parsed: AppConfig = toml::from_str(&toml_str).unwrap();
    println!("Parsed: {} v{} on port {}",
        parsed.app_name, parsed.version, parsed.server.port);

    // =========================
    // 31.7 Custom Serialization
    // =========================

    println!("\n=== 31.7 Custom Serialization ===\n");

    #[derive(Debug)]
    struct HexColor { r: u8, g: u8, b: u8 }

    impl Serialize for HexColor {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_str(&format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b))
        }
    }

    impl<'de> Deserialize<'de> for HexColor {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = String::deserialize(deserializer)?;
            let s = s.trim_start_matches('#');
            if s.len() != 6 {
                return Err(serde::de::Error::custom("expected 6 hex digits"));
            }
            let r = u8::from_str_radix(&s[0..2], 16).map_err(serde::de::Error::custom)?;
            let g = u8::from_str_radix(&s[2..4], 16).map_err(serde::de::Error::custom)?;
            let b = u8::from_str_radix(&s[4..6], 16).map_err(serde::de::Error::custom)?;
            Ok(HexColor { r, g, b })
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Theme {
        name: String,
        background: HexColor,
        foreground: HexColor,
    }

    let theme = Theme {
        name: "Dark Mode".into(),
        background: HexColor { r: 30, g: 30, b: 30 },
        foreground: HexColor { r: 220, g: 220, b: 220 },
    };

    let json = serde_json::to_string_pretty(&theme).unwrap();
    println!("Theme JSON:\n{}", json);

    let parsed: Theme = serde_json::from_str(&json).unwrap();
    println!("Parsed bg: {:?}", parsed.background);

    // =========================
    // 31.8 Error Handling
    // =========================

    println!("\n=== 31.8 Error Handling ===\n");

    let bad_json = r#"{"name": "test", "age": "not_a_number", "email": "x"}"#;
    match serde_json::from_str::<User>(bad_json) {
        Ok(u) => println!("Parsed: {:?}", u),
        Err(e) => println!("Type mismatch: {}", e),
    }

    let incomplete = r#"{"name": "test"}"#;
    match serde_json::from_str::<User>(incomplete) {
        Ok(u) => println!("Parsed: {:?}", u),
        Err(e) => println!("Missing field: {}", e),
    }

    println!("\n=== Step 31 Complete! ===");
}
