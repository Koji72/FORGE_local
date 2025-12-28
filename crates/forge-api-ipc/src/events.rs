//! IPC event definitions

use serde::{Deserialize, Serialize};

/// Database change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbChangedEvent {
    pub entity_type: String,
    pub entity_id: String,
    pub operation: String,
    pub changed_fields: Vec<String>,
}

/// Agent progress event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProgressEvent {
    pub run_id: String,
    pub progress_percent: u8,
    pub current_step: String,
    pub tokens_so_far: Option<u32>,
}

/// Sync state event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStateEvent {
    pub state: String,
    pub pending_changes: u32,
    pub last_error: Option<String>,
}

/// Toast notification event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToastEvent {
    pub level: String,
    pub title: String,
    pub message: String,
    pub duration_ms: u32,
}

/// Plugin state event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginStateEvent {
    pub plugin_id: String,
    pub state: String,
    pub error: Option<String>,
}
