//! # Forge Storage
//!
//! SQLite storage layer with SQLCipher encryption for Forge.
//!
//! This crate provides:
//! - Database connection management
//! - Schema migrations
//! - CRUD operations for all entities
//! - Backup and restore functionality

pub mod error;
pub mod migrations;
pub mod repository;

pub use error::StorageError;

use rusqlite::Connection;
use std::path::Path;
use tracing::info;

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub path: std::path::PathBuf,
    pub passphrase: Option<String>,
    pub wal_mode: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            path: std::path::PathBuf::from("forge.db"),
            passphrase: None,
            wal_mode: true,
        }
    }
}

/// Open a database connection with optional encryption
pub fn open_database(config: &DatabaseConfig) -> Result<Connection, StorageError> {
    let conn = Connection::open(&config.path)?;

    // Set SQLCipher key if provided
    if let Some(ref passphrase) = config.passphrase {
        conn.pragma_update(None, "key", passphrase)?;
    }

    // Enable WAL mode for better concurrency
    if config.wal_mode {
        conn.pragma_update(None, "journal_mode", "WAL")?;
    }

    // Enable foreign keys
    conn.pragma_update(None, "foreign_keys", "ON")?;

    info!("Database opened: {:?}", config.path);

    Ok(conn)
}

/// Initialize a new database with schema
pub fn initialize_database(conn: &Connection) -> Result<(), StorageError> {
    migrations::run_migrations(conn)?;
    info!("Database initialized with schema");
    Ok(())
}
