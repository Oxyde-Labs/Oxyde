//! Unreal Engine bindings for Oxyde SDK
//!
//! This module provides bindings for integrating Oxyde with Unreal Engine.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "unreal")]
use ffi_support::{ByteBuffer, FfiStr};

use crate::agent::{Agent, AgentContext};
use crate::config::AgentConfig;
use crate::oxyde_game::bindings::{EngineBinding, load_agent_config, parse_context_json};
use crate::{OxydeError, Result};

/// Unreal-specific agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnrealAgentConfig {
    /// Config file path
    pub config_path: String,
    
    /// Additional Unreal-specific parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Unreal Engine binding for Oxyde SDK
pub struct UnrealBinding {
    /// Registry of created agents
    agents: Arc<Mutex<HashMap<String, Agent>>>,
}

impl UnrealBinding {
    /// Create a new Unreal Engine binding
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
    
    /// Parse Unreal Engine context
    ///
    /// # Arguments
    ///
    /// * `context_json` - JSON string with Unreal context data
    ///
    /// # Returns
    ///
    /// Oxyde agent context or an error
    pub fn parse_unreal_context(&self, context_json: &str) -> Result<AgentContext> {
        let context_map = parse_context_json(context_json)?;
        
        let mut agent_context = AgentContext::new();
        for (key, value) in context_map {
            agent_context.insert(key, value);
        }
        
        Ok(agent_context)
    }
}

impl EngineBinding for UnrealBinding {
    fn create_agent(&self, config_path: &str) -> Result<Agent> {
        let config = load_agent_config(config_path)?;
        let agent = Agent::new(config);
        
        // Register the agent
        self.register_agent(agent.id(), agent.clone_for_binding());
        
        Ok(agent)
    }
    
    fn update_agent(&self, agent: &Agent, context_json: &str) -> Result<()> {
        let context = self.parse_unreal_context(context_json)?;
        
        // Get a new copy of the agent from the registry
        let agent_id = agent.id();
        let agents = self.agents.lock().unwrap();
        if let Some(stored_agent) = agents.get(&agent_id.to_string()) {
            // Use a cloned reference of the stored agent
            let agent_ref = stored_agent.clone_for_binding();
            drop(agents); // Release the lock
            
            tokio::spawn(async move {
                agent_ref.update_context(context).await;
            });
        }
        
        Ok(())
    }
    
    fn process_input(&self, agent: &Agent, input: &str) -> Result<String> {
        // Process input asynchronously, but block on result for FFI
        let runtime = tokio::runtime::Runtime::new().map_err(|e| {
            OxydeError::BindingError(format!("Failed to create Tokio runtime: {}", e))
        })?;
        
        runtime.block_on(async {
            agent.process_input(input).await
        })
    }
    
    fn name(&self) -> &'static str {
        "unreal"
    }
}

// FFI exports for Unreal Engine
#[cfg(feature = "unreal")]
pub mod ffi {
    use super::*;
    use std::ffi::CString;
    use std::os::raw::c_char;
    
    static mut BINDING: Option<UnrealBinding> = None;
    
    fn get_binding() -> &'static UnrealBinding {
        unsafe {
            if BINDING.is_none() {
                BINDING = Some(UnrealBinding::new());
            }
            BINDING.as_ref().unwrap()
        }
    }
    
    /// Initialize the Oxyde SDK for Unreal Engine
    #[no_mangle]
    pub extern "C" fn oxyde_unreal_init() -> bool {
        get_binding();
        true
    }
    
    /// Create a new agent from a configuration file
    #[no_mangle]
    pub extern "C" fn oxyde_unreal_create_agent(config_path: FfiStr) -> *mut c_char {
        let binding = get_binding();
        let config_path_str = config_path.into_string();
        
        match binding.create_agent(&config_path_str) {
            Ok(agent) => {
                let agent_id = agent.id().to_string();
                CString::new(agent_id).unwrap().into_raw()
            },
            Err(_) => std::ptr::null_mut(),
        }
    }
    
    /// Update an agent with new context data
    #[no_mangle]
    pub extern "C" fn oxyde_unreal_update_agent(agent_id: FfiStr, context_json: FfiStr) -> bool {
        let binding = get_binding();
        let agent_id_str = agent_id.into_string();
        let context_json_str = context_json.into_string();
        
        match binding.get_agent(&agent_id_str) {
            Ok(agent) => {
                binding.update_agent(&agent, &context_json_str).is_ok()
            },
            Err(_) => false,
        }
    }
    
    /// Process input for an agent
    #[no_mangle]
    pub extern "C" fn oxyde_unreal_process_input(agent_id: FfiStr, input: FfiStr) -> ByteBuffer {
        let binding = get_binding();
        let agent_id_str = agent_id.into_string();
        let input_str = input.into_string();
        
        match binding.get_agent(&agent_id_str) {
            Ok(agent) => {
                match binding.process_input(&agent, &input_str) {
                    Ok(response) => ByteBuffer::from_str(&response),
                    Err(_) => ByteBuffer::from_str("Error processing input"),
                }
            },
            Err(_) => ByteBuffer::from_str("Agent not found"),
        }
    }
    
    /// Free a string allocated by the Oxyde SDK
    #[no_mangle]
    pub extern "C" fn oxyde_unreal_free_string(s: *mut c_char) {
        if !s.is_null() {
            unsafe {
                let _ = CString::from_raw(s);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_unreal_binding_creation() {
        let binding = UnrealBinding::new();
        assert_eq!(binding.name(), "unreal");
    }
    
    #[test]
    fn test_parse_unreal_context() {
        let binding = UnrealBinding::new();
        let context_json = r#"{"player_location": {"x": 100, "y": 200, "z": 50}, "interaction_distance": 3.5}"#;
        
        let context = binding.parse_unreal_context(context_json).unwrap();
        
        assert!(context.contains_key("player_location"));
        assert_eq!(context.get("interaction_distance").unwrap().as_f64().unwrap(), 3.5);
    }
}
