//! Unreal Engine bindings for Oxyde SDK
//!
//! This module provides bindings for integrating Oxyde with Unreal Engine.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::ffi::CString;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "unreal")]
use ffi_support::FfiStr;

use crate::agent::{Agent, AgentContext};
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
    agents: Arc<Mutex<HashMap<String, Arc<Agent>>>>,
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
    pub fn get_agent(&self, id: &str) -> Result<Arc<Agent>> {
        let agents = self.agents.lock().map_err(|e| {
            OxydeError::BindingError(format!("Failed to lock agents mutex: {}", e))
        })?;
        agents.get(id)
            .cloned()
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
    pub fn register_agent(&self, id: Uuid, agent: Arc<Agent>) {
        match self.agents.lock() {
            Ok(mut agents) => {
                agents.insert(id.to_string(), agent);
            }
            Err(poisoned) => {
                log::warn!("Agents mutex was poisoned, recovering and continuing");
                let mut agents = poisoned.into_inner();
                agents.insert(id.to_string(), agent);
            }
        }
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

    /// Get agent emotion vector
    ///
    /// # Arguments
    ///
    /// * `agent` - Agent to get emotion vector for
    ///
    /// # Returns
    ///
    /// Emotion vector [joy, trust, fear, surprise, sadness, disgust, anger, anticipation] or an error
    pub fn get_agent_emotion_vector(&self, agent: &Agent) -> Result<[f32; 8]> {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| {
            OxydeError::BindingError(format!("Failed to create Tokio runtime: {}", e))
        })?;
        
        runtime.block_on(async {
            Ok(agent.emotion_vector().await)
        })
    }

}

impl EngineBinding for UnrealBinding {
    fn create_agent(&self, config_path: &str) -> Result<Arc<Agent>> {
        let config = load_agent_config(config_path)?;
        let agent = Arc::new(Agent::new(config));
        
        // Register the agent
        self.register_agent(agent.id(), agent.clone());
        
        Ok(agent)
    }

    fn create_agent_from_json(&self, json_config: &str) -> Result<Arc<Agent>> {
        let config = crate::oxyde_game::bindings::parse_agent_config_json(json_config)?;
        let agent = Arc::new(Agent::new(config));
        
        // Register the agent
        self.register_agent(agent.id(), agent.clone());
        
        Ok(agent)
    }
    
    fn update_agent(&self, agent: &Agent, context_json: &str) -> Result<()> {
        let context = self.parse_unreal_context(context_json)?;

        // Get a new copy of the agent from the registry
        let agent_id = agent.id();
        let agents = self.agents.lock().map_err(|e| {
            OxydeError::BindingError(format!("Failed to lock agents mutex: {}", e))
        })?;
        if let Some(stored_agent) = agents.get(&agent_id.to_string()) {
            // Use a cloned reference of the stored agent
            let agent_ref = stored_agent.clone();
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
            // Safe because we just initialized it above if it was None
            BINDING.as_ref().expect("Unreal binding initialization failed")
        }
    }
    
