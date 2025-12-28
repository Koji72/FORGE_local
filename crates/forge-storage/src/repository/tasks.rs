//! Task repository implementation

use crate::StorageError;
use forge_domain::{EntityId, Task, TaskStatus, TimestampMs};
use rusqlite::{params, Connection, Row};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Task filter for queries
#[derive(Debug, Default, Clone)]
pub struct TaskFilter {
    pub status: Option<Vec<TaskStatus>>,
    pub goal_id: Option<EntityId>,
    pub parent_task_id: Option<EntityId>,
    pub due_before: Option<TimestampMs>,
    pub due_after: Option<TimestampMs>,
    pub context_tags: Option<Vec<String>>,
    pub include_deleted: bool,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Task repository for CRUD operations
pub struct TaskRepository {
    conn: Arc<Mutex<Connection>>,
}

impl TaskRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Create a new task
    pub async fn create(&self, task: &Task) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        conn.execute(
            r#"
            INSERT INTO tasks (
                id, title, description_md, status, priority, priority_score,
                due_at, scheduled_at, completed_at, parent_task_id, goal_id,
                context_tags, energy_level, time_estimate_min, recurrence_rule,
                created_at, updated_at, deleted_at
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18
            )
            "#,
            params![
                task.id.to_string(),
                task.title,
                task.description_md,
                status_to_str(task.status),
                task.priority,
                task.priority_score,
                task.due_at,
                task.scheduled_at,
                task.completed_at,
                task.parent_task_id.map(|id| id.to_string()),
                task.goal_id.map(|id| id.to_string()),
                serde_json::to_string(&task.context_tags).ok(),
                task.energy_level.map(|e| format!("{:?}", e).to_lowercase()),
                task.time_estimate_min,
                task.recurrence_rule,
                task.created_at,
                task.updated_at,
                task.deleted_at,
            ],
        )?;
        Ok(())
    }

    /// Get a task by ID
    pub async fn get(&self, id: EntityId) -> Result<Option<Task>, StorageError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT * FROM tasks WHERE id = ?1 AND deleted_at IS NULL"
        )?;

        let result = stmt.query_row(params![id.to_string()], |row| {
            Ok(row_to_task(row))
        });

        match result {
            Ok(task) => Ok(Some(task?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Update an existing task
    pub async fn update(&self, task: &Task) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        let rows = conn.execute(
            r#"
            UPDATE tasks SET
                title = ?2,
                description_md = ?3,
                status = ?4,
                priority = ?5,
                priority_score = ?6,
                due_at = ?7,
                scheduled_at = ?8,
                completed_at = ?9,
                parent_task_id = ?10,
                goal_id = ?11,
                context_tags = ?12,
                energy_level = ?13,
                time_estimate_min = ?14,
                recurrence_rule = ?15,
                updated_at = ?16,
                deleted_at = ?17
            WHERE id = ?1
            "#,
            params![
                task.id.to_string(),
                task.title,
                task.description_md,
                status_to_str(task.status),
                task.priority,
                task.priority_score,
                task.due_at,
                task.scheduled_at,
                task.completed_at,
                task.parent_task_id.map(|id| id.to_string()),
                task.goal_id.map(|id| id.to_string()),
                serde_json::to_string(&task.context_tags).ok(),
                task.energy_level.map(|e| format!("{:?}", e).to_lowercase()),
                task.time_estimate_min,
                task.recurrence_rule,
                task.updated_at,
                task.deleted_at,
            ],
        )?;

        if rows == 0 {
            return Err(StorageError::NotFound {
                entity_type: "Task".to_string(),
                id: task.id.to_string(),
            });
        }
        Ok(())
    }

    /// Soft delete a task
    pub async fn delete(&self, id: EntityId, deleted_at: TimestampMs) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        let rows = conn.execute(
            "UPDATE tasks SET deleted_at = ?2, updated_at = ?2 WHERE id = ?1 AND deleted_at IS NULL",
            params![id.to_string(), deleted_at],
        )?;

        if rows == 0 {
            return Err(StorageError::NotFound {
                entity_type: "Task".to_string(),
                id: id.to_string(),
            });
        }
        Ok(())
    }

    /// Hard delete a task (permanent)
    pub async fn hard_delete(&self, id: EntityId) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM tasks WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }

    /// List tasks with optional filters
    pub async fn list(&self, filter: TaskFilter) -> Result<Vec<Task>, StorageError> {
        let conn = self.conn.lock().await;

        let mut sql = String::from("SELECT * FROM tasks WHERE 1=1");
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if !filter.include_deleted {
            sql.push_str(" AND deleted_at IS NULL");
        }

        if let Some(ref statuses) = filter.status {
            if !statuses.is_empty() {
                let placeholders: Vec<String> = statuses.iter().enumerate()
                    .map(|(i, _)| format!("?{}", params_vec.len() + i + 1))
                    .collect();
                sql.push_str(&format!(" AND status IN ({})", placeholders.join(",")));
                for s in statuses {
                    params_vec.push(Box::new(status_to_str(*s).to_string()));
                }
            }
        }

        if let Some(goal_id) = filter.goal_id {
            params_vec.push(Box::new(goal_id.to_string()));
            sql.push_str(&format!(" AND goal_id = ?{}", params_vec.len()));
        }

        if let Some(due_before) = filter.due_before {
            params_vec.push(Box::new(due_before));
            sql.push_str(&format!(" AND due_at <= ?{}", params_vec.len()));
        }

        if let Some(due_after) = filter.due_after {
            params_vec.push(Box::new(due_after));
            sql.push_str(&format!(" AND due_at >= ?{}", params_vec.len()));
        }

        sql.push_str(" ORDER BY created_at DESC");

        if let Some(limit) = filter.limit {
            params_vec.push(Box::new(limit as i64));
            sql.push_str(&format!(" LIMIT ?{}", params_vec.len()));
        }

        if let Some(offset) = filter.offset {
            params_vec.push(Box::new(offset as i64));
            sql.push_str(&format!(" OFFSET ?{}", params_vec.len()));
        }

        let mut stmt = conn.prepare(&sql)?;

        let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter()
            .map(|p| p.as_ref())
            .collect();

        let rows = stmt.query_map(params_refs.as_slice(), |row| {
            Ok(row_to_task(row))
        })?;

        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(row??);
        }
        Ok(tasks)
    }

    /// Count tasks matching filter
    pub async fn count(&self, filter: TaskFilter) -> Result<u64, StorageError> {
        let conn = self.conn.lock().await;

        let mut sql = String::from("SELECT COUNT(*) FROM tasks WHERE 1=1");

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

        let count: i64 = conn.query_row(&sql, [], |row| row.get(0))?;
        Ok(count as u64)
    }
}

