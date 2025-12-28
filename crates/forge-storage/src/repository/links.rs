//! Link repository implementation

use crate::StorageError;
use forge_domain::{EntityId, EntityType, Link, TimestampMs};
use rusqlite::{params, Connection, Row};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Link repository for managing entity relationships
pub struct LinkRepository {
    conn: Arc<Mutex<Connection>>,
}

impl LinkRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Create a new link
    pub async fn create(&self, link: &Link) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        conn.execute(
            r#"
            INSERT INTO links (id, from_type, from_id, to_type, to_id, label, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            params![
                link.id.to_string(),
                entity_type_to_str(link.from_type),
                link.from_id.to_string(),
                entity_type_to_str(link.to_type),
                link.to_id.to_string(),
                link.label,
                link.created_at,
            ],
        )?;
        Ok(())
    }

    /// Get a link by ID
    pub async fn get(&self, id: EntityId) -> Result<Option<Link>, StorageError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT * FROM links WHERE id = ?1")?;

        let result = stmt.query_row(params![id.to_string()], |row| {
            Ok(row_to_link(row))
        });

        match result {
            Ok(link) => Ok(Some(link?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Delete a link
    pub async fn delete(&self, id: EntityId) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM links WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }

    /// List links from an entity
    pub async fn list_from(&self, entity_type: EntityType, entity_id: EntityId) -> Result<Vec<Link>, StorageError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT * FROM links WHERE from_type = ?1 AND from_id = ?2"
        )?;

        let rows = stmt.query_map(
            params![entity_type_to_str(entity_type), entity_id.to_string()],
            |row| Ok(row_to_link(row)),
        )?;

        let mut links = Vec::new();
        for row in rows {
            links.push(row??);
        }
        Ok(links)
    }

    /// List links to an entity
    pub async fn list_to(&self, entity_type: EntityType, entity_id: EntityId) -> Result<Vec<Link>, StorageError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT * FROM links WHERE to_type = ?1 AND to_id = ?2"
        )?;

        let rows = stmt.query_map(
            params![entity_type_to_str(entity_type), entity_id.to_string()],
            |row| Ok(row_to_link(row)),
        )?;

        let mut links = Vec::new();
        for row in rows {
            links.push(row??);
        }
        Ok(links)
    }

    /// List all links for an entity (both directions)
    pub async fn list_by_entity(&self, entity_type: EntityType, entity_id: EntityId) -> Result<Vec<Link>, StorageError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            r#"
            SELECT * FROM links
            WHERE (from_type = ?1 AND from_id = ?2)
               OR (to_type = ?1 AND to_id = ?2)
            "#
        )?;

        let rows = stmt.query_map(
            params![entity_type_to_str(entity_type), entity_id.to_string()],
            |row| Ok(row_to_link(row)),
        )?;

        let mut links = Vec::new();
        for row in rows {
            links.push(row??);
        }
        Ok(links)
    }

    /// Check if a link exists between two entities
    pub async fn exists(&self, from_type: EntityType, from_id: EntityId, to_type: EntityType, to_id: EntityId) -> Result<bool, StorageError> {
        let conn = self.conn.lock().await;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM links WHERE from_type = ?1 AND from_id = ?2 AND to_type = ?3 AND to_id = ?4",
            params![
                entity_type_to_str(from_type),
                from_id.to_string(),
                entity_type_to_str(to_type),
                to_id.to_string(),
            ],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }
}

fn entity_type_to_str(et: EntityType) -> &'static str {
    match et {
        EntityType::Task => "task",
        EntityType::Note => "note",
        EntityType::Goal => "goal",
    }
}

fn str_to_entity_type(s: &str) -> EntityType {
    match s {
        "task" => EntityType::Task,
        "note" => EntityType::Note,
        "goal" => EntityType::Goal,
        _ => EntityType::Task,
    }
}

fn row_to_link(row: &Row) -> Result<Link, StorageError> {
    let id_str: String = row.get("id")?;
    let from_type_str: String = row.get("from_type")?;
    let from_id_str: String = row.get("from_id")?;
    let to_type_str: String = row.get("to_type")?;
    let to_id_str: String = row.get("to_id")?;

    Ok(Link {
        id: id_str.parse().map_err(|_| StorageError::Validation("Invalid UUID".to_string()))?,
        from_type: str_to_entity_type(&from_type_str),
        from_id: from_id_str.parse().map_err(|_| StorageError::Validation("Invalid UUID".to_string()))?,
        to_type: str_to_entity_type(&to_type_str),
        to_id: to_id_str.parse().map_err(|_| StorageError::Validation("Invalid UUID".to_string()))?,
        label: row.get("label")?,
        created_at: row.get("created_at")?,
    })
}
