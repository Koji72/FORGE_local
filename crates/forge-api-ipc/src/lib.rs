//! # Forge API IPC
//!
//! IPC API definitions and DTOs for Forge.
//!
//! This crate provides:
//! - Request/response DTOs
//! - Error codes
//! - API versioning

pub mod commands;
pub mod error;
pub mod events;

use serde::{Deserialize, Serialize};

/// Standard API response wrapper
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Ok { ok: bool, data: T },
    Err { ok: bool, error: ApiError },
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        ApiResponse::Ok { ok: true, data }
    }

    pub fn err(error: ApiError) -> Self {
        ApiResponse::Err { ok: false, error }
    }
}

/// Standard API error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ApiError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

// Pre-defined error codes
pub mod error_codes {
    pub const ERR_VALIDATION: &str = "ERR_VALIDATION";
    pub const ERR_NOT_FOUND: &str = "ERR_NOT_FOUND";
    pub const ERR_CONFLICT: &str = "ERR_CONFLICT";
    pub const ERR_DB_IO: &str = "ERR_DB_IO";
    pub const ERR_DB_MIGRATION: &str = "ERR_DB_MIGRATION";
    pub const ERR_DB_LOCKED: &str = "ERR_DB_LOCKED";
    pub const ERR_INDEXING: &str = "ERR_INDEXING";
    pub const ERR_LLM_UNAVAILABLE: &str = "ERR_LLM_UNAVAILABLE";
    pub const ERR_LLM_TIMEOUT: &str = "ERR_LLM_TIMEOUT";
    pub const ERR_LLM_BAD_OUTPUT: &str = "ERR_LLM_BAD_OUTPUT";
    pub const ERR_SYNC_DISABLED: &str = "ERR_SYNC_DISABLED";
    pub const ERR_SYNC_CRYPTO: &str = "ERR_SYNC_CRYPTO";
    pub const ERR_SYNC_TRANSPORT: &str = "ERR_SYNC_TRANSPORT";
    pub const ERR_PLUGIN_PERMISSION: &str = "ERR_PLUGIN_PERMISSION";
    pub const ERR_PLUGIN_INVALID_SIGNATURE: &str = "ERR_PLUGIN_INVALID_SIGNATURE";
    pub const ERR_PLUGIN_RUNTIME: &str = "ERR_PLUGIN_RUNTIME";
    pub const ERR_INTERNAL: &str = "ERR_INTERNAL";
}
