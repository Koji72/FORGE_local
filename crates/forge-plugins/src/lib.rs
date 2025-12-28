//! # Forge Plugins
//!
//! Plugin system with permissions and sandboxing for Forge.
//!
//! This crate provides:
//! - Plugin manifest parsing
//! - Permission system
//! - Signature verification
//! - Plugin lifecycle management

pub mod manifest;
pub mod permissions;
pub mod signature;
pub mod runtime;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Manifest error: {0}")]
    Manifest(String),

    #[error("Runtime error: {0}")]
    Runtime(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
