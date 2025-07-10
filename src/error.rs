//! Error types for the Oxyde SDK
//!
//! This module provides the error types used throughout the SDK.

use std::fmt;
use std::io;

use thiserror::Error;

/// Main error type for the Oxyde SDK
#[derive(Error, Debug)]
pub enum OxydeError {
    /// Configuration errors
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Memory system errors
    #[error("Memory error: {0}")]
    MemoryError(String),

    /// Inference engine errors
    #[error("Inference error: {0}")]
    InferenceError(String),

    /// Intent understanding errors
    #[error("Intent error: {0}")]
    IntentError(String),

    /// Behavior execution errors
    #[error("Behavior error: {0}")]
    BehaviorError(String),

    /// Engine binding errors
    #[error("Binding error: {0}")]
    BindingError(String),

    /// IO errors
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Request errors
    #[error("Request error: {0}")]
    RequestError(String),
    
    /// CLI errors
    #[error("CLI error: {0}")]
    CliError(String),
}

// Display implementation is automatically provided by thiserror derive macro
// No need for manual implementation
