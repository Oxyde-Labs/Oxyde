//! Unity bindings for Oxyde SDK
//!
//! This module provides bindings for integrating Oxyde with Unity game engine.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "unity")]
use ffi_support::{ByteBuffer, FfiStr};

use crate::agent::{Agent, AgentContext, AgentState};
use crate::config::AgentConfig;
use crate::oxyde_game::bindings::{EngineBinding, load_agent_config, parse_context_json};
use crate::{OxydeError, Result};

/// Unity-specific agent state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnityAgentState {
    /// Agent ID
    pub id: String,
    
    /// Agent name
    pub name: String,
    
    /// Current state
    pub state: String,
    
    /// Last response
    pub last_response: Option<String>,
    
    /// Available behaviors
    pub behaviors: Vec<String>,
}

impl From<&Agent> for UnityAgentState {
    fn from(agent: &Agent) -> Self {
        // This would be populated properly in a complete implementation
        UnityAgentState {
            id: agent.id().to_string(),
            name: agent.name().to_string(),
            state: format!("{:?}", AgentState::Idle), // Placeholder
            last_response: None,
            behaviors: Vec::new(),
        }
    }
}

/// Unity binding for Oxyde SDK
pub struct UnityBinding {
    /// Registry of created agents
    agents: Arc<Mutex<HashMap<String, Agent>>>,
}

impl UnityBinding {
    /// Create a new Unity binding
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
    
    /// Convert Unity context to Oxyde context
    ///
    /// # Arguments
    ///
    /// * `unity_context` - JSON string with Unity context data
    ///
    /// # Returns
    ///
    /// Oxyde agent context or an error
    pub fn parse_unity_context(&self, unity_context: &str) -> Result<AgentContext> {
        let context_map = parse_context_json(unity_context)?;
        
        let mut agent_context = AgentContext::new();
        for (key, value) in context_map {
            agent_context.insert(key, value);
        }
        
        Ok(agent_context)
    }
    
    /// Get agent state as JSON
    ///
    /// # Arguments
    ///
    /// * `agent` - Agent to get state for
    ///
    /// # Returns
    ///
    /// JSON string with agent state or an error
    pub fn get_agent_state_json(&self, agent: &Agent) -> Result<String> {
        let state = UnityAgentState::from(agent);
        serde_json::to_string(&state).map_err(|e| {
            OxydeError::BindingError(format!("Failed to serialize agent state: {}", e))
        })
    }
}

impl EngineBinding for UnityBinding {
    fn create_agent(&self, config_path: &str) -> Result<Agent> {
        let config = load_agent_config(config_path)?;
        let agent = Agent::new(config);
        
        // Register the agent
        self.register_agent(agent.id(), agent.clone_for_binding());
        
        Ok(agent)
    }
    
    fn update_agent(&self, agent: &Agent, context_json: &str) -> Result<()> {
        let context = self.parse_unity_context(context_json)?;
        
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
        "unity"
    }
}

// FFI exports for Unity
#[cfg(feature = "unity")]
pub mod ffi {
    use super::*;
    use std::ffi::CString;
    use std::os::raw::c_char;
    
    static mut BINDING: Option<UnityBinding> = None;
    
    fn get_binding() -> &'static UnityBinding {
        unsafe {
            if BINDING.is_none() {
                BINDING = Some(UnityBinding::new());
            }
            BINDING.as_ref().unwrap()
        }
    }
    
    /// Initialize the Oxyde SDK for Unity
    #[no_mangle]
    pub extern "C" fn oxyde_unity_init() -> bool {
        get_binding();
        true
    }
    
    /// Create a new agent from a configuration file
    #[no_mangle]
    pub extern "C" fn oxyde_unity_create_agent(config_path: FfiStr) -> *mut c_char {
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
    pub extern "C" fn oxyde_unity_update_agent(agent_id: FfiStr, context_json: FfiStr) -> bool {
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
    pub extern "C" fn oxyde_unity_process_input(agent_id: FfiStr, input: FfiStr) -> ByteBuffer {
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
    
    /// Get agent state
    #[no_mangle]
    pub extern "C" fn oxyde_unity_get_agent_state(agent_id: FfiStr) -> ByteBuffer {
        let binding = get_binding();
        let agent_id_str = agent_id.into_string();
        
        match binding.get_agent(&agent_id_str) {
            Ok(agent) => {
                match binding.get_agent_state_json(&agent) {
                    Ok(state_json) => ByteBuffer::from_str(&state_json),
                    Err(_) => ByteBuffer::from_str("{}"),
                }
            },
            Err(_) => ByteBuffer::from_str("{}"),
        }
    }
    
    /// Free a string allocated by the Oxyde SDK
    #[no_mangle]
    pub extern "C" fn oxyde_unity_free_string(s: *mut c_char) {
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
    fn test_unity_binding_creation() {
        let binding = UnityBinding::new();
        assert_eq!(binding.name(), "unity");
    }
    
    #[test]
    fn test_parse_unity_context() {
        let binding = UnityBinding::new();
        let context_json = r#"{"player_x": 10.5, "player_y": 20.5, "player_name": "Hero"}"#;
        
        let context = binding.parse_unity_context(context_json).unwrap();
        
        assert_eq!(context.get("player_x").unwrap().as_f64().unwrap(), 10.5);
        assert_eq!(context.get("player_y").unwrap().as_f64().unwrap(), 20.5);
        assert_eq!(context.get("player_name").unwrap().as_str().unwrap(), "Hero");
    }
}
