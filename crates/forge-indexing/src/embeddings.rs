//! Embedding generation via Ollama

use crate::IndexingError;
use serde::{Deserialize, Serialize};

/// Ollama embeddings client
pub struct OllamaEmbeddings {
    base_url: String,
    model: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct EmbeddingRequest {
    model: String,
    prompt: String,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    embedding: Vec<f32>,
}

impl OllamaEmbeddings {
    pub fn new(base_url: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            model: model.into(),
            client: reqwest::Client::new(),
        }
    }

    /// Generate embedding for text
    pub async fn embed(&self, text: &str) -> Result<Vec<f32>, IndexingError> {
        let url = format!("{}/api/embeddings", self.base_url);
        let request = EmbeddingRequest {
            model: self.model.clone(),
            prompt: text.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .error_for_status()
            .map_err(|e| IndexingError::Embedding(e.to_string()))?;

        let embedding_response: EmbeddingResponse = response.json().await?;
        Ok(embedding_response.embedding)
    }
}
