// ============================================================
// STEP 20: Type System Patterns
// ============================================================
// Run: rustc main.rs && ./main
//
// Powerful design patterns leveraging Rust's type system:
// - Newtype pattern (type safety at zero cost)
// - Typestate pattern (compile-time state machines)
// - Builder pattern
// - Phantom types
// - Sealed traits
// ============================================================

use std::fmt;
use std::marker::PhantomData;

fn main() {
    // =========================
    // 20.1 Newtype for Type Safety
    // =========================

    println!("--- Newtype Pattern ---");

    // Without newtypes, easy to swap arguments:
    //   fn connect(host: &str, port: u16, timeout: u16) { ... }
    //   connect("localhost", 3000, 8080); // oops! swapped port and timeout

    // With newtypes, the compiler catches mistakes:
    let user = UserId(42);
    let order = OrderId(100);
    // process_order(user, order); // ERROR: expected OrderId, got UserId
    process_order(order, user);

    // Newtypes for validated data:
    match EmailAddr::new("alice@example.com") {
        Some(email) => println!("Valid: {}", email),
        None => println!("Invalid email"),
    }
    match EmailAddr::new("not-an-email") {
        Some(email) => println!("Valid: {}", email),
        None => println!("Invalid: not-an-email"),
    }

    // =========================
    // 20.2 Builder Pattern
    // =========================

    println!("\n--- Builder Pattern ---");

    let server = ServerConfig::builder()
        .host("0.0.0.0")
        .port(8080)
        .max_connections(1000)
        .tls(true)
        .build();
    println!("{:?}", server);

    // Minimal config with defaults:
    let minimal = ServerConfig::builder()
        .host("localhost")
        .build();
    println!("{:?}", minimal);

    // =========================
    // 20.3 Typestate Pattern
    // =========================

    println!("\n--- Typestate Pattern ---");

    // The type system enforces valid state transitions.
    // You literally CANNOT call methods in the wrong order.

    let rocket = Rocket::new("Falcon 9");
    println!("{}", rocket);

    // Must fuel before launching:
    let rocket = rocket.fuel(100);
    println!("{}", rocket);

    // Must check before launching:
    let rocket = rocket.check();
    println!("{}", rocket);

    // NOW we can launch:
    let rocket = rocket.launch();
    println!("{}", rocket);

    // These would NOT compile:
    // Rocket::new("Bad").launch();       // can't launch from Building
    // Rocket::new("Bad").fuel(50).launch(); // can't launch from Fueled

    // =========================
    // 20.4 Phantom Types
    // =========================

    println!("\n--- Phantom Types ---");

    // PhantomData<T> marks that a type "uses" T without storing it.
    // Great for units, permissions, or format markers.

    let meters = Length::<Metric>::new(100.0);
    let more_meters = Length::<Metric>::new(50.0);
    let total = meters.add(&more_meters);
    println!("{:.0}m + {:.0}m = {:.0}m", meters.value, more_meters.value, total.value);

    let feet = Length::<Imperial>::new(328.0);
    // This would NOT compile — can't add meters and feet:
    // let bad = meters.add(&feet);
    println!("Feet: {:.0}", feet.value);

    // =========================
    // 20.5 Enum-Based State Machine
    // =========================

    println!("\n--- State Machine ---");

    let mut conn = Connection::new();
    println!("State: {}", conn);

    conn = conn.connect();
    println!("State: {}", conn);

    conn = conn.authenticate("admin", "secret");
    println!("State: {}", conn);

    let data = conn.query("SELECT * FROM users");
    println!("Query result: {}", data);

    conn = conn.disconnect();
    println!("State: {}", conn);

    // =========================
    // 20.6 Sealed Trait Pattern
    // =========================

    println!("\n--- Sealed Trait ---");

    // A sealed trait can only be implemented within your crate.
    // External users can USE it but not IMPLEMENT it.
    println!("Allowed types for Protocol:");
    println!("  Http status: {}", Http.status());
    println!("  Grpc status: {}", Grpc.status());
    // Others can't implement Protocol because `Sealed` is private

    // =========================
    // 20.7 Type-Level Integers (Const Generics)
    // =========================

    println!("\n--- Const Generics ---");

    let v3 = FixedVec::<i32, 3>::from_array([1, 2, 3]);
    let v5 = FixedVec::<i32, 5>::from_array([10, 20, 30, 40, 50]);
    println!("3-vec: {:?}", v3.as_slice());
    println!("5-vec: {:?}", v5.as_slice());

    let dot = v3.dot(&FixedVec::from_array([4, 5, 6]));
    println!("Dot product [1,2,3]·[4,5,6] = {}", dot);

    // =========================
    // 20.8 Zero-Sized Types (ZSTs)
    // =========================

    println!("\n--- Zero-Sized Types ---");

    // ZSTs take no memory. Useful as markers.
    let read_token = Token::<ReadOnly>::new();
    let write_token = Token::<ReadWrite>::new();

    read_data(&read_token);
    read_data(&write_token); // ReadWrite can also read
    write_data(&write_token);
    // write_data(&read_token); // ERROR: ReadOnly can't write

    println!("\n--- Step 20 Complete! ---");
}

// ============================================================
// Implementations
// ============================================================

// --- 20.1 Newtype ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UserId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct OrderId(u64);

fn process_order(order: OrderId, user: UserId) {
    println!("Processing order {:?} for user {:?}", order, user);
}

struct EmailAddr(String);

impl EmailAddr {
    fn new(s: &str) -> Option<Self> {
        if s.contains('@') && s.contains('.') {
            Some(EmailAddr(s.to_string()))
        } else {
            None
        }
    }
}

