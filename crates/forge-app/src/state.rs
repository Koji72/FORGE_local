//! Application state management

use forge_storage::Database;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Application state containing all services
pub struct AppState {
    /// Database instance
    pub db: Database,
    /// Path to the vault directory
    pub vault_path: PathBuf,
    /// Vault ID
    pub vault_id: String,
}

impl AppState {
    /// Initialize application state with database
    pub async fn init(vault_path: PathBuf, password: Option<&str>) -> Result<Self, AppError> {
        let db_path = vault_path.join("forge.db");

        let db = Database::open(&db_path, password)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Run migrations
        db.migrate()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Get or create vault ID
        let vault_id = db.get_or_create_vault_id()
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(Self {
            db,
            vault_path,
            vault_id,
        })
    }

    /// Get task repository
    pub fn tasks(&self) -> &forge_storage::TaskRepository {
        self.db.tasks()
    }

    /// Get note repository
    pub fn notes(&self) -> &forge_storage::NoteRepository {
        self.db.notes()
    }

    /// Get goal repository
    pub fn goals(&self) -> &forge_storage::GoalRepository {
        self.db.goals()
    }

    /// Get link repository
    pub fn links(&self) -> &forge_storage::LinkRepository {
        self.db.links()
    }

    /// Get settings repository
    pub fn settings(&self) -> &forge_storage::SettingsRepository {
        self.db.settings()
    }

    /// Get agent run repository
    pub fn agent_runs(&self) -> &forge_storage::AgentRunRepository {
        self.db.agent_runs()
    }
}

/// Shared application state type
pub type SharedState = Arc<RwLock<AppState>>;

/// Application error type
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("AppError", 2)?;

        let (code, message) = match self {
            AppError::Database(msg) => ("DATABASE_ERROR", msg.as_str()),
            AppError::NotFound(msg) => ("NOT_FOUND", msg.as_str()),
            AppError::InvalidRequest(msg) => ("INVALID_REQUEST", msg.as_str()),
            AppError::Internal(msg) => ("INTERNAL_ERROR", msg.as_str()),
        };

        state.serialize_field("code", code)?;
        state.serialize_field("message", message)?;
        state.end()
    }
}

/// API response wrapper
#[derive(Debug, serde::Serialize)]
pub struct ApiResponse<T: serde::Serialize> {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<AppError>,
}

impl<T: serde::Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            ok: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(err: AppError) -> Self {
        Self {
            ok: false,
            data: None,
            error: Some(err),
        }
    }
}

/// Generate a new entity ID (UUIDv7)
pub fn new_entity_id() -> String {
    uuid::Uuid::now_v7().to_string()
}

/// Get current timestamp in milliseconds
pub fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}
