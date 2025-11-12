//! Configuration for Oxyde agents and systems
//!
//! This module provides configuration structures for agents, inference engines,
//! memory systems, and behaviors.

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{OxydeError, Result};

/// Configuration for an agent's personality and behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPersonality {
    /// Agent name
    pub name: String,
    
    /// Agent role (e.g., "Shopkeeper", "Guard", "Villager")
    pub role: String,
    
    /// Agent backstory and personality traits
    pub backstory: Vec<String>,
    
    /// Agent knowledge base (facts it knows about the world)
    pub knowledge: Vec<String>,
}

/// Vector embedding model type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EmbeddingModelType {
    /// Use mini-bert model
    MiniBert,
    /// Use distilled BERT
    DistilBert,
    /// Use a custom model
    Custom,
}

impl Default for EmbeddingModelType {
    fn default() -> Self {
        Self::MiniBert
    }
}

/// Configuration for the memory system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Maximum number of memories to store
    #[serde(default = "default_memory_capacity")]
    pub capacity: usize,
    
    /// Whether to persist memories to disk
    #[serde(default)]
    pub persistence: bool,
    
    /// Time-based decay rate for memories (0.0 - 1.0)
    #[serde(default = "default_memory_decay")]
    pub decay_rate: f64,
    
    /// Importance threshold for retrieving memories
    #[serde(default = "default_memory_threshold")]
    pub importance_threshold: f64,
    
    /// Number of memories to keep in short-term memory
    #[serde(default = "default_short_term_capacity")]
    pub short_term_capacity: usize,
    
    /// Whether to use vector embeddings for memory retrieval
    #[serde(default)]
    pub use_embeddings: bool,
    
    /// Vector embedding model type
    #[serde(default)]
    pub embedding_model: EmbeddingModelType,
    
    /// Path to custom embedding model (if using custom model)
    pub custom_model_path: Option<String>,
    
    /// Dimension of the embeddings
    #[serde(default = "default_embedding_dim")]
    pub embedding_dimension: usize,
    
    /// Memory categories to prioritize
    #[serde(default)]
    pub priority_categories: Vec<String>,
}

fn default_memory_capacity() -> usize {
    100
}

fn default_memory_decay() -> f64 {
    0.05
}

fn default_memory_threshold() -> f64 {
    0.2
}

fn default_short_term_capacity() -> usize {
    10
}

fn default_embedding_dim() -> usize {
    384 // Standard dimension for mini BERT models
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            capacity: default_memory_capacity(),
            persistence: false,
            decay_rate: default_memory_decay(),
            importance_threshold: default_memory_threshold(),
            short_term_capacity: default_short_term_capacity(),
            use_embeddings: false,
            embedding_model: EmbeddingModelType::default(),
            custom_model_path: None,
            embedding_dimension: default_embedding_dim(),
            priority_categories: Vec::new(),
        }
    }
}

