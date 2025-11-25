//! Configuration for Oxyde agents and systems
//!
//! This module provides configuration structures for agents, inference engines,
//! memory systems, and behaviors.

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use toml;

use serde::{Deserialize, Serialize};

use crate::{audio::TTSConfig, OxydeError, PromptConfig, Result};

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

    ///Text to Speech Configurations
    #[serde(default)]
    pub tts: Option<TTSConfig>,

    /// Prompt Configurations
    #[serde(default)]
    pub prompts: Option<PromptConfig>,
}

impl AgentConfig {
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

        let mut config: Self = match extension {
            Some("json") => serde_json::from_reader(reader).map_err(|e| {
                OxydeError::ConfigurationError(format!("Failed to parse JSON config: {}", e))
            })?,
            Some("yaml") | Some("yml") => serde_yaml::from_reader(reader).map_err(|e| {
                OxydeError::ConfigurationError(format!("Failed to parse YAML config: {}", e))
            })?,
            Some("toml") => {
                let content = std::io::read_to_string(reader).map_err(|e| {
                    OxydeError::ConfigurationError(format!("Failed to read TOML config: {}", e))
                })?;
                toml::from_str(&content).map_err(|e| {
                    OxydeError::ConfigurationError(format!("Failed to parse TOML config: {}", e))
                })?
            },
            _ => {
                return Err(OxydeError::ConfigurationError(
                    "Unknown config file format. Expected .json, .toml, .yaml, or .yml".to_string(),
                ))
            }
        };

        // If no prompts are embedded, try to load from prompts.toml
        if config.prompts.is_none() {
            let prompts_path = path
                .as_ref()
                .parent()
                .map(|p| p.join("prompts.toml"))
                .unwrap_or_else(|| PathBuf::from("prompts.toml"));
        // config.prompts = Some(PromptConfig::from_file_or_default(prompts_path)?);
            if prompts_path.exists() {
                config.prompts = Some(PromptConfig::from_file(prompts_path)?);
            }
        }

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
            Some("json") => serde_json::to_writer_pretty(file, self).map_err(|e| {
                OxydeError::ConfigurationError(format!("Failed to write JSON config: {}", e))
            }),
            Some("yaml") | Some("yml") => serde_yaml::to_writer(file, self).map_err(|e| {
                OxydeError::ConfigurationError(format!("Failed to write YAML config: {}", e))
            }),
            Some("toml") => {
                let content = std::fs::read_to_string(path.as_ref()).map_err(|e| {
                    OxydeError::ConfigurationError(format!("Failed to read TOML config: {}", e))
                })?;
                toml::to_string(self).map_err(|e| {
                    OxydeError::ConfigurationError(format!("Failed to serialize to TOML: {}", e))
                }).and_then(|content| {
                    std::fs::write(path.as_ref(), content).map_err(|e| {
                        OxydeError::ConfigurationError(format!("Failed to write TOML config: {}", e))
                    })
                })
            }
            _ => Err(OxydeError::ConfigurationError(
                "Unknown config file format. Expected .json, .yaml, or .yml".to_string(),
            )),
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
            tts: None,
            prompts: None,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AgentConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.agent.name, "Test Agent");
        assert_eq!(deserialized.agent.role, "Tester");
    }
}
