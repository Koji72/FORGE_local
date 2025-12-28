//! # Forge App
//!
//! Application wiring and Tauri command handlers for Forge.
//!
//! This crate provides:
//! - Application state management
//! - Tauri command implementations
//! - Event emitters
//! - Service orchestration

pub mod handlers;
pub mod state;

pub use handlers::*;
pub use state::{AppState, SharedState, AppError, ApiResponse};

use std::sync::Arc;
use tokio::sync::RwLock;

/// Create shared application state
pub async fn create_shared_state(
    vault_path: std::path::PathBuf,
    password: Option<&str>,
) -> Result<SharedState, state::AppError> {
    let state = AppState::init(vault_path, password).await?;
    Ok(Arc::new(RwLock::new(state)))
}

/// Register all Tauri commands with the app builder
#[macro_export]
macro_rules! register_commands {
    () => {
        tauri::generate_handler![
            // Tasks
            forge_app::v1_tasks_create,
            forge_app::v1_tasks_get,
            forge_app::v1_tasks_update,
            forge_app::v1_tasks_delete,
            forge_app::v1_tasks_list,
            // Notes
            forge_app::v1_notes_create,
            forge_app::v1_notes_get,
            forge_app::v1_notes_update,
            forge_app::v1_notes_delete,
            forge_app::v1_notes_list,
            // Goals
            forge_app::v1_goals_create,
            forge_app::v1_goals_get,
            forge_app::v1_goals_update,
            forge_app::v1_goals_delete,
            forge_app::v1_goals_list,
            // Search
            forge_app::v1_search_fts,
            forge_app::v1_search_semantic,
            // Settings
            forge_app::v1_settings_get,
            forge_app::v1_settings_set,
            forge_app::v1_settings_get_all,
            // Vault
            forge_app::v1_vault_info,
        ]
    };
}
