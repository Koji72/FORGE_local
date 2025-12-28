//! Database migrations

use crate::StorageError;
use rusqlite::Connection;
use tracing::info;

/// Run all pending migrations
pub fn run_migrations(conn: &Connection) -> Result<(), StorageError> {
    // Check current version
    let current_version = get_schema_version(conn)?;
    info!("Current schema version: {}", current_version);

    // Apply migrations in order
    if current_version < 1 {
        apply_migration_001(conn)?;
    }

    Ok(())
}

fn get_schema_version(conn: &Connection) -> Result<i32, StorageError> {
    // Try to get version from schema_version table
    let result: Result<i32, _> = conn.query_row(
        "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1",
        [],
        |row| row.get(0),
    );

    match result {
        Ok(version) => Ok(version),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0),
        Err(rusqlite::Error::SqliteFailure(_, Some(ref msg)))
            if msg.contains("no such table") =>
        {
            Ok(0)
        }
        Err(e) => Err(e.into()),
    }
}

fn apply_migration_001(conn: &Connection) -> Result<(), StorageError> {
    info!("Applying migration 001: initial schema");

    // Load and execute the migration SQL
    // In production, this would read from migrations/0001_init.sql
    // For now, we include the essential schema inline

    conn.execute_batch(include_str!("../../../migrations/0001_init.sql"))
        .map_err(|e| StorageError::Migration(format!("Failed to apply migration 001: {}", e)))?;

    info!("Migration 001 applied successfully");
    Ok(())
}
