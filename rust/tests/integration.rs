/// Integration tests — require a real encrypted SQLite database.
///
/// Run all:  cargo test --test integration
/// Run one:  cargo test --test integration suite::lifecycle
mod common;
mod suite;