impl fmt::Display for EmailAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// --- 20.2 Builder ---

#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    max_connections: u32,
    tls: bool,
}

struct ServerConfigBuilder {
    host: String,
    port: u16,
    max_connections: u32,
    tls: bool,
}

impl ServerConfig {
    fn builder() -> ServerConfigBuilder {
        ServerConfigBuilder {
            host: String::from("127.0.0.1"),
            port: 3000,
            max_connections: 100,
            tls: false,
        }
    }
}

impl ServerConfigBuilder {
    fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }
    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }
    fn tls(mut self, enabled: bool) -> Self {
        self.tls = enabled;
        self
    }
    fn build(self) -> ServerConfig {
        ServerConfig {
            host: self.host,
            port: self.port,
            max_connections: self.max_connections,
            tls: self.tls,
        }
    }
}

// --- 20.3 Typestate ---

// Each state is a separate zero-sized type
struct Building;
struct Fueled;
struct Checked;
struct Launched;

struct Rocket<State> {
    name: String,
    fuel: u32,
    _state: PhantomData<State>,
}

impl Rocket<Building> {
    fn new(name: &str) -> Rocket<Building> {
        Rocket {
            name: name.to_string(),
            fuel: 0,
            _state: PhantomData,
        }
    }

    fn fuel(self, amount: u32) -> Rocket<Fueled> {
        Rocket {
            name: self.name,
            fuel: amount,
            _state: PhantomData,
        }
    }
}

impl Rocket<Fueled> {
    fn check(self) -> Rocket<Checked> {
        Rocket {
            name: self.name,
            fuel: self.fuel,
            _state: PhantomData,
        }
    }
}

impl Rocket<Checked> {
    fn launch(self) -> Rocket<Launched> {
        Rocket {
            name: self.name,
            fuel: self.fuel,
            _state: PhantomData,
        }
    }
}

impl<State> fmt::Display for Rocket<State> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = std::any::type_name::<State>();
        let short = state.rsplit("::").next().unwrap_or(state);
        write!(f, "Rocket '{}' [{}] fuel={}", self.name, short, self.fuel)
    }
}

// --- 20.4 Phantom Types ---

struct Metric;
struct Imperial;

struct Length<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl<Unit> Length<Unit> {
    fn new(value: f64) -> Self {
        Length {
            value,
            _unit: PhantomData,
        }
    }

    // Can only add same units:
    fn add(&self, other: &Length<Unit>) -> Length<Unit> {
        Length::new(self.value + other.value)
    }
}

// --- 20.5 Enum State Machine ---

enum Connection {
    Disconnected,
    Connected { host: String },
    Authenticated { host: String, user: String },
}

impl Connection {
    fn new() -> Self {
        Connection::Disconnected
    }

    fn connect(self) -> Self {
        match self {
            Connection::Disconnected => {
                Connection::Connected {
                    host: "db.example.com".into(),
                }
            }
            other => {
                println!("  Already connected!");
                other
            }
        }
    }

    fn authenticate(self, user: &str, _pass: &str) -> Self {
        match self {
            Connection::Connected { host } => Connection::Authenticated {
                host,
                user: user.into(),
            },
            other => {
                println!("  Must connect first!");
                other
            }
        }
    }

    fn query(&self, sql: &str) -> String {
        match self {
            Connection::Authenticated { user, .. } => {
                format!("{} ran: {}", user, sql)
            }
            _ => "ERROR: not authenticated".into(),
        }
    }

    fn disconnect(self) -> Self {
        Connection::Disconnected
    }
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Connection::Disconnected => write!(f, "Disconnected"),
            Connection::Connected { host } => write!(f, "Connected({})", host),
            Connection::Authenticated { host, user } => {
                write!(f, "Authenticated({}@{})", user, host)
            }
        }
    }
}

// --- 20.6 Sealed Trait ---

mod private {
    pub trait Sealed {}
}

trait Protocol: private::Sealed {
    fn status(&self) -> &str;
}

struct Http;
struct Grpc;

impl private::Sealed for Http {}
impl private::Sealed for Grpc {}

impl Protocol for Http {
    fn status(&self) -> &str {
        "HTTP/1.1 200 OK"
    }
}
impl Protocol for Grpc {
    fn status(&self) -> &str {
        "gRPC OK"
    }
}

// --- 20.7 Const Generics ---

#[derive(Debug)]
struct FixedVec<T, const N: usize> {
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> FixedVec<T, N> {
    fn from_array(arr: [T; N]) -> Self {
        FixedVec { data: arr }
    }

    fn as_slice(&self) -> &[T] {
        &self.data
    }
}

impl<const N: usize> FixedVec<i32, N> {
    fn dot(&self, other: &FixedVec<i32, N>) -> i32 {
        self.data.iter().zip(other.data.iter()).map(|(a, b)| a * b).sum()
    }
}

// --- 20.8 ZST Permissions ---

struct ReadOnly;
struct ReadWrite;

trait Readable {}
trait Writable {}

impl Readable for ReadOnly {}
impl Readable for ReadWrite {}
impl Writable for ReadWrite {}

struct Token<Permission> {
    _perm: PhantomData<Permission>,
}

impl<P> Token<P> {
    fn new() -> Self {
        Token { _perm: PhantomData }
    }
}

fn read_data<P: Readable>(_token: &Token<P>) {
    println!("  Reading data...");
}

fn write_data<P: Writable>(_token: &Token<P>) {
    println!("  Writing data...");
}