fn status_to_str(status: TaskStatus) -> &'static str {
    match status {
        TaskStatus::Inbox => "inbox",
        TaskStatus::Next => "next",
        TaskStatus::Waiting => "waiting",
        TaskStatus::Someday => "someday",
        TaskStatus::Done => "done",
        TaskStatus::Archived => "archived",
    }
}

fn str_to_status(s: &str) -> TaskStatus {
    match s {
        "inbox" => TaskStatus::Inbox,
        "next" => TaskStatus::Next,
        "waiting" => TaskStatus::Waiting,
        "someday" => TaskStatus::Someday,
        "done" => TaskStatus::Done,
        "archived" => TaskStatus::Archived,
        _ => TaskStatus::Inbox,
    }
}

fn row_to_task(row: &Row) -> Result<Task, StorageError> {
    let id_str: String = row.get("id")?;
    let status_str: String = row.get("status")?;
    let parent_id_str: Option<String> = row.get("parent_task_id")?;
    let goal_id_str: Option<String> = row.get("goal_id")?;
    let tags_json: Option<String> = row.get("context_tags")?;
    let energy_str: Option<String> = row.get("energy_level")?;

    Ok(Task {
        id: id_str.parse().map_err(|_| StorageError::Validation("Invalid UUID".to_string()))?,
        title: row.get("title")?,
        description_md: row.get("description_md")?,
        status: str_to_status(&status_str),
        priority: row.get("priority")?,
        priority_score: row.get("priority_score")?,
        due_at: row.get("due_at")?,
        scheduled_at: row.get("scheduled_at")?,
        completed_at: row.get("completed_at")?,
        parent_task_id: parent_id_str.and_then(|s| s.parse().ok()),
        goal_id: goal_id_str.and_then(|s| s.parse().ok()),
        context_tags: tags_json
            .and_then(|j| serde_json::from_str(&j).ok())
            .unwrap_or_default(),
        energy_level: energy_str.and_then(|s| match s.as_str() {
            "low" => Some(forge_domain::EnergyLevel::Low),
            "medium" => Some(forge_domain::EnergyLevel::Medium),
            "high" => Some(forge_domain::EnergyLevel::High),
            _ => None,
        }),
        time_estimate_min: row.get("time_estimate_min")?,
        recurrence_rule: row.get("recurrence_rule")?,
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
        deleted_at: row.get("deleted_at")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    async fn setup_test_db() -> Arc<Mutex<Connection>> {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(include_str!("../../../../migrations/0001_init.sql")).unwrap();
        Arc::new(Mutex::new(conn))
    }

    #[tokio::test]
    async fn test_create_and_get_task() {
        let conn = setup_test_db().await;
        let repo = TaskRepository::new(conn);

        let task = Task::new("Test task");
        repo.create(&task).await.unwrap();

        let fetched = repo.get(task.id).await.unwrap().unwrap();
        assert_eq!(fetched.title, "Test task");
        assert_eq!(fetched.status, TaskStatus::Inbox);
    }

    #[tokio::test]
    async fn test_update_task() {
        let conn = setup_test_db().await;
        let repo = TaskRepository::new(conn);

        let mut task = Task::new("Original title");
        repo.create(&task).await.unwrap();

        task.title = "Updated title".to_string();
        task.status = TaskStatus::Next;
        repo.update(&task).await.unwrap();

        let fetched = repo.get(task.id).await.unwrap().unwrap();
        assert_eq!(fetched.title, "Updated title");
        assert_eq!(fetched.status, TaskStatus::Next);
    }

    #[tokio::test]
    async fn test_delete_task() {
        let conn = setup_test_db().await;
        let repo = TaskRepository::new(conn);

        let task = Task::new("To delete");
        repo.create(&task).await.unwrap();

        repo.delete(task.id, chrono::Utc::now().timestamp_millis()).await.unwrap();

        let fetched = repo.get(task.id).await.unwrap();
        assert!(fetched.is_none());
    }

    #[tokio::test]
    async fn test_list_tasks() {
        let conn = setup_test_db().await;
        let repo = TaskRepository::new(conn);

        for i in 0..5 {
            let task = Task::new(format!("Task {}", i));
            repo.create(&task).await.unwrap();
        }

        let tasks = repo.list(TaskFilter::default()).await.unwrap();
        assert_eq!(tasks.len(), 5);
    }
}
