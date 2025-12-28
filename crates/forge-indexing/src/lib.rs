//! # Forge Indexing
//!
//! Full-text and semantic search indexing for Forge.
//!
//! This crate provides:
//! - FTS5 full-text search integration
//! - Embedding generation via Ollama
//! - HNSW approximate nearest neighbor search
//! - Hybrid search combining FTS and semantic

pub mod embeddings;
pub mod fts;
pub mod hnsw;
pub mod hybrid;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexingError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Embedding error: {0}")]
    Embedding(String),

    #[error("HNSW error: {0}")]
    Hnsw(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
