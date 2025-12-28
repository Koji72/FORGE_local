//! Settings repository implementation

use crate::StorageError;
use rusqlite::{params, Connection};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Settings repository for key-value configuration
pub struct SettingsRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SettingsRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Get a setting value
    pub async fn get(&self, key: &str) -> Result<Option<Value>, StorageError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT value_json FROM settings WHERE key = ?1")?;

        let result = stmt.query_row(params![key], |row| {
            let json_str: String = row.get(0)?;
            Ok(json_str)
        });

        match result {
            Ok(json_str) => {
                let value: Value = serde_json::from_str(&json_str)?;
                Ok(Some(value))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Get a setting as a specific type
    pub async fn get_typed<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<Option<T>, StorageError> {
        if let Some(value) = self.get(key).await? {
            let typed: T = serde_json::from_value(value)?;
            Ok(Some(typed))
        } else {
            Ok(None)
        }
    }

    /// Set a setting value
    pub async fn set(&self, key: &str, value: &Value) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        let now = chrono::Utc::now().timestamp_millis();
        let json_str = serde_json::to_string(value)?;

        conn.execute(
            r#"
            INSERT INTO settings (key, value_json, updated_at)
            VALUES (?1, ?2, ?3)
            ON CONFLICT(key) DO UPDATE SET
                value_json = excluded.value_json,
                updated_at = excluded.updated_at
            "#,
            params![key, json_str, now],
        )?;
        Ok(())
    }

    /// Set a typed setting value
    pub async fn set_typed<T: serde::Serialize>(&self, key: &str, value: &T) -> Result<(), StorageError> {
        let json_value = serde_json::to_value(value)?;
        self.set(key, &json_value).await
    }

    /// Delete a setting
    pub async fn delete(&self, key: &str) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM settings WHERE key = ?1", params![key])?;
        Ok(())
    }

    /// Get all settings
    pub async fn get_all(&self) -> Result<HashMap<String, Value>, StorageError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT key, value_json FROM settings")?;

        let rows = stmt.query_map([], |row| {
            let key: String = row.get(0)?;
            let value_json: String = row.get(1)?;
            Ok((key, value_json))
        })?;

        let mut settings = HashMap::new();
        for row in rows {
            let (key, json_str) = row?;
            if let Ok(value) = serde_json::from_str(&json_str) {
                settings.insert(key, value);
            }
        }
        Ok(settings)
    }

    /// Get settings by prefix
    pub async fn get_by_prefix(&self, prefix: &str) -> Result<HashMap<String, Value>, StorageError> {
        let conn = self.conn.lock().await;
        let pattern = format!("{}%", prefix);
        let mut stmt = conn.prepare("SELECT key, value_json FROM settings WHERE key LIKE ?1")?;

        let rows = stmt.query_map(params![pattern], |row| {
            let key: String = row.get(0)?;
            let value_json: String = row.get(1)?;
            Ok((key, value_json))
        })?;

        let mut settings = HashMap::new();
        for row in rows {
            let (key, json_str) = row?;
            if let Ok(value) = serde_json::from_str(&json_str) {
                settings.insert(key, value);
            }
        }
        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use serde_json::json;

    async fn setup_test_db() -> Arc<Mutex<Connection>> {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(include_str!("../../../../migrations/0001_init.sql")).unwrap();
        Arc::new(Mutex::new(conn))
    }

    #[tokio::test]
    async fn test_set_and_get() {
        let conn = setup_test_db().await;
        let repo = SettingsRepository::new(conn);

        repo.set("test.key", &json!("test_value")).await.unwrap();

        let value = repo.get("test.key").await.unwrap().unwrap();
        assert_eq!(value, json!("test_value"));
    }

    #[tokio::test]
    async fn test_get_typed() {
        let conn = setup_test_db().await;
        let repo = SettingsRepository::new(conn);

        repo.set("test.number", &json!(42)).await.unwrap();

        let value: i32 = repo.get_typed("test.number").await.unwrap().unwrap();
        assert_eq!(value, 42);
    }
}
