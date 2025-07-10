//! WebAssembly bindings for Oxyde SDK
//!
//! This module provides bindings for integrating Oxyde with WebAssembly
//! for browser-based games.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use uuid::Uuid;

use crate::agent::{Agent, AgentContext, AgentState};
use crate::config::AgentConfig;
use crate::oxyde_game::bindings::{EngineBinding, load_agent_config, parse_context_json};
use crate::{OxydeError, Result};

/// WebAssembly binding for Oxyde SDK
pub struct WasmBinding {
    /// Registry of created agents
    agents: Arc<Mutex<HashMap<String, Agent>>>,
}

impl WasmBinding {
    /// Create a new WebAssembly binding
    pub fn new() -> Self {
        Self {
            agents: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Get an agent by ID
    ///
    /// # Arguments
    ///
    /// * `id` - Agent ID
    ///
    /// # Returns
    ///
    /// The agent or an error if not found
    pub fn get_agent(&self, id: &str) -> Result<Agent> {
        let agents = self.agents.lock().unwrap();
        agents.get(id)
            .map(|agent| agent.clone_for_binding())
            .ok_or_else(|| {
                OxydeError::BindingError(format!("Agent with ID {} not found", id))
            })
    }
    
    /// Register a new agent
    ///
    /// # Arguments
    ///
    /// * `id` - Agent unique identifier
    /// * `agent` - Agent to register
    pub fn register_agent(&self, id: Uuid, agent: Agent) {
        let mut agents = self.agents.lock().unwrap();
        agents.insert(id.to_string(), agent);
    }
    
    /// Parse WebAssembly context
    ///
    /// # Arguments
    ///
    /// * `context_json` - JSON string with WebAssembly context data
    ///
    /// # Returns
    ///
    /// Oxyde agent context or an error
    pub fn parse_wasm_context(&self, context_json: &str) -> Result<AgentContext> {
        let context_map = parse_context_json(context_json)?;
        
        let mut agent_context = AgentContext::new();
        for (key, value) in context_map {
            agent_context.insert(key, value);
        }
        
        Ok(agent_context)
    }
    
    /// Get agent state
    ///
    /// # Arguments
    ///
    /// * `agent` - Agent to get state for
    ///
    /// # Returns
    ///
    /// Current agent state
    pub async fn get_agent_state(&self, agent: &Agent) -> AgentState {
        agent.state().await
    }
}

impl EngineBinding for WasmBinding {
    fn create_agent(&self, config_path: &str) -> Result<Agent> {
        let config = load_agent_config(config_path)?;
        let agent = Agent::new(config);
        
        // Register the agent
        self.register_agent(agent.id(), agent.clone_for_binding());
        
        Ok(agent)
    }
    
    fn update_agent(&self, agent: &Agent, context_json: &str) -> Result<()> {
        let context = self.parse_wasm_context(context_json)?;
        
        // Get a new copy of the agent from the registry
        let agent_id = agent.id();
        let agents = self.agents.lock().unwrap();
        if let Some(stored_agent) = agents.get(&agent_id.to_string()) {
            // Use a cloned reference of the stored agent
            let agent_ref = stored_agent.clone_for_binding();
            drop(agents); // Release the lock
            
            // Create a runtime for the WASM context
            let runtime = tokio::runtime::Runtime::new().map_err(|e| {
                OxydeError::BindingError(format!("Failed to create Tokio runtime: {}", e))
            })?;
            
            runtime.block_on(async {
                agent_ref.update_context(context).await;
            });
        }
        
        Ok(())
    }
    
    fn process_input(&self, agent: &Agent, input: &str) -> Result<String> {
        // Process input asynchronously, but block on result for WASM
        let runtime = tokio::runtime::Runtime::new().map_err(|e| {
            OxydeError::BindingError(format!("Failed to create Tokio runtime: {}", e))
        })?;
        
        runtime.block_on(async {
            agent.process_input(input).await
        })
    }
    
    fn name(&self) -> &'static str {
        "wasm"
    }
}

// WASM exports
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct OxydeWasm {
    binding: WasmBinding,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl OxydeWasm {
    /// Create a new Oxyde WASM instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            binding: WasmBinding::new(),
        }
    }
    
    /// Initialize the Oxyde SDK
    #[wasm_bindgen]
    pub fn init() -> bool {
        // Set up logging for WASM
        console_error_panic_hook::set_once();
        true
    }
    
    /// Create a new agent from a configuration file
    #[wasm_bindgen]
    pub fn create_agent(&self, config_path: &str) -> Result<String, JsError> {
        match self.binding.create_agent(config_path) {
            Ok(agent) => Ok(agent.id().to_string()),
            Err(e) => Err(JsError::new(&e.to_string())),
        }
    }
    
    /// Update an agent with new context data
    #[wasm_bindgen]
    pub fn update_agent(&self, agent_id: &str, context_json: &str) -> Result<(), JsError> {
        match self.binding.get_agent(agent_id) {
            Ok(agent) => {
                match self.binding.update_agent(&agent, context_json) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(JsError::new(&e.to_string())),
                }
            },
            Err(e) => Err(JsError::new(&e.to_string())),
        }
    }
    
    /// Process input for an agent
    #[wasm_bindgen]
    pub fn process_input(&self, agent_id: &str, input: &str) -> Result<String, JsError> {
        match self.binding.get_agent(agent_id) {
            Ok(agent) => {
                match self.binding.process_input(&agent, input) {
                    Ok(response) => Ok(response),
                    Err(e) => Err(JsError::new(&e.to_string())),
                }
            },
            Err(e) => Err(JsError::new(&e.to_string())),
        }
    }
    
    /// Get agent state
    #[wasm_bindgen]
    pub fn get_agent_state(&self, agent_id: &str) -> Result<String, JsError> {
        match self.binding.get_agent(agent_id) {
            Ok(agent) => {
                // Create a runtime for the WASM context
                let runtime = match tokio::runtime::Runtime::new() {
                    Ok(rt) => rt,
                    Err(e) => return Err(JsError::new(&e.to_string())),
                };
                
                let state = runtime.block_on(async {
                    self.binding.get_agent_state(&agent).await
                });
                
                Ok(format!("{:?}", state))
            },
            Err(e) => Err(JsError::new(&e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wasm_binding_creation() {
        let binding = WasmBinding::new();
        assert_eq!(binding.name(), "wasm");
    }
    
    #[test]
    fn test_parse_wasm_context() {
        let binding = WasmBinding::new();
        let context_json = r#"{"gameState": {"level": 1, "score": 100}, "playerHealth": 80}"#;
        
        let context = binding.parse_wasm_context(context_json).unwrap();
        
        assert!(context.contains_key("gameState"));
        assert_eq!(context.get("playerHealth").unwrap().as_i64().unwrap(), 80);
    }
}
