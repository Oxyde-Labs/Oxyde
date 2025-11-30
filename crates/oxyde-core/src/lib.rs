//! Core types and utilities for the Oxyde SDK

mod error;

pub use error::OxydeError;

use std::collections::HashMap;

/// Result type alias using OxydeError
pub type Result<T> = std::result::Result<T, OxydeError>;

/// Agent context type - a map of string keys to JSON values
/// Used to pass arbitrary context data to behaviors
pub type AgentContext = HashMap<String, serde_json::Value>;
