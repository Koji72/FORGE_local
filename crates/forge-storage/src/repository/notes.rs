//! Note repository implementation

use crate::StorageError;
use forge_domain::{EntityId, Note, NoteType, TimestampMs};
use rusqlite::{params, Connection, Row};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Note filter for queries
#[derive(Debug, Default, Clone)]
pub struct NoteFilter {
    pub note_type: Option<Vec<NoteType>>,
    pub tags: Option<Vec<String>>,
    pub pinned_only: bool,
    pub include_deleted: bool,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Note repository for CRUD operations
pub struct NoteRepository {
    conn: Arc<Mutex<Connection>>,
}

impl NoteRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Create a new note
    pub async fn create(&self, note: &Note) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        conn.execute(
            r#"
            INSERT INTO notes (
                id, title, content_md, note_type, tags, pinned,
                created_at, updated_at, deleted_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
            params![
                note.id.to_string(),
                note.title,
                note.content_md,
                note_type_to_str(note.note_type),
                serde_json::to_string(&note.tags).ok(),
                note.pinned as i32,
                note.created_at,
                note.updated_at,
                note.deleted_at,
            ],
        )?;
        Ok(())
    }

    /// Get a note by ID
    pub async fn get(&self, id: EntityId) -> Result<Option<Note>, StorageError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT * FROM notes WHERE id = ?1 AND deleted_at IS NULL"
        )?;

        let result = stmt.query_row(params![id.to_string()], |row| {
            Ok(row_to_note(row))
        });

        match result {
            Ok(note) => Ok(Some(note?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Update an existing note
    pub async fn update(&self, note: &Note) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        let rows = conn.execute(
            r#"
            UPDATE notes SET
                title = ?2,
                content_md = ?3,
                note_type = ?4,
                tags = ?5,
                pinned = ?6,
                updated_at = ?7,
                deleted_at = ?8
            WHERE id = ?1
            "#,
            params![
                note.id.to_string(),
                note.title,
                note.content_md,
                note_type_to_str(note.note_type),
                serde_json::to_string(&note.tags).ok(),
                note.pinned as i32,
                note.updated_at,
                note.deleted_at,
            ],
        )?;

        if rows == 0 {
            return Err(StorageError::NotFound {
                entity_type: "Note".to_string(),
                id: note.id.to_string(),
            });
        }
        Ok(())
    }

    /// Soft delete a note
    pub async fn delete(&self, id: EntityId, deleted_at: TimestampMs) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        let rows = conn.execute(
            "UPDATE notes SET deleted_at = ?2, updated_at = ?2 WHERE id = ?1 AND deleted_at IS NULL",
            params![id.to_string(), deleted_at],
        )?;

        if rows == 0 {
            return Err(StorageError::NotFound {
                entity_type: "Note".to_string(),
                id: id.to_string(),
            });
        }
        Ok(())
    }

    /// List notes with optional filters
    pub async fn list(&self, filter: NoteFilter) -> Result<Vec<Note>, StorageError> {
        let conn = self.conn.lock().await;

        let mut sql = String::from("SELECT * FROM notes WHERE 1=1");

        if !filter.include_deleted {
            sql.push_str(" AND deleted_at IS NULL");
        }

        if filter.pinned_only {
            sql.push_str(" AND pinned = 1");
        }

        if let Some(ref types) = filter.note_type {
            if !types.is_empty() {
                let type_strs: Vec<String> = types.iter()
                    .map(|t| format!("'{}'", note_type_to_str(*t)))
                    .collect();
                sql.push_str(&format!(" AND note_type IN ({})", type_strs.join(",")));
            }
        }

        sql.push_str(" ORDER BY pinned DESC, updated_at DESC");

        if let Some(limit) = filter.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = filter.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map([], |row| Ok(row_to_note(row)))?;

        let mut notes = Vec::new();
        for row in rows {
            notes.push(row??);
        }
        Ok(notes)
    }

    /// Count notes matching filter
    pub async fn count(&self, filter: NoteFilter) -> Result<u64, StorageError> {
        let conn = self.conn.lock().await;

        let mut sql = String::from("SELECT COUNT(*) FROM notes WHERE 1=1");

        if !filter.include_deleted {
            sql.push_str(" AND deleted_at IS NULL");
        }

        if filter.pinned_only {
            sql.push_str(" AND pinned = 1");
        }

        let count: i64 = conn.query_row(&sql, [], |row| row.get(0))?;
        Ok(count as u64)
    }
}

fn note_type_to_str(nt: NoteType) -> &'static str {
    match nt {
        NoteType::Note => "note",
        NoteType::Meeting => "meeting",
        NoteType::Journal => "journal",
        NoteType::Reference => "reference",
    }
}

fn str_to_note_type(s: &str) -> NoteType {
    match s {
        "note" => NoteType::Note,
        "meeting" => NoteType::Meeting,
        "journal" => NoteType::Journal,
        "reference" => NoteType::Reference,
        _ => NoteType::Note,
    }
}

fn row_to_note(row: &Row) -> Result<Note, StorageError> {
    let id_str: String = row.get("id")?;
    let type_str: String = row.get("note_type")?;
    let tags_json: Option<String> = row.get("tags")?;
    let pinned: i32 = row.get("pinned")?;

    Ok(Note {
        id: id_str.parse().map_err(|_| StorageError::Validation("Invalid UUID".to_string()))?,
        title: row.get("title")?,
        content_md: row.get("content_md")?,
        note_type: str_to_note_type(&type_str),
        tags: tags_json
            .and_then(|j| serde_json::from_str(&j).ok())
            .unwrap_or_default(),
        pinned: pinned != 0,
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
        deleted_at: row.get("deleted_at")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    async fn setup_test_db() -> Arc<Mutex<Connection>> {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(include_str!("../../../../migrations/0001_init.sql")).unwrap();
        Arc::new(Mutex::new(conn))
    }

    #[tokio::test]
    async fn test_create_and_get_note() {
        let conn = setup_test_db().await;
        let repo = NoteRepository::new(conn);

        let note = Note::new("Test Note", "# Content");
        repo.create(&note).await.unwrap();

        let fetched = repo.get(note.id).await.unwrap().unwrap();
        assert_eq!(fetched.title, "Test Note");
        assert_eq!(fetched.content_md, "# Content");
    }
}
