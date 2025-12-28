//! IPC command request and response DTOs

use serde::{Deserialize, Serialize};

// =============================================================================
// Common Types
// =============================================================================

pub type EntityId = String;
pub type TimestampMs = i64;

// =============================================================================
// Vault Commands
// =============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultInfo {
    pub vault_id: String,
    pub db_path: String,
    pub encrypted: bool,
    pub created_at: TimestampMs,
    pub size_bytes: u64,
    pub entity_counts: EntityCounts,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityCounts {
    pub tasks: u64,
    pub notes: u64,
    pub goals: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportRequest {
    pub format: ExportFormat,
    pub path: String,
    #[serde(default)]
    pub include_deleted: bool,
    #[serde(default)]
    pub entities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportFormat {
    Json,
    Markdown,
    Sqlite,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportResponse {
    pub path: String,
    pub size_bytes: u64,
    pub entity_counts: EntityCounts,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportRequest {
    pub path: String,
    pub format: ExportFormat,
    #[serde(default)]
    pub merge_strategy: MergeStrategy,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeStrategy {
    #[default]
    SkipExisting,
    Overwrite,
    CreateNew,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportResponse {
    pub imported_counts: EntityCounts,
    pub skipped_counts: EntityCounts,
    pub errors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupResponse {
    pub backup_id: String,
    pub path: String,
    pub size_bytes: u64,
    pub checksum: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RotateBackupsResponse {
    pub kept: u32,
    pub deleted: u32,
}

// =============================================================================
// Task Commands
// =============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    #[serde(default)]
    pub description_md: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub priority: Option<u8>,
    #[serde(default)]
    pub due_at: Option<TimestampMs>,
    #[serde(default)]
    pub scheduled_at: Option<TimestampMs>,
    #[serde(default)]
    pub parent_task_id: Option<EntityId>,
    #[serde(default)]
    pub goal_id: Option<EntityId>,
    #[serde(default)]
    pub context_tags: Vec<String>,
    #[serde(default)]
    pub energy_level: Option<String>,
    #[serde(default)]
    pub time_estimate_min: Option<u32>,
    #[serde(default)]
    pub recurrence_rule: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub id: EntityId,
    pub title: String,
    pub description_md: Option<String>,
    pub status: String,
    pub priority: Option<u8>,
    pub priority_score: Option<f64>,
    pub due_at: Option<TimestampMs>,
    pub scheduled_at: Option<TimestampMs>,
    pub completed_at: Option<TimestampMs>,
    pub parent_task_id: Option<EntityId>,
    pub goal_id: Option<EntityId>,
    pub context_tags: Vec<String>,
    pub energy_level: Option<String>,
    pub time_estimate_min: Option<u32>,
    pub recurrence_rule: Option<String>,
    pub created_at: TimestampMs,
    pub updated_at: TimestampMs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub id: EntityId,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description_md: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub priority: Option<u8>,
    #[serde(default)]
    pub due_at: Option<TimestampMs>,
    #[serde(default)]
    pub scheduled_at: Option<TimestampMs>,
    #[serde(default)]
    pub goal_id: Option<EntityId>,
    #[serde(default)]
    pub context_tags: Option<Vec<String>>,
    #[serde(default)]
    pub energy_level: Option<String>,
    #[serde(default)]
    pub time_estimate_min: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResponse {
    pub id: EntityId,
    pub updated_at: TimestampMs,
    pub changes: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListTasksRequest {
    #[serde(default)]
    pub status: Option<Vec<String>>,
    #[serde(default)]
    pub goal_id: Option<EntityId>,
    #[serde(default)]
    pub due_before: Option<TimestampMs>,
    #[serde(default)]
    pub due_after: Option<TimestampMs>,
    #[serde(default)]
    pub context_tags: Option<Vec<String>>,
    #[serde(default)]
    pub include_deleted: bool,
    #[serde(default)]
    pub order_by: Option<String>,
    #[serde(default)]
    pub order_dir: Option<String>,
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub offset: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTasksResponse {
    pub tasks: Vec<TaskResponse>,
    pub total: u64,
    pub has_more: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetByIdRequest {
    pub id: EntityId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteRequest {
    pub id: EntityId,
    #[serde(default)]
    pub hard_delete: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteResponse {
    pub id: EntityId,
    pub deleted_at: TimestampMs,
}

// =============================================================================
// Note Commands
// =============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub content_md: String,
    #[serde(default)]
    pub note_type: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub pinned: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteResponse {
    pub id: EntityId,
    pub title: String,
    pub content_md: String,
    pub note_type: String,
    pub tags: Vec<String>,
    pub pinned: bool,
    pub created_at: TimestampMs,
    pub updated_at: TimestampMs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNoteRequest {
    pub id: EntityId,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub content_md: Option<String>,
    #[serde(default)]
    pub note_type: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub pinned: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListNotesRequest {
    #[serde(default)]
    pub note_type: Option<Vec<String>>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub pinned_only: bool,
    #[serde(default)]
    pub include_deleted: bool,
    #[serde(default)]
    pub order_by: Option<String>,
    #[serde(default)]
    pub order_dir: Option<String>,
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub offset: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListNotesResponse {
    pub notes: Vec<NoteResponse>,
    pub total: u64,
    pub has_more: bool,
}

// =============================================================================
// Goal Commands
// =============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGoalRequest {
    pub title: String,
    #[serde(default)]
    pub description_md: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub horizon: Option<String>,
    #[serde(default)]
    pub target_date: Option<TimestampMs>,
    #[serde(default)]
    pub parent_goal_id: Option<EntityId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalResponse {
    pub id: EntityId,
    pub title: String,
    pub description_md: Option<String>,
    pub status: String,
    pub horizon: String,
    pub target_date: Option<TimestampMs>,
    pub progress_percent: u8,
    pub parent_goal_id: Option<EntityId>,
    pub created_at: TimestampMs,
    pub updated_at: TimestampMs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGoalRequest {
    pub id: EntityId,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description_md: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub horizon: Option<String>,
    #[serde(default)]
    pub target_date: Option<TimestampMs>,
    #[serde(default)]
    pub progress_percent: Option<u8>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListGoalsRequest {
    #[serde(default)]
    pub status: Option<Vec<String>>,
    #[serde(default)]
    pub horizon: Option<Vec<String>>,
    #[serde(default)]
    pub include_deleted: bool,
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub offset: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListGoalsResponse {
    pub goals: Vec<GoalResponse>,
    pub total: u64,
    pub has_more: bool,
}

// =============================================================================
// Link Commands
// =============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLinkRequest {
    pub from_type: String,
    pub from_id: EntityId,
    pub to_type: String,
    pub to_id: EntityId,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkResponse {
    pub id: EntityId,
    pub from_type: String,
    pub from_id: EntityId,
    pub to_type: String,
    pub to_id: EntityId,
    pub label: Option<String>,
    pub created_at: TimestampMs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListLinksByEntityRequest {
    pub entity_type: String,
    pub entity_id: EntityId,
    #[serde(default)]
    pub direction: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListLinksResponse {
    pub links: Vec<LinkResponse>,
}

// =============================================================================
// Search Commands
// =============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct FtsSearchRequest {
    pub query: String,
    #[serde(default)]
    pub entity_types: Vec<String>,
    #[serde(default)]
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub entity_type: String,
    pub entity_id: EntityId,
    pub title: String,
    pub snippet: Option<String>,
    pub score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FtsSearchResponse {
    pub results: Vec<SearchResult>,
    pub total: u64,
    pub query_time_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SemanticSearchRequest {
    pub query: String,
    #[serde(default)]
    pub entity_types: Vec<String>,
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub min_score: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SemanticSearchResult {
    pub entity_type: String,
    pub entity_id: EntityId,
    pub title: String,
    pub similarity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SemanticSearchResponse {
    pub results: Vec<SemanticSearchResult>,
    pub query_time_ms: u64,
    pub embedding_model: String,
}

// =============================================================================
// Agent Commands
// =============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct RunAgentRequest {
    pub agent_type: String,
    #[serde(default)]
    pub context: serde_json::Value,
    #[serde(default)]
    pub options: AgentOptions,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AgentOptions {
    #[serde(default)]
    pub timeout_ms: Option<u64>,
    #[serde(default)]
    pub dry_run: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunAgentResponse {
    pub run_id: EntityId,
    pub status: String,
    pub started_at: TimestampMs,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentRunResponse {
    pub id: EntityId,
    pub agent_type: String,
    pub status: String,
    pub input_json: serde_json::Value,
    pub output_json: Option<serde_json::Value>,
    pub error_json: Option<serde_json::Value>,
    pub tokens_used: Option<u32>,
    pub duration_ms: Option<u64>,
    pub started_at: Option<TimestampMs>,
    pub completed_at: Option<TimestampMs>,
    pub created_at: TimestampMs,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListAgentRunsRequest {
    #[serde(default)]
    pub agent_type: Option<String>,
    #[serde(default)]
    pub status: Option<Vec<String>>,
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub offset: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListAgentRunsResponse {
    pub runs: Vec<AgentRunResponse>,
    pub total: u64,
}

// =============================================================================
// Sync Commands
// =============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct EnableSyncRequest {
    pub relay_url: String,
    pub passphrase: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnableSyncResponse {
    pub enabled: bool,
    pub vault_id: String,
    pub device_id: String,
    pub relay_connected: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncStatusResponse {
    pub enabled: bool,
    pub relay_url: Option<String>,
    pub relay_connected: bool,
    pub vault_id: Option<String>,
    pub device_id: Option<String>,
    pub last_push_at: Option<TimestampMs>,
    pub last_pull_at: Option<TimestampMs>,
    pub pending_changes: u32,
    pub devices: Vec<SyncDevice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncDevice {
    pub device_id: String,
    pub device_name: Option<String>,
    pub last_seen_at: Option<TimestampMs>,
    pub is_current: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PushPullResponse {
    pub pushed: u32,
    pub pulled: u32,
    pub conflicts_resolved: u32,
    pub duration_ms: u64,
}

// =============================================================================
// Settings Commands
// =============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSettingRequest {
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSettingResponse {
    pub key: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetSettingRequest {
    pub key: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetSettingResponse {
    pub key: String,
    pub updated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllSettingsResponse {
    pub settings: std::collections::HashMap<String, serde_json::Value>,
}
