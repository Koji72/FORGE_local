//! # Forge Agents
//!
//! AI agent orchestration and execution for Forge.
//!
//! This crate provides:
//! - Agent definitions and schemas
//! - LLM adapter (Ollama)
//! - Tool implementations
//! - Output validation and repair

pub mod executor;
pub mod llm;
pub mod schema;
pub mod tools;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("LLM unavailable: {0}")]
    LlmUnavailable(String),

    #[error("LLM timeout after {0}ms")]
    LlmTimeout(u64),

    #[error("LLM output validation failed: {0}")]
    LlmBadOutput(String),

    #[error("Agent not found: {0}")]
    AgentNotFound(String),

    #[error("Tool error: {0}")]
    Tool(String),

    #[error("Schema validation error: {0}")]
    Schema(String),
}
