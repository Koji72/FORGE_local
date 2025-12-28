//! # Forge App
//!
//! Application wiring and Tauri command handlers for Forge.
//!
//! This crate provides:
//! - Application state management
//! - Tauri command implementations
//! - Event emitters
//! - Service orchestration

pub mod state;
pub mod handlers;

use std::sync::Arc;
use tokio::sync::RwLock;

/// Application state shared across handlers
pub struct AppState {
    // TODO: Add database connection, services, etc.
}

impl AppState {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared application state type
pub type SharedState = Arc<RwLock<AppState>>;

/// Create shared application state
pub fn create_shared_state() -> SharedState {
    Arc::new(RwLock::new(AppState::new()))
}
