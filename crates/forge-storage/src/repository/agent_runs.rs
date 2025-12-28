//! Agent runs repository implementation

use crate::StorageError;
use forge_domain::{AgentRun, AgentRunStatus, EntityId, TimestampMs};
use rusqlite::{params, Connection, Row};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Agent run filter for queries
#[derive(Debug, Default, Clone)]
pub struct AgentRunFilter {
    pub agent_type: Option<String>,
    pub status: Option<Vec<AgentRunStatus>>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Agent run repository
pub struct AgentRunRepository {
    conn: Arc<Mutex<Connection>>,
}

impl AgentRunRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Create a new agent run
    pub async fn create(&self, run: &AgentRun) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        conn.execute(
            r#"
            INSERT INTO agent_runs (
                id, agent_type, status, input_json, output_json, error_json,
                tokens_used, duration_ms, started_at, completed_at, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#,
            params![
                run.id.to_string(),
                run.agent_type,
                status_to_str(run.status),
                serde_json::to_string(&run.input_json).ok(),
                run.output_json.as_ref().and_then(|v| serde_json::to_string(v).ok()),
                run.error_json.as_ref().and_then(|v| serde_json::to_string(v).ok()),
                run.tokens_used,
                run.duration_ms.map(|d| d as i64),
                run.started_at,
                run.completed_at,
                run.created_at,
            ],
        )?;
        Ok(())
    }

    /// Get an agent run by ID
    pub async fn get(&self, id: EntityId) -> Result<Option<AgentRun>, StorageError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT * FROM agent_runs WHERE id = ?1")?;

        let result = stmt.query_row(params![id.to_string()], |row| {
            Ok(row_to_agent_run(row))
        });

        match result {
            Ok(run) => Ok(Some(run?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Update an agent run
    pub async fn update(&self, run: &AgentRun) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        conn.execute(
            r#"
            UPDATE agent_runs SET
                status = ?2,
                output_json = ?3,
                error_json = ?4,
                tokens_used = ?5,
                duration_ms = ?6,
                started_at = ?7,
                completed_at = ?8
            WHERE id = ?1
            "#,
            params![
                run.id.to_string(),
                status_to_str(run.status),
                run.output_json.as_ref().and_then(|v| serde_json::to_string(v).ok()),
                run.error_json.as_ref().and_then(|v| serde_json::to_string(v).ok()),
                run.tokens_used,
                run.duration_ms.map(|d| d as i64),
                run.started_at,
                run.completed_at,
            ],
        )?;
        Ok(())
    }

    /// Mark a run as started
    pub async fn mark_started(&self, id: EntityId, started_at: TimestampMs) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        conn.execute(
            "UPDATE agent_runs SET status = 'running', started_at = ?2 WHERE id = ?1",
            params![id.to_string(), started_at],
        )?;
        Ok(())
    }

    /// Mark a run as completed
    pub async fn mark_completed(
        &self,
        id: EntityId,
        output: serde_json::Value,
        tokens_used: Option<u32>,
        completed_at: TimestampMs,
    ) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        let run = self.get(id).await?.ok_or_else(|| StorageError::NotFound {
            entity_type: "AgentRun".to_string(),
            id: id.to_string(),
        })?;

        let duration_ms = run.started_at.map(|s| (completed_at - s) as u64);

        conn.execute(
            r#"
            UPDATE agent_runs SET
                status = 'completed',
                output_json = ?2,
                tokens_used = ?3,
                duration_ms = ?4,
                completed_at = ?5
            WHERE id = ?1
            "#,
            params![
                id.to_string(),
                serde_json::to_string(&output).ok(),
                tokens_used,
                duration_ms.map(|d| d as i64),
                completed_at,
            ],
        )?;
        Ok(())
    }

    /// Mark a run as failed
    pub async fn mark_failed(
        &self,
        id: EntityId,
        error: serde_json::Value,
        completed_at: TimestampMs,
    ) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        conn.execute(
            r#"
            UPDATE agent_runs SET
                status = 'failed',
                error_json = ?2,
                completed_at = ?3
            WHERE id = ?1
            "#,
            params![
                id.to_string(),
                serde_json::to_string(&error).ok(),
                completed_at,
            ],
        )?;
        Ok(())
    }

    /// Mark a run as cancelled
    pub async fn mark_cancelled(&self, id: EntityId, cancelled_at: TimestampMs) -> Result<(), StorageError> {
        let conn = self.conn.lock().await;
        conn.execute(
            "UPDATE agent_runs SET status = 'cancelled', completed_at = ?2 WHERE id = ?1",
            params![id.to_string(), cancelled_at],
        )?;
        Ok(())
    }

    /// List agent runs with optional filters
    pub async fn list(&self, filter: AgentRunFilter) -> Result<Vec<AgentRun>, StorageError> {
        let conn = self.conn.lock().await;

        let mut sql = String::from("SELECT * FROM agent_runs WHERE 1=1");

        if let Some(ref agent_type) = filter.agent_type {
            sql.push_str(&format!(" AND agent_type = '{}'", agent_type));
        }

        if let Some(ref statuses) = filter.status {
            if !statuses.is_empty() {
                let status_strs: Vec<String> = statuses.iter()
                    .map(|s| format!("'{}'", status_to_str(*s)))
                    .collect();
                sql.push_str(&format!(" AND status IN ({})", status_strs.join(",")));
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
        let rows = stmt.query_map([], |row| Ok(row_to_agent_run(row)))?;

        let mut runs = Vec::new();
        for row in rows {
            runs.push(row??);
        }
        Ok(runs)
    }
}

fn status_to_str(status: AgentRunStatus) -> &'static str {
    match status {
        AgentRunStatus::Pending => "pending",
        AgentRunStatus::Running => "running",
        AgentRunStatus::Completed => "completed",
        AgentRunStatus::Failed => "failed",
        AgentRunStatus::Cancelled => "cancelled",
    }
}

fn str_to_status(s: &str) -> AgentRunStatus {
    match s {
        "pending" => AgentRunStatus::Pending,
        "running" => AgentRunStatus::Running,
        "completed" => AgentRunStatus::Completed,
        "failed" => AgentRunStatus::Failed,
        "cancelled" => AgentRunStatus::Cancelled,
        _ => AgentRunStatus::Pending,
    }
}

fn row_to_agent_run(row: &Row) -> Result<AgentRun, StorageError> {
    let id_str: String = row.get("id")?;
    let status_str: String = row.get("status")?;
    let input_json_str: String = row.get("input_json")?;
    let output_json_str: Option<String> = row.get("output_json")?;
    let error_json_str: Option<String> = row.get("error_json")?;
    let duration_ms: Option<i64> = row.get("duration_ms")?;

    Ok(AgentRun {
        id: id_str.parse().map_err(|_| StorageError::Validation("Invalid UUID".to_string()))?,
        agent_type: row.get("agent_type")?,
        status: str_to_status(&status_str),
        input_json: serde_json::from_str(&input_json_str).unwrap_or(serde_json::Value::Null),
        output_json: output_json_str.and_then(|s| serde_json::from_str(&s).ok()),
        error_json: error_json_str.and_then(|s| serde_json::from_str(&s).ok()),
        tokens_used: row.get("tokens_used")?,
        duration_ms: duration_ms.map(|d| d as u64),
        started_at: row.get("started_at")?,
        completed_at: row.get("completed_at")?,
        created_at: row.get("created_at")?,
    })
}
