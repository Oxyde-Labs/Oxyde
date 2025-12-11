//! Engine bindings for Oxyde SDK
//!
//! This module provides bindings for integrating Oxyde with various game engines.

// Re-exports
pub use self::unity::{UnityBinding, UnityAgentState};
pub use self::unreal::{UnrealBinding, UnrealAgentConfig};
pub use self::wasm::WasmBinding;

// Modules
pub mod unity;
pub mod unreal;
pub mod wasm;

use std::sync::Arc;
use crate::agent::Agent;
use crate::config::AgentConfig;
use crate::{OxydeError, Result};

/// Common trait for all engine bindings
pub trait EngineBinding {
    /// Create a new agent from a configuration file
    ///
    /// # Arguments
    ///
    /// * `config_path` - Path to the agent configuration file
    ///
    /// # Returns
    ///
    /// A new agent instance or an error
    fn create_agent(&self, config_path: &str) -> Result<Arc<Agent>>;

    /// Create a new agent from a configuration JSON string
    ///
    /// # Arguments
    ///
    /// * `json_config` - The agent configuration as a JSON string
    ///
    /// # Returns
    ///
    /// A new agent instance or an error
    fn create_agent_from_json(&self, json_config: &str) -> Result<Arc<Agent>>;
    
    /// Update an agent with new context data
    ///
    /// # Arguments
    ///
    /// * `agent` - Agent to update
    /// * `context_json` - JSON string with context data
    ///
    /// # Returns
    ///
    /// Success or an error
    fn update_agent(&self, agent: &Agent, context_json: &str) -> Result<()>;
    
    /// Process input for an agent
    ///
    /// # Arguments
    ///
    /// * `agent` - Agent to process input for
    /// * `input` - Input text
    ///
    /// # Returns
    ///
    /// Agent's response or an error
    fn process_input(&self, agent: &Agent, input: &str) -> Result<String>;
    
    /// Get the binding name
    ///
    /// # Returns
    ///
    /// Name of the binding
    fn name(&self) -> &'static str;
}

/// Helper function to load an agent configuration from a file
///
/// # Arguments
///
/// * `config_path` - Path to the agent configuration file
///
/// # Returns
///
/// An agent configuration or an error
pub fn load_agent_config(config_path: &str) -> Result<AgentConfig> {
    AgentConfig::from_file(config_path).map_err(|e| {
        OxydeError::BindingError(format!("Failed to load agent config: {}", e))
    })
}

/// Helper function to parse agent configuration from JSON string
///
/// # Arguments
///
/// * `json` - JSON string containing the agent configuration
///
/// # Returns
///
/// An agent configuration or an error
pub fn parse_agent_config_json(json: &str) -> Result<AgentConfig> {
    serde_json::from_str(json).map_err(|e| {
        OxydeError::BindingError(format!("Failed to parse agent config JSON: {}", e))
    })
}

/// Helper function to parse context JSON
///
/// # Arguments
///
/// * `context_json` - JSON string with context data
///
/// # Returns
///
/// Parsed context data or an error
pub fn parse_context_json(context_json: &str) -> Result<serde_json::Map<String, serde_json::Value>> {
    serde_json::from_str(context_json).map_err(|e| {
        OxydeError::BindingError(format!("Failed to parse context JSON: {}", e))
    })
}
