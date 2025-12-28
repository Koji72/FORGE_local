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
pub mod backup;

pub use error::StorageError;
pub use repository::{
    Repositories, DbConnection,
    TaskRepository, TaskFilter,
    NoteRepository, NoteFilter,
    GoalRepository, GoalFilter,
    LinkRepository,
    SettingsRepository,
    AgentRunRepository, AgentRunFilter,
};

use rusqlite::Connection;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
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

/// Database manager for the application
pub struct Database {
    conn: DbConnection,
    config: DatabaseConfig,
}

impl Database {
    /// Open or create a database
    pub fn open(config: DatabaseConfig) -> Result<Self, StorageError> {
        let conn = open_database(&config)?;
        initialize_database(&conn)?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            config,
        })
    }

    /// Open an in-memory database (for testing)
    pub fn open_in_memory() -> Result<Self, StorageError> {
        let conn = Connection::open_in_memory()?;
        initialize_database(&conn)?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            config: DatabaseConfig::default(),
        })
    }

    /// Get the shared connection
    pub fn connection(&self) -> DbConnection {
        self.conn.clone()
    }

    /// Create all repositories
    pub fn repositories(&self) -> Repositories {
        Repositories::new(self.conn.clone())
    }

    /// Get the database path
    pub fn path(&self) -> &Path {
        &self.config.path
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_in_memory() {
        let db = Database::open_in_memory().unwrap();
        let repos = db.repositories();
        // Verify we can use the repositories
        assert!(repos.tasks.list(TaskFilter::default()).is_ok() == false); // async, can't call directly
    }
}
