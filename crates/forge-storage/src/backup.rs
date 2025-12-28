//! Backup and restore functionality

use crate::StorageError;
use rusqlite::Connection;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

/// Backup configuration
#[derive(Debug, Clone)]
pub struct BackupConfig {
    pub backup_dir: PathBuf,
    pub keep_count: usize,
    pub prefix: String,
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            backup_dir: PathBuf::from("backups"),
            keep_count: 5,
            prefix: "forge-backup".to_string(),
        }
    }
}

/// Backup metadata
#[derive(Debug, Clone)]
pub struct BackupInfo {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub checksum: String,
    pub created_at: i64,
}

/// Create a backup of the database
pub fn create_backup(
    conn: &Connection,
    config: &BackupConfig,
) -> Result<BackupInfo, StorageError> {
    // Ensure backup directory exists
    fs::create_dir_all(&config.backup_dir)?;

    // Generate backup filename with timestamp
    let timestamp = chrono::Utc::now().format("%Y%m%d-%H%M%S");
    let filename = format!("{}_{}.db", config.prefix, timestamp);
    let backup_path = config.backup_dir.join(&filename);

    // Use SQLite backup API
    let mut dst = Connection::open(&backup_path)?;
    let backup = rusqlite::backup::Backup::new(conn, &mut dst)?;
    backup.run_to_completion(100, std::time::Duration::from_millis(10), None)?;
    drop(dst);

    // Calculate checksum
    let data = fs::read(&backup_path)?;
    let checksum = format!("{:x}", Sha256::digest(&data));
    let size_bytes = data.len() as u64;

    info!("Backup created: {:?} ({} bytes)", backup_path, size_bytes);

    Ok(BackupInfo {
        path: backup_path,
        size_bytes,
        checksum,
        created_at: chrono::Utc::now().timestamp_millis(),
    })
}

/// Rotate backups, keeping only the most recent ones
pub fn rotate_backups(config: &BackupConfig) -> Result<(usize, usize), StorageError> {
    let mut backups: Vec<_> = fs::read_dir(&config.backup_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with(&config.prefix) && n.ends_with(".db"))
                .unwrap_or(false)
        })
        .collect();

    // Sort by name (which includes timestamp, so newest last)
    backups.sort_by_key(|e| e.path());
    backups.reverse();

    let kept = backups.len().min(config.keep_count);
    let mut deleted = 0;

    for entry in backups.into_iter().skip(config.keep_count) {
        match fs::remove_file(entry.path()) {
            Ok(_) => {
                deleted += 1;
                info!("Deleted old backup: {:?}", entry.path());
            }
            Err(e) => {
                warn!("Failed to delete backup {:?}: {}", entry.path(), e);
            }
        }
    }

    Ok((kept, deleted))
}

/// List available backups
pub fn list_backups(config: &BackupConfig) -> Result<Vec<BackupInfo>, StorageError> {
    if !config.backup_dir.exists() {
        return Ok(Vec::new());
    }

    let mut backups: Vec<BackupInfo> = fs::read_dir(&config.backup_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with(&config.prefix) && n.ends_with(".db"))
                .unwrap_or(false)
        })
        .filter_map(|e| {
            let path = e.path();
            let metadata = fs::metadata(&path).ok()?;
            Some(BackupInfo {
                path,
                size_bytes: metadata.len(),
                checksum: String::new(), // Skip checksum for listing
                created_at: metadata
                    .created()
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| d.as_millis() as i64)
                    .unwrap_or(0),
            })
        })
        .collect();

    backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(backups)
}

/// Restore a database from backup
pub fn restore_backup(
    backup_path: &Path,
    target_path: &Path,
) -> Result<(), StorageError> {
    // Verify backup exists
    if !backup_path.exists() {
        return Err(StorageError::Validation(format!(
            "Backup not found: {:?}",
            backup_path
        )));
    }

    // Copy backup to target
    fs::copy(backup_path, target_path)?;

    info!("Restored backup from {:?} to {:?}", backup_path, target_path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_and_list_backup() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let backup_dir = temp_dir.path().join("backups");

        // Create a test database
        let conn = Connection::open(&db_path).unwrap();
        conn.execute("CREATE TABLE test (id INTEGER)", []).unwrap();

        let config = BackupConfig {
            backup_dir: backup_dir.clone(),
            keep_count: 3,
            prefix: "test-backup".to_string(),
        };

        // Create backup
        let backup = create_backup(&conn, &config).unwrap();
        assert!(backup.path.exists());
        assert!(backup.size_bytes > 0);
        assert!(!backup.checksum.is_empty());

        // List backups
        let backups = list_backups(&config).unwrap();
        assert_eq!(backups.len(), 1);
    }
}