impl MemoryConfig {
    /// Validate the memory configuration
    ///
    /// # Returns
    ///
    /// Ok if the configuration is valid, Err with a descriptive message otherwise
    pub fn validate(&self) -> Result<()> {
        // Validate capacity
        if self.capacity == 0 {
            return Err(OxydeError::ConfigurationError(
                "Memory capacity must be greater than 0".to_string()
            ));
        }

        // Validate short-term capacity
        if self.short_term_capacity == 0 {
            return Err(OxydeError::ConfigurationError(
                "Short-term memory capacity must be greater than 0".to_string()
            ));
        }

        if self.short_term_capacity > self.capacity {
            return Err(OxydeError::ConfigurationError(
                format!(
                    "Short-term capacity ({}) cannot exceed total capacity ({})",
                    self.short_term_capacity, self.capacity
                )
            ));
        }

        // Validate decay rate (0.0 - 1.0)
        if !(0.0..=1.0).contains(&self.decay_rate) {
            return Err(OxydeError::ConfigurationError(
                format!(
                    "Decay rate must be between 0.0 and 1.0, got {}",
                    self.decay_rate
                )
            ));
        }

        // Validate importance threshold (0.0 - 1.0)
        if !(0.0..=1.0).contains(&self.importance_threshold) {
            return Err(OxydeError::ConfigurationError(
                format!(
                    "Importance threshold must be between 0.0 and 1.0, got {}",
                    self.importance_threshold
                )
            ));
        }

        // Validate embedding dimension
        if self.use_embeddings && self.embedding_dimension == 0 {
            return Err(OxydeError::ConfigurationError(
                "Embedding dimension must be greater than 0 when embeddings are enabled".to_string()
            ));
        }

        // Validate custom model path if using custom embedding model
        if self.embedding_model == EmbeddingModelType::Custom {
            if self.custom_model_path.is_none() {
                return Err(OxydeError::ConfigurationError(
                    "Custom model path must be provided when using custom embedding model".to_string()
                ));
            }

            if let Some(ref path) = self.custom_model_path {
                if path.is_empty() {
                    return Err(OxydeError::ConfigurationError(
                        "Custom model path cannot be empty".to_string()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Configuration for the inference engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceConfig {
    /// Model to use for inference
    #[serde(default = "default_model")]
    pub model: String,
    
    /// Whether to use local models or cloud APIs
    #[serde(default)]
    pub use_local: bool,
    
    /// Path to the local model file (if use_local is true)
    pub local_model_path: Option<String>,
    
    /// Cloud API endpoint (if use_local is false)
    pub api_endpoint: Option<String>,
    
    /// API key for cloud service
    pub api_key: Option<String>,
    
    /// Inference temperature (0.0 - 1.0)
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    
    /// Maximum number of tokens to generate
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
    
    /// Timeout for inference requests in milliseconds
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
    
    /// Fallback API to use if primary fails
    pub fallback_api: Option<String>,
}

fn default_model() -> String {
    "llama2-7b".to_string()
}

fn default_temperature() -> f32 {
    0.7
}

fn default_max_tokens() -> usize {
    256
}

fn default_timeout() -> u64 {
    5000
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            model: default_model(),
            use_local: false,
            local_model_path: None,
            api_endpoint: Some("https://api.openai.com/v1/chat/completions".to_string()),
            api_key: None,
            temperature: default_temperature(),
            max_tokens: default_max_tokens(),
            timeout_ms: default_timeout(),
            fallback_api: None,
        }
    }
}

impl InferenceConfig {
    /// Validate the inference configuration
    ///
    /// # Returns
    ///
    /// Ok if the configuration is valid, Err with a descriptive message otherwise
    pub fn validate(&self) -> Result<()> {
        // Validate temperature (0.0 - 2.0, though typically 0.0 - 1.0)
        if !(0.0..=2.0).contains(&self.temperature) {
            return Err(OxydeError::ConfigurationError(
                format!(
                    "Temperature must be between 0.0 and 2.0, got {}",
                    self.temperature
                )
            ));
        }

        // Validate max tokens
        if self.max_tokens == 0 {
            return Err(OxydeError::ConfigurationError(
                "Max tokens must be greater than 0".to_string()
            ));
        }

        if self.max_tokens > 100000 {
            return Err(OxydeError::ConfigurationError(
                format!(
                    "Max tokens ({}) exceeds reasonable limit (100000)",
                    self.max_tokens
                )
            ));
        }

        // Validate timeout
        if self.timeout_ms == 0 {
            return Err(OxydeError::ConfigurationError(
                "Timeout must be greater than 0ms".to_string()
            ));
        }

        if self.timeout_ms > 300000 {
            return Err(OxydeError::ConfigurationError(
                format!(
                    "Timeout ({}ms) exceeds maximum allowed (300000ms / 5 minutes)",
                    self.timeout_ms
                )
            ));
        }

        // Validate local model configuration
        if self.use_local {
            if self.local_model_path.is_none() {
                return Err(OxydeError::ConfigurationError(
                    "Local model path must be provided when use_local is true".to_string()
                ));
            }

            if let Some(ref path) = self.local_model_path {
                if path.is_empty() {
                    return Err(OxydeError::ConfigurationError(
                        "Local model path cannot be empty".to_string()
                    ));
                }
            }
        }

        // Validate cloud API configuration
        if !self.use_local {
            if self.api_endpoint.is_none() {
                return Err(OxydeError::ConfigurationError(
                    "API endpoint must be provided when using cloud inference".to_string()
                ));
            }

            if let Some(ref endpoint) = self.api_endpoint {
                if endpoint.is_empty() {
                    return Err(OxydeError::ConfigurationError(
                        "API endpoint cannot be empty".to_string()
                    ));
                }

                // Basic URL validation
                if !endpoint.starts_with("http://") && !endpoint.starts_with("https://") {
                    return Err(OxydeError::ConfigurationError(
                        format!(
                            "API endpoint must be a valid HTTP(S) URL, got: {}",
                            endpoint
                        )
                    ));
                }
            }
        }

        // Validate model name is not empty
        if self.model.is_empty() {
            return Err(OxydeError::ConfigurationError(
                "Model name cannot be empty".to_string()
            ));
        }

        Ok(())
    }
}

/// Configuration for a behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorConfig {
    /// Trigger condition for the behavior
    pub trigger: String,
    
    /// Cooldown period in seconds before the behavior can trigger again
    #[serde(default)]
    pub cooldown: u64,
    
    /// Priority of the behavior (higher means more important)
    #[serde(default)]
    pub priority: u32,
    
    /// Additional behavior-specific configuration
    #[serde(flatten)]
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Complete agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Agent personality configuration
    pub agent: AgentPersonality,
    
    /// Memory system configuration
    #[serde(default)]
    pub memory: MemoryConfig,
    
    /// Inference engine configuration
    #[serde(default)]
    pub inference: InferenceConfig,
    
    /// Behavior configurations
    #[serde(default)]
    pub behavior: HashMap<String, BehaviorConfig>,
}

impl AgentConfig {
    /// Validate the agent configuration
    ///
    /// # Returns
    ///
    /// Ok if the configuration is valid, Err with a descriptive message otherwise
    pub fn validate(&self) -> Result<()> {
        // Validate agent personality
        if self.agent.name.is_empty() {
            return Err(OxydeError::ConfigurationError(
                "Agent name cannot be empty".to_string()
            ));
        }

        if self.agent.role.is_empty() {
            return Err(OxydeError::ConfigurationError(
                "Agent role cannot be empty".to_string()
            ));
        }

        // Validate memory configuration
        self.memory.validate()?;

        // Validate inference configuration
        self.inference.validate()?;

        // Validate behavior configurations
        for (name, behavior_config) in &self.behavior {
            if name.is_empty() {
                return Err(OxydeError::ConfigurationError(
                    "Behavior name cannot be empty".to_string()
                ));
            }

            if behavior_config.trigger.is_empty() {
                return Err(OxydeError::ConfigurationError(
                    format!("Behavior '{}' must have a non-empty trigger", name)
                ));
            }
        }

        Ok(())
    }

    /// Load an agent configuration from a file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file (JSON or YAML)
    ///
    /// # Returns
    ///
    /// The loaded AgentConfig or an error
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path.as_ref()).map_err(|e| {
            OxydeError::ConfigurationError(format!("Failed to open config file: {}", e))
        })?;

        let reader = BufReader::new(file);

        let extension = path.as_ref().extension().and_then(|ext| ext.to_str());

        let config: AgentConfig = match extension {
            Some("json") => {
                serde_json::from_reader(reader).map_err(|e| {
                    OxydeError::ConfigurationError(format!("Failed to parse JSON config: {}", e))
                })?
            },
            Some("yaml") | Some("yml") => {
                serde_yaml::from_reader(reader).map_err(|e| {
                    OxydeError::ConfigurationError(format!("Failed to parse YAML config: {}", e))
                })?
            },
            _ => {
                return Err(OxydeError::ConfigurationError(
                    "Unknown config file format. Expected .json, .yaml, or .yml".to_string()
                ));
            }
        };

        // Validate the loaded configuration
        config.validate()?;

        Ok(config)
    }
    
    /// Save the agent configuration to a file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to save the configuration file (JSON or YAML)
    ///
    /// # Returns
    ///
    /// Success or an error
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path.as_ref()).map_err(|e| {
            OxydeError::ConfigurationError(format!("Failed to create config file: {}", e))
        })?;
        
        let extension = path.as_ref().extension().and_then(|ext| ext.to_str());
        
        match extension {
            Some("json") => {
                serde_json::to_writer_pretty(file, self).map_err(|e| {
                    OxydeError::ConfigurationError(format!("Failed to write JSON config: {}", e))
                })
            },
            Some("yaml") | Some("yml") => {
                serde_yaml::to_writer(file, self).map_err(|e| {
                    OxydeError::ConfigurationError(format!("Failed to write YAML config: {}", e))
                })
            },
            _ => {
                Err(OxydeError::ConfigurationError(
                    "Unknown config file format. Expected .json, .yaml, or .yml".to_string()
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_configs() {
        let memory_config = MemoryConfig::default();
        assert_eq!(memory_config.capacity, 100);
        assert_eq!(memory_config.persistence, false);
        assert_eq!(memory_config.decay_rate, 0.05);
        
        let inference_config = InferenceConfig::default();
        assert_eq!(inference_config.model, "llama2-7b");
        assert_eq!(inference_config.temperature, 0.7);
        assert_eq!(inference_config.max_tokens, 256);
    }
    
    #[test]
    fn test_serialization() {
        let config = AgentConfig {
            agent: AgentPersonality {
                name: "Test Agent".to_string(),
                role: "Tester".to_string(),
                backstory: vec!["A test agent".to_string()],
                knowledge: vec!["Testing knowledge".to_string()],
            },
            memory: MemoryConfig::default(),
            inference: InferenceConfig::default(),
            behavior: HashMap::new(),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AgentConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.agent.name, "Test Agent");
        assert_eq!(deserialized.agent.role, "Tester");
    }

    #[test]
    fn test_memory_config_validation_success() {
        let config = MemoryConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_memory_config_validation_zero_capacity() {
        let mut config = MemoryConfig::default();
        config.capacity = 0;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("capacity must be greater than 0"));
    }

    #[test]
    fn test_memory_config_validation_short_term_exceeds_capacity() {
        let mut config = MemoryConfig::default();
        config.capacity = 50;
        config.short_term_capacity = 100;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot exceed total capacity"));
    }

    #[test]
    fn test_memory_config_validation_invalid_decay_rate() {
        let mut config = MemoryConfig::default();
        config.decay_rate = 1.5;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Decay rate must be between 0.0 and 1.0"));
    }

    #[test]
    fn test_memory_config_validation_invalid_importance_threshold() {
        let mut config = MemoryConfig::default();
        config.importance_threshold = -0.1;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Importance threshold must be between 0.0 and 1.0"));
    }

    #[test]
    fn test_memory_config_validation_custom_model_without_path() {
        let mut config = MemoryConfig::default();
        config.embedding_model = EmbeddingModelType::Custom;
        config.custom_model_path = None;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Custom model path must be provided"));
    }

    #[test]
    fn test_inference_config_validation_success() {
        let config = InferenceConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_inference_config_validation_invalid_temperature() {
        let mut config = InferenceConfig::default();
        config.temperature = 3.0;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Temperature must be between 0.0 and 2.0"));
    }

    #[test]
    fn test_inference_config_validation_zero_max_tokens() {
        let mut config = InferenceConfig::default();
        config.max_tokens = 0;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Max tokens must be greater than 0"));
    }

    #[test]
    fn test_inference_config_validation_excessive_max_tokens() {
        let mut config = InferenceConfig::default();
        config.max_tokens = 200000;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exceeds reasonable limit"));
    }

    #[test]
    fn test_inference_config_validation_zero_timeout() {
        let mut config = InferenceConfig::default();
        config.timeout_ms = 0;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Timeout must be greater than 0ms"));
    }

    #[test]
    fn test_inference_config_validation_local_without_path() {
        let mut config = InferenceConfig::default();
        config.use_local = true;
        config.local_model_path = None;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Local model path must be provided"));
    }

    #[test]
    fn test_inference_config_validation_cloud_without_endpoint() {
        let mut config = InferenceConfig::default();
        config.use_local = false;
        config.api_endpoint = None;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("API endpoint must be provided"));
    }

    #[test]
    fn test_inference_config_validation_invalid_url() {
        let mut config = InferenceConfig::default();
        config.api_endpoint = Some("not-a-valid-url".to_string());

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must be a valid HTTP(S) URL"));
    }

    #[test]
    fn test_agent_config_validation_success() {
        let config = AgentConfig {
            agent: AgentPersonality {
                name: "Test".to_string(),
                role: "Tester".to_string(),
                backstory: vec![],
                knowledge: vec![],
            },
            memory: MemoryConfig::default(),
            inference: InferenceConfig::default(),
            behavior: HashMap::new(),
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_agent_config_validation_empty_name() {
        let config = AgentConfig {
            agent: AgentPersonality {
                name: "".to_string(),
                role: "Tester".to_string(),
                backstory: vec![],
                knowledge: vec![],
            },
            memory: MemoryConfig::default(),
            inference: InferenceConfig::default(),
            behavior: HashMap::new(),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Agent name cannot be empty"));
    }

    #[test]
    fn test_agent_config_validation_empty_role() {
        let config = AgentConfig {
            agent: AgentPersonality {
                name: "Test".to_string(),
                role: "".to_string(),
                backstory: vec![],
                knowledge: vec![],
            },
            memory: MemoryConfig::default(),
            inference: InferenceConfig::default(),
            behavior: HashMap::new(),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Agent role cannot be empty"));
    }

    #[test]
    fn test_agent_config_validation_cascades_to_memory() {
        let config = AgentConfig {
            agent: AgentPersonality {
                name: "Test".to_string(),
                role: "Tester".to_string(),
                backstory: vec![],
                knowledge: vec![],
            },
            memory: MemoryConfig {
                capacity: 0,  // Invalid
                ..Default::default()
            },
            inference: InferenceConfig::default(),
            behavior: HashMap::new(),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("capacity"));
    }

    #[test]
    fn test_agent_config_validation_cascades_to_inference() {
        let config = AgentConfig {
            agent: AgentPersonality {
                name: "Test".to_string(),
                role: "Tester".to_string(),
                backstory: vec![],
                knowledge: vec![],
            },
            memory: MemoryConfig::default(),
            inference: InferenceConfig {
                temperature: 5.0,  // Invalid
                ..Default::default()
            },
            behavior: HashMap::new(),
        };

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Temperature"));
    }
}