    /// Helper to convert string to raw CString pointer safely
    fn string_to_ptr(s: String) -> *mut c_char {
        CString::new(s)
            .unwrap_or_else(|_| CString::new("").unwrap())
            .into_raw()
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
                string_to_ptr(agent_id)
            },
            Err(_) => std::ptr::null_mut(),
        }
    }

    /// Create a new agent from a configuration JSON string
    #[no_mangle]
    pub extern "C" fn oxyde_unreal_create_agent_from_json(json_config: FfiStr) -> *mut c_char {
        let binding = get_binding();
        let json_config_str = json_config.into_string();
        
        match binding.create_agent_from_json(&json_config_str) {
            Ok(agent) => {
                let agent_id = agent.id().to_string();
                string_to_ptr(agent_id)
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
    pub extern "C" fn oxyde_unreal_process_input(agent_id: FfiStr, input: FfiStr) -> *mut c_char {
        let binding = get_binding();
        let agent_id_str = agent_id.into_string();
        let input_str = input.into_string();

        match binding.get_agent(&agent_id_str) {
            Ok(agent) => {
                // keep your current async/blocking logic; just convert the final String to char*
                let rt = tokio::runtime::Runtime::new().ok();
                if let Some(rt) = rt {
                    match rt.block_on(async { agent.process_input(&input_str).await }) {
                        Ok(response) => string_to_ptr(response),
                        Err(e) => string_to_ptr(format!("Error processing input: {}", e)),
                    }
                } else {
                    string_to_ptr("Error processing input".to_string())
                }
            }
            Err(_) => string_to_ptr("Agent not found".to_string()),
        }
    }

    #[no_mangle]
    pub extern "C" fn oxyde_unreal_get_agent_state(agent_id: FfiStr) -> *mut c_char {
        let binding = get_binding();
        let agent_id_str = agent_id.into_string();

        match binding.get_agent(&agent_id_str) {
            Ok(agent) => {
                let state_json = format!("{{\"id\":\"{}\",\"name\":\"{}\"}}", agent.id(), agent.name());
                string_to_ptr(state_json)
            }
            Err(_) => string_to_ptr("{}".to_string()),
        }
    }
    
    /// Get agent emotion vector
    #[no_mangle]
    pub extern "C" fn oxyde_unreal_get_emotion_vector(
        agent_id: FfiStr,
        out_joy: *mut f32,
        out_trust: *mut f32,
        out_fear: *mut f32,
        out_surprise: *mut f32,
        out_sadness: *mut f32,
        out_disgust: *mut f32,
        out_anger: *mut f32,
        out_anticipation: *mut f32
    ) -> bool {
        let binding = get_binding();
        let agent_id_str = agent_id.into_string();
        
        match binding.get_agent(&agent_id_str) {
            Ok(agent) => {
                match binding.get_agent_emotion_vector(&agent) {
                    Ok(emotion_vector) => {
                        unsafe {
                            if !out_joy.is_null() {
                                *out_joy = emotion_vector[0];
                            }
                            if !out_trust.is_null() {
                                *out_trust = emotion_vector[1];
                            }
                            if !out_fear.is_null() {
                                *out_fear = emotion_vector[2];
                            }
                            if !out_surprise.is_null() {
                                *out_surprise = emotion_vector[3];
                            }
                            if !out_sadness.is_null() {
                                *out_sadness = emotion_vector[4];
                            }
                            if !out_disgust.is_null() {
                                *out_disgust = emotion_vector[5];
                            }
                            if !out_anger.is_null() {
                                *out_anger = emotion_vector[6];
                            }
                            if !out_anticipation.is_null() {
                                *out_anticipation = emotion_vector[7];
                            }
                        }
                        true
                    },
                    Err(_) => false,
                }
            },
            Err(_) => false,
        }
    }

    // ==================== Memory System FFI ====================

    /// Add a memory to an agent's memory system
    #[no_mangle]
    pub extern "C" fn oxyde_unreal_add_memory(
        agent_id: FfiStr,
        category: FfiStr,
        content: FfiStr,
        importance: f64,
    ) -> bool {
        let binding = get_binding();
        let agent_id_str = agent_id.into_string();
        let category_str = category.into_string();
        let content_str = content.into_string();
        
        let memory_category = match crate::memory::MemoryCategory::from_str(&category_str) {
            Some(cat) => cat,
            None => return false,
        };
        
        match binding.get_agent(&agent_id_str) {
            Ok(agent) => {
                let runtime = match tokio::runtime::Runtime::new() {
                    Ok(rt) => rt,
                    Err(_) => return false,
                };
                runtime.block_on(async {
                    agent.add_memory(memory_category, &content_str, importance, None).await.is_ok()
                })
            },
            Err(_) => false,
        }
    }

    /// Add a memory with emotional context to an agent's memory system
    #[no_mangle]
    pub extern "C" fn oxyde_unreal_add_emotional_memory(
        agent_id: FfiStr,
        category: FfiStr,
        content: FfiStr,
        importance: f64,
        valence: f64,
        intensity: f64,
    ) -> bool {
        let binding = get_binding();
        let agent_id_str = agent_id.into_string();
        let category_str = category.into_string();
        let content_str = content.into_string();
        
        let memory_category = match crate::memory::MemoryCategory::from_str(&category_str) {
            Some(cat) => cat,
            None => return false,
        };
        
        match binding.get_agent(&agent_id_str) {
            Ok(agent) => {
                let runtime = match tokio::runtime::Runtime::new() {
                    Ok(rt) => rt,
                    Err(_) => return false,
                };
                runtime.block_on(async {
                    agent.add_emotional_memory(
                        memory_category, &content_str, importance, valence, intensity, None
                    ).await.is_ok()
                })
            },
            Err(_) => false,
        }
    }

    /// Get the number of memories stored by an agent
    #[no_mangle]
    pub extern "C" fn oxyde_unreal_get_memory_count(agent_id: FfiStr) -> u32 {
        let binding = get_binding();
        let agent_id_str = agent_id.into_string();
        
        match binding.get_agent(&agent_id_str) {
            Ok(agent) => {
                let runtime = match tokio::runtime::Runtime::new() {
                    Ok(rt) => rt,
                    Err(_) => return 0,
                };
                runtime.block_on(async {
                    agent.memory_count().await as u32
                })
            },
            Err(_) => 0,
        }
    }

    /// Clear all non-permanent memories from an agent
    #[no_mangle]
    pub extern "C" fn oxyde_unreal_clear_memories(agent_id: FfiStr) -> u32 {
        let binding = get_binding();
        let agent_id_str = agent_id.into_string();
        
        match binding.get_agent(&agent_id_str) {
            Ok(agent) => {
                let runtime = match tokio::runtime::Runtime::new() {
                    Ok(rt) => rt,
                    Err(_) => return 0,
                };
                runtime.block_on(async {
                    agent.clear_memories().await as u32
                })
            },
            Err(_) => 0,
        }
    }

    /// Retrieve memories by category as JSON array
    #[no_mangle]
    pub extern "C" fn oxyde_unreal_get_memories_by_category(
        agent_id: FfiStr,
        category: FfiStr,
    ) -> *mut c_char {
        let binding = get_binding();
        let agent_id_str = agent_id.into_string();
        let category_str = category.into_string();
        
        let memory_category = match crate::memory::MemoryCategory::from_str(&category_str) {
            Some(cat) => cat,
            None => return string_to_ptr("[]".to_string()),
        };
        
        match binding.get_agent(&agent_id_str) {
            Ok(agent) => {
                let runtime = match tokio::runtime::Runtime::new() {
                    Ok(rt) => rt,
                    Err(_) => return string_to_ptr("[]".to_string()),
                };
                let memories = runtime.block_on(async {
                    agent.get_memories_by_category(memory_category).await
                });
                let json = serde_json::to_string(&memories).unwrap_or_else(|_| "[]".to_string());
                string_to_ptr(json)
            },
            Err(_) => string_to_ptr("[]".to_string()),
        }
    }

    /// Retrieve memories relevant to a query as JSON array
    #[no_mangle]
    pub extern "C" fn oxyde_unreal_retrieve_relevant_memories(
        agent_id: FfiStr,
        query: FfiStr,
        limit: u32,
    ) -> *mut c_char {
        let binding = get_binding();
        let agent_id_str = agent_id.into_string();
        let query_str = query.into_string();
        
        match binding.get_agent(&agent_id_str) {
            Ok(agent) => {
                let runtime = match tokio::runtime::Runtime::new() {
                    Ok(rt) => rt,
                    Err(_) => return string_to_ptr("[]".to_string()),
                };
                let result = runtime.block_on(async {
                    agent.retrieve_relevant_memories(&query_str, limit as usize).await
                });
                let memories = result.unwrap_or_default();
                let json = serde_json::to_string(&memories).unwrap_or_else(|_| "[]".to_string());
                string_to_ptr(json)
            },
            Err(_) => string_to_ptr("[]".to_string()),
        }
    }

    /// Forget a specific memory by ID
    #[no_mangle]
    pub extern "C" fn oxyde_unreal_forget_memory(
        agent_id: FfiStr,
        memory_id: FfiStr,
    ) -> bool {
        let binding = get_binding();
        let agent_id_str = agent_id.into_string();
        let memory_id_str = memory_id.into_string();
        
        match binding.get_agent(&agent_id_str) {
            Ok(agent) => {
                let runtime = match tokio::runtime::Runtime::new() {
                    Ok(rt) => rt,
                    Err(_) => return false,
                };
                runtime.block_on(async {
                    agent.forget_memory(&memory_id_str).await.is_ok()
                })
            },
            Err(_) => false,
        }
    }

    /// Forget all memories of a specific category
    #[no_mangle]
    pub extern "C" fn oxyde_unreal_forget_memories_by_category(
        agent_id: FfiStr,
        category: FfiStr,
    ) -> u32 {
        let binding = get_binding();
        let agent_id_str = agent_id.into_string();
        let category_str = category.into_string();
        
        let memory_category = match crate::memory::MemoryCategory::from_str(&category_str) {
            Some(cat) => cat,
            None => return 0,
        };
        
        match binding.get_agent(&agent_id_str) {
            Ok(agent) => {
                let runtime = match tokio::runtime::Runtime::new() {
                    Ok(rt) => rt,
                    Err(_) => return 0,
                };
                runtime.block_on(async {
                    agent.forget_memories_by_category(memory_category).await as u32
                })
            },
            Err(_) => 0,
        }
    }

    // ==================== End Memory System FFI ====================

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
