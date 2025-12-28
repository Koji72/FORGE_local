//! # Forge Sync
//!
//! E2EE sync protocol and CRDT handling for Forge.
//!
//! This crate provides:
//! - CRDT operations using yrs
//! - E2EE encryption/decryption
//! - WebSocket relay client
//! - SQL materialization from CRDT

pub mod client;
pub mod crypto;
pub mod crdt;
pub mod protocol;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Sync disabled")]
    Disabled,

    #[error("Crypto error: {0}")]
    Crypto(String),

    #[error("Transport error: {0}")]
    Transport(String),

    #[error("CRDT error: {0}")]
    Crdt(String),

    #[error("Protocol error: {0}")]
    Protocol(String),
}
