//! Error conversion utilities

use crate::{error_codes, ApiError};

impl From<forge_domain::DomainError> for ApiError {
    fn from(err: forge_domain::DomainError) -> Self {
        match err {
            forge_domain::DomainError::Validation(msg) => {
                ApiError::new(error_codes::ERR_VALIDATION, msg)
            }
            forge_domain::DomainError::InvalidStateTransition { from, to } => ApiError::new(
                error_codes::ERR_VALIDATION,
                format!("Invalid state transition from {} to {}", from, to),
            ),
            forge_domain::DomainError::NotFound { entity_type, id } => ApiError::new(
                error_codes::ERR_NOT_FOUND,
                format!("{} with id {} not found", entity_type, id),
            ),
        }
    }
}
