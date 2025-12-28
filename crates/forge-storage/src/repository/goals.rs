//! Goal repository implementation

use crate::StorageError;
use forge_domain::{EntityId, Goal, GoalHorizon, GoalStatus, TimestampMs};
use rusqlite::{params, Connection, Row};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Goal filter for queries
#[derive(Debug, Default, Clone)]
pub struct GoalFilter {
    pub status: Option<Vec<GoalStatus>>,
    pub horizon: Option<Vec<GoalHorizon>>,
    pub parent_goal_id: Option<EntityId>,
    pub include_deleted: bool,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Goal repository for CRUD operations
pub struct GoalRepository {
    conn: Arc<Mutex<Connection>>,
}

impl GoalRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Create a new goal
    pub async fn create(&self, goal: &Goal) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        conn.execute(
            r#"
            INSERT INTO goals (
                id, title, description_md, status, horizon, target_date,
                progress_percent, parent_goal_id, created_at, updated_at, deleted_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#,
            params![
                goal.id.to_string(),
                goal.title,
                goal.description_md,
                status_to_str(goal.status),
                horizon_to_str(goal.horizon),
                goal.target_date,
                goal.progress_percent,
                goal.parent_goal_id.map(|id| id.to_string()),
                goal.created_at,
                goal.updated_at,
                goal.deleted_at,
            ],
        )?;
        Ok(())
    }

    /// Get a goal by ID
    pub async fn get(&self, id: EntityId) -> Result<Option<Goal>, StorageError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT * FROM goals WHERE id = ?1 AND deleted_at IS NULL"
        )?;

        let result = stmt.query_row(params![id.to_string()], |row| {
            Ok(row_to_goal(row))
        });

        match result {
            Ok(goal) => Ok(Some(goal?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Update an existing goal
    pub async fn update(&self, goal: &Goal) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        let rows = conn.execute(
            r#"
            UPDATE goals SET
                title = ?2,
                description_md = ?3,
                status = ?4,
                horizon = ?5,
                target_date = ?6,
                progress_percent = ?7,
                parent_goal_id = ?8,
                updated_at = ?9,
                deleted_at = ?10
            WHERE id = ?1
            "#,
            params![
                goal.id.to_string(),
                goal.title,
                goal.description_md,
                status_to_str(goal.status),
                horizon_to_str(goal.horizon),
                goal.target_date,
                goal.progress_percent,
                goal.parent_goal_id.map(|id| id.to_string()),
                goal.updated_at,
                goal.deleted_at,
            ],
        )?;

        if rows == 0 {
            return Err(StorageError::NotFound {
                entity_type: "Goal".to_string(),
                id: goal.id.to_string(),
            });
        }
        Ok(())
    }

    /// Soft delete a goal
    pub async fn delete(&self, id: EntityId, deleted_at: TimestampMs) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        let rows = conn.execute(
            "UPDATE goals SET deleted_at = ?2, updated_at = ?2 WHERE id = ?1 AND deleted_at IS NULL",
            params![id.to_string(), deleted_at],
        )?;

        if rows == 0 {
            return Err(StorageError::NotFound {
                entity_type: "Goal".to_string(),
                id: id.to_string(),
            });
        }
        Ok(())
    }

    /// List goals with optional filters
    pub async fn list(&self, filter: GoalFilter) -> Result<Vec<Goal>, StorageError> {
        let conn = self.conn.lock().await;

        let mut sql = String::from("SELECT * FROM goals WHERE 1=1");

        if !filter.include_deleted {
            sql.push_str(" AND deleted_at IS NULL");
        }

        if let Some(ref statuses) = filter.status {
            if !statuses.is_empty() {
                let status_strs: Vec<String> = statuses.iter()
                    .map(|s| format!("'{}'", status_to_str(*s)))
                    .collect();
                sql.push_str(&format!(" AND status IN ({})", status_strs.join(",")));
            }
        }

        if let Some(ref horizons) = filter.horizon {
            if !horizons.is_empty() {
                let horizon_strs: Vec<String> = horizons.iter()
                    .map(|h| format!("'{}'", horizon_to_str(*h)))
                    .collect();
                sql.push_str(&format!(" AND horizon IN ({})", horizon_strs.join(",")));
            }
        }

        sql.push_str(" ORDER BY created_at DESC");

        if let Some(limit) = filter.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = filter.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map([], |row| Ok(row_to_goal(row)))?;

        let mut goals = Vec::new();
        for row in rows {
            goals.push(row??);
        }
        Ok(goals)
    }

    /// Count goals matching filter
    pub async fn count(&self, filter: GoalFilter) -> Result<u64, StorageError> {
        let conn = self.conn.lock().await;

        let mut sql = String::from("SELECT COUNT(*) FROM goals WHERE 1=1");

        if !filter.include_deleted {
            sql.push_str(" AND deleted_at IS NULL");
        }

        let count: i64 = conn.query_row(&sql, [], |row| row.get(0))?;
        Ok(count as u64)
    }
}

fn status_to_str(status: GoalStatus) -> &'static str {
    match status {
        GoalStatus::Active => "active",
        GoalStatus::Completed => "completed",
        GoalStatus::Paused => "paused",
        GoalStatus::Archived => "archived",
    }
}

fn str_to_status(s: &str) -> GoalStatus {
    match s {
        "active" => GoalStatus::Active,
        "completed" => GoalStatus::Completed,
        "paused" => GoalStatus::Paused,
        "archived" => GoalStatus::Archived,
        _ => GoalStatus::Active,
    }
}

fn horizon_to_str(horizon: GoalHorizon) -> &'static str {
    match horizon {
        GoalHorizon::Short => "short",
        GoalHorizon::Medium => "medium",
        GoalHorizon::Long => "long",
        GoalHorizon::Vision => "vision",
    }
}

fn str_to_horizon(s: &str) -> GoalHorizon {
    match s {
        "short" => GoalHorizon::Short,
        "medium" => GoalHorizon::Medium,
        "long" => GoalHorizon::Long,
        "vision" => GoalHorizon::Vision,
        _ => GoalHorizon::Medium,
    }
}

fn row_to_goal(row: &Row) -> Result<Goal, StorageError> {
    let id_str: String = row.get("id")?;
    let status_str: String = row.get("status")?;
    let horizon_str: String = row.get("horizon")?;
    let parent_id_str: Option<String> = row.get("parent_goal_id")?;

    Ok(Goal {
        id: id_str.parse().map_err(|_| StorageError::Validation("Invalid UUID".to_string()))?,
        title: row.get("title")?,
        description_md: row.get("description_md")?,
        status: str_to_status(&status_str),
        horizon: str_to_horizon(&horizon_str),
        target_date: row.get("target_date")?,
        progress_percent: row.get("progress_percent")?,
        parent_goal_id: parent_id_str.and_then(|s| s.parse().ok()),
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
        deleted_at: row.get("deleted_at")?,
    })
}
