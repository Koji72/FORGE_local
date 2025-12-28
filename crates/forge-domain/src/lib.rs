//! # Forge Domain
//!
//! Domain entities and business logic for Forge.
//!
//! This crate contains:
//! - Entity definitions (Task, Note, Goal, Link)
//! - Value objects and enums
//! - Domain validation rules
//! - Business logic invariants

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

// =============================================================================
// Errors
// =============================================================================

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Invalid state transition from {from} to {to}")]
    InvalidStateTransition { from: String, to: String },

    #[error("Entity not found: {entity_type} with id {id}")]
    NotFound { entity_type: String, id: String },
}

// =============================================================================
// Common Types
// =============================================================================

/// Timestamp in milliseconds since Unix epoch (UTC)
pub type TimestampMs = i64;

/// Entity identifier (UUIDv7)
pub type EntityId = Uuid;

// =============================================================================
// Task
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Inbox,
    Next,
    Waiting,
    Someday,
    Done,
    Archived,
}

impl TaskStatus {
    pub fn can_transition_to(&self, target: TaskStatus) -> bool {
        use TaskStatus::*;
        match (self, target) {
            // From Inbox, can go anywhere
            (Inbox, _) => true,
            // From Next/Waiting/Someday, can go to Done, Archived, or back to Inbox
            (Next | Waiting | Someday, Done | Archived | Inbox) => true,
            // From Next, can go to Waiting or Someday
            (Next, Waiting | Someday) => true,
            // From Waiting/Someday, can go to Next
            (Waiting | Someday, Next) => true,
            // From Done, can only go to Archived or back to Next
            (Done, Archived | Next) => true,
            // From Archived, can only unarchive to Done
            (Archived, Done) => true,
            // Same state is always valid
            (a, b) if a == b => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnergyLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: EntityId,
    pub title: String,
    pub description_md: Option<String>,
    pub status: TaskStatus,
    pub priority: Option<u8>,
    pub priority_score: Option<f64>,
    pub due_at: Option<TimestampMs>,
    pub scheduled_at: Option<TimestampMs>,
    pub completed_at: Option<TimestampMs>,
    pub parent_task_id: Option<EntityId>,
    pub goal_id: Option<EntityId>,
    pub context_tags: Vec<String>,
    pub energy_level: Option<EnergyLevel>,
    pub time_estimate_min: Option<u32>,
    pub recurrence_rule: Option<String>,
    pub created_at: TimestampMs,
    pub updated_at: TimestampMs,
    pub deleted_at: Option<TimestampMs>,
}

impl Task {
    pub fn new(title: impl Into<String>) -> Self {
        let now = Utc::now().timestamp_millis();
        Self {
            id: Uuid::now_v7(),
            title: title.into(),
            description_md: None,
            status: TaskStatus::Inbox,
            priority: None,
            priority_score: None,
            due_at: None,
            scheduled_at: None,
            completed_at: None,
            parent_task_id: None,
            goal_id: None,
            context_tags: Vec::new(),
            energy_level: None,
            time_estimate_min: None,
            recurrence_rule: None,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    pub fn is_completed(&self) -> bool {
        self.status == TaskStatus::Done || self.status == TaskStatus::Archived
    }
}

// =============================================================================
// Note
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteType {
    Note,
    Meeting,
    Journal,
    Reference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: EntityId,
    pub title: String,
    pub content_md: String,
    pub note_type: NoteType,
    pub tags: Vec<String>,
    pub pinned: bool,
    pub created_at: TimestampMs,
    pub updated_at: TimestampMs,
    pub deleted_at: Option<TimestampMs>,
}

impl Note {
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        let now = Utc::now().timestamp_millis();
        Self {
            id: Uuid::now_v7(),
            title: title.into(),
            content_md: content.into(),
            note_type: NoteType::Note,
            tags: Vec::new(),
            pinned: false,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }
}

// =============================================================================
// Goal
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalStatus {
    Active,
    Completed,
    Paused,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalHorizon {
    Short,
    Medium,
    Long,
    Vision,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: EntityId,
    pub title: String,
    pub description_md: Option<String>,
    pub status: GoalStatus,
    pub horizon: GoalHorizon,
    pub target_date: Option<TimestampMs>,
    pub progress_percent: u8,
    pub parent_goal_id: Option<EntityId>,
    pub created_at: TimestampMs,
    pub updated_at: TimestampMs,
    pub deleted_at: Option<TimestampMs>,
}

impl Goal {
    pub fn new(title: impl Into<String>) -> Self {
        let now = Utc::now().timestamp_millis();
        Self {
            id: Uuid::now_v7(),
            title: title.into(),
            description_md: None,
            status: GoalStatus::Active,
            horizon: GoalHorizon::Medium,
            target_date: None,
            progress_percent: 0,
            parent_goal_id: None,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }
}

// =============================================================================
// Link (Entity Graph)
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Task,
    Note,
    Goal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub id: EntityId,
    pub from_type: EntityType,
    pub from_id: EntityId,
    pub to_type: EntityType,
    pub to_id: EntityId,
    pub label: Option<String>,
    pub created_at: TimestampMs,
}

impl Link {
    pub fn new(
        from_type: EntityType,
        from_id: EntityId,
        to_type: EntityType,
        to_id: EntityId,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            from_type,
            from_id,
            to_type,
            to_id,
            label: None,
            created_at: Utc::now().timestamp_millis(),
        }
    }
}

// =============================================================================
// Agent Run
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentRunStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRun {
    pub id: EntityId,
    pub agent_type: String,
    pub status: AgentRunStatus,
    pub input_json: serde_json::Value,
    pub output_json: Option<serde_json::Value>,
    pub error_json: Option<serde_json::Value>,
    pub tokens_used: Option<u32>,
    pub duration_ms: Option<u64>,
    pub started_at: Option<TimestampMs>,
    pub completed_at: Option<TimestampMs>,
    pub created_at: TimestampMs,
}

impl AgentRun {
    pub fn new(agent_type: impl Into<String>, input: serde_json::Value) -> Self {
        Self {
            id: Uuid::now_v7(),
            agent_type: agent_type.into(),
            status: AgentRunStatus::Pending,
            input_json: input,
            output_json: None,
            error_json: None,
            tokens_used: None,
            duration_ms: None,
            started_at: None,
            completed_at: None,
            created_at: Utc::now().timestamp_millis(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_status_transitions() {
        assert!(TaskStatus::Inbox.can_transition_to(TaskStatus::Next));
        assert!(TaskStatus::Inbox.can_transition_to(TaskStatus::Done));
        assert!(TaskStatus::Next.can_transition_to(TaskStatus::Done));
        assert!(TaskStatus::Done.can_transition_to(TaskStatus::Archived));
        assert!(!TaskStatus::Archived.can_transition_to(TaskStatus::Inbox));
    }

    #[test]
    fn test_new_task() {
        let task = Task::new("Test task");
        assert_eq!(task.title, "Test task");
        assert_eq!(task.status, TaskStatus::Inbox);
        assert!(!task.is_deleted());
        assert!(!task.is_completed());
    }

    #[test]
    fn test_new_note() {
        let note = Note::new("Test note", "# Content");
        assert_eq!(note.title, "Test note");
        assert_eq!(note.content_md, "# Content");
        assert_eq!(note.note_type, NoteType::Note);
    }
}
