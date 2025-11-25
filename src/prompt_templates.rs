//! Prompt template system for the Oxyde SDK
//!
//! This module provides a flexible template system for NPC dialogue, memory formatting,
//! and goal-driven behavior prompts. Templates can be loaded from configuration files
//! and customized per NPC role or instance.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{OxydeError, Result};

/// Template for NPC system prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPromptTemplate {
    /// Base personality description
    pub base_description: String,

    /// Behavior guidelines
    pub behavior_guidelines: Vec<String>,

    /// Response constraints (e.g., length, style)
    pub response_constraints: String,

    /// Context inclusion template
    pub context_template: String,
}

/// Template for emotional state modifiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalPromptTemplate {
    /// Emotional state descriptions
    pub emotional_states: HashMap<String, String>,

    /// Response style descriptions
    pub response_styles: HashMap<String, String>,

    /// Emotional modifier template
    pub modifier_template: String,
}

/// Template for goal-driven prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalPromptTemplate {
    /// Goal context template
    pub goal_context_template: String,

    /// Goal priority descriptions
    pub priority_descriptions: HashMap<String, String>,

    /// Goal type descriptions
    pub goal_type_descriptions: HashMap<String, String>,

    /// Motivation template
    pub motivation_template: String,
}

/// Template for memory formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPromptTemplate {
    /// Memory context header
    pub context_header: String,

    /// Single memory format
    pub memory_format: String,

    /// Memory category labels
    pub category_labels: HashMap<String, String>,

    /// Importance indicators
    pub importance_indicators: HashMap<String, String>,
}

/// Complete prompt configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptConfig {
    /// Version of the prompt configuration
    #[serde(default = "default_version")]
    pub version: String,

    /// System prompt templates by role
    pub system_prompts: HashMap<String, SystemPromptTemplate>,

    /// Default system prompt template
    pub default_system_prompt: SystemPromptTemplate,

    /// Emotional prompt templates
    pub emotional_prompts: EmotionalPromptTemplate,

    /// Goal-driven prompt templates
    pub goal_prompts: GoalPromptTemplate,

    /// Memory formatting templates
    pub memory_prompts: MemoryPromptTemplate,

    /// Greeting templates by role
    #[serde(default)]
    pub greetings: HashMap<String, Vec<String>>,

    /// Farewell templates by role
    #[serde(default)]
    pub farewells: HashMap<String, Vec<String>>,
}

fn default_version() -> String {
    "1.0".to_string()
}

impl PromptConfig {
    /// Load the bundled default configuration
    pub fn from_bundled_default() -> Result<Self> {
        // Embed the default prompts.toml at compile time
        let default_toml = include_str!("./config/defaults.toml");
        toml::from_str(default_toml).map_err(|e| {
            OxydeError::ConfigurationError(format!(
                "Failed to parse bundled default prompts: {}",
                e
            ))
        })
    }

    /// Load from file, with fallback to bundled defaults
    pub fn from_file_or_default<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::from_file(path.as_ref()).or_else(|_| {
            log::warn!(
                "Could not load prompts from {:?}, using bundled defaults",
                path.as_ref()
            );
            Self::from_bundled_default()
        })
    }
    /// Load prompt configuration from a file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file (TOML, JSON, or YAML)
    ///
    /// # Returns
    ///
    /// The loaded PromptConfig or an error
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref()).map_err(|e| {
            OxydeError::ConfigurationError(format!("Failed to read prompts file: {}", e))
        })?;

        let extension = path.as_ref().extension().and_then(|ext| ext.to_str());

        match extension {
            Some("toml") => toml::from_str(&content).map_err(|e| {
                OxydeError::ConfigurationError(format!("Failed to parse TOML prompts: {}", e))
            }),
            Some("json") => serde_json::from_str(&content).map_err(|e| {
                OxydeError::ConfigurationError(format!("Failed to parse JSON prompts: {}", e))
            }),
            Some("yaml") | Some("yml") => serde_yaml::from_str(&content).map_err(|e| {
                OxydeError::ConfigurationError(format!("Failed to parse YAML prompts: {}", e))
            }),
            _ => Err(OxydeError::ConfigurationError(
                "Unknown prompts file format. Expected .toml, .json, .yaml, or .yml".to_string(),
            )),
        }
    }

    /// Save the prompt configuration to a file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to save the configuration file
    ///
    /// # Returns
    ///
    /// Success or an error
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let extension = path.as_ref().extension().and_then(|ext| ext.to_str());

        let content = match extension {
            Some("toml") => toml::to_string_pretty(self).map_err(|e| {
                OxydeError::ConfigurationError(format!("Failed to serialize TOML prompts: {}", e))
            })?,
            Some("json") => serde_json::to_string_pretty(self).map_err(|e| {
                OxydeError::ConfigurationError(format!("Failed to serialize JSON prompts: {}", e))
            })?,
            Some("yaml") | Some("yml") => serde_yaml::to_string(self).map_err(|e| {
                OxydeError::ConfigurationError(format!("Failed to serialize YAML prompts: {}", e))
            })?,
            _ => {
                return Err(OxydeError::ConfigurationError(
                    "Unknown prompts file format. Expected .toml, .json, .yaml, or .yml"
                        .to_string(),
                ))
            }
        };

        fs::write(path.as_ref(), content).map_err(|e| {
            OxydeError::ConfigurationError(format!("Failed to write prompts file: {}", e))
        })?;

        Ok(())
    }

    /// Generate a complete system prompt for an NPC
    ///
    /// # Arguments
    ///
    /// * `npc_name` - Name of the NPC
    /// * `npc_role` - Role of the NPC
    /// * `conversation_history` - Recent conversation history
    ///
    /// # Returns
    ///
    /// The formatted system prompt
    pub fn generate_system_prompt(
        &self,
        npc_name: &str,
        npc_role: &str,
        conversation_history: &[String],
    ) -> String {
        let template = self
            .system_prompts
            .get(npc_role)
            .unwrap_or(&self.default_system_prompt);

        let mut prompt = template
            .base_description
            .replace("{npc_name}", npc_name)
            .replace("{npc_role}", npc_role);

        if !template.behavior_guidelines.is_empty() {
            prompt.push_str("\n\n");
            for guideline in &template.behavior_guidelines {
                prompt.push_str(&format!("- {}\n", guideline));
            }
        }

        if !template.response_constraints.is_empty() {
            prompt.push_str("\n\n");
            prompt.push_str(&template.response_constraints);
        }

        if !conversation_history.is_empty() {
            let context = conversation_history.join("\n");
            let context_section = template
                .context_template
                .replace("{conversation_history}", &context);
            prompt.push_str("\n\n");
            prompt.push_str(&context_section);
        }

        prompt
    }

    /// Generate an emotional modifier prompt
    ///
    /// # Arguments
    ///
    /// * `dominant_emotion` - The dominant emotional state
    /// * `response_style` - The desired response style
    ///
    /// # Returns
    ///
    /// The formatted emotional modifier
    pub fn generate_emotional_modifier(
        &self,
        dominant_emotion: &str,
        response_style: &str,
    ) -> String {
        let emotion_desc = self
            .emotional_prompts
            .emotional_states
            .get(dominant_emotion)
            .map(|s| s.as_str())
            .unwrap_or("You are in a neutral emotional state.");

        let style_desc = self
            .emotional_prompts
            .response_styles
            .get(response_style)
            .map(|s| s.as_str())
            .unwrap_or("Respond normally and professionally.");

        self.emotional_prompts
            .modifier_template
            .replace("{emotional_state}", emotion_desc)
            .replace("{response_style}", style_desc)
    }

    /// Generate a goal-driven prompt
    ///
    /// # Arguments
    ///
    /// * `goal_description` - Description of the current goal
    /// * `goal_type` - Type of the goal
    /// * `priority` - Priority level (0.0 - 1.0)
    /// * `progress` - Progress toward the goal (0.0 - 1.0)
    /// * `motivation` - Current motivation level
    ///
    /// # Returns
    ///
    /// The formatted goal context prompt
    pub fn generate_goal_prompt(
        &self,
        goal_description: &str,
        goal_type: &str,
        priority: f32,
        progress: f32,
        motivation: f32,
    ) -> String {
        let priority_desc = self.get_priority_description(priority);
        let goal_type_desc = self
            .goal_prompts
            .goal_type_descriptions
            .get(goal_type)
            .map(|s| s.as_str())
            .unwrap_or("personal goal");

        self.goal_prompts
            .goal_context_template
            .replace("{goal_description}", goal_description)
            .replace("{goal_type}", goal_type_desc)
            .replace("{priority}", &priority_desc)
            .replace("{progress}", &format!("{:.0}%", progress * 100.0))
            .replace("{motivation}", &format!("{:.1}/10", motivation * 10.0))
    }

    /// Format memory context for inclusion in prompts
    ///
    /// # Arguments
    ///
    /// * `memories` - List of memory contents and metadata
    ///
    /// # Returns
    ///
    /// The formatted memory context
    pub fn format_memory_context(&self, memories: &[(String, String, f64)]) -> String {
        if memories.is_empty() {
            return String::new();
        }

        let mut context = self.memory_prompts.context_header.clone();
        context.push_str("\n");

        for (content, category, importance) in memories {
            let category_label = self
                .memory_prompts
                .category_labels
                .get(category)
                .map(|s| s.as_str())
                .unwrap_or(category);

            let importance_indicator = self.get_importance_indicator(*importance);

            let formatted = self
                .memory_prompts
                .memory_format
                .replace("{content}", content)
                .replace("{category}", category_label)
                .replace("{importance}", &importance_indicator);

            context.push_str(&formatted);
            context.push('\n');
        }

        context
    }

    /// Get a random greeting for a role
    ///
    /// # Arguments
    ///
    /// * `role` - The NPC role
    ///
    /// # Returns
    ///
    /// A greeting string
    pub fn get_greeting(&self, role: &str) -> String {
        self.greetings
            .get(role)
            .and_then(|greetings| {
                if greetings.is_empty() {
                    None
                } else {
                    Some(greetings[rand::random::<usize>() % greetings.len()].clone())
                }
            })
            .unwrap_or_else(|| "Hello there!".to_string())
    }

    /// Get a random farewell for a role
    ///
    /// # Arguments
    ///
    /// * `role` - The NPC role
    ///
    /// # Returns
    ///
    /// A farewell string
    pub fn get_farewell(&self, role: &str) -> String {
        self.farewells
            .get(role)
            .and_then(|farewells| {
                if farewells.is_empty() {
                    None
                } else {
                    Some(farewells[rand::random::<usize>() % farewells.len()].clone())
                }
            })
            .unwrap_or_else(|| "Goodbye!".to_string())
    }

    // Helper methods

    fn get_priority_description(&self, priority: f32) -> String {
        let key = if priority >= 0.8 {
            "high"
        } else if priority >= 0.5 {
            "medium"
        } else {
            "low"
        };

        self.goal_prompts
            .priority_descriptions
            .get(key)
            .cloned()
            .unwrap_or_else(|| format!("priority level {:.1}", priority))
    }

    fn get_importance_indicator(&self, importance: f64) -> String {
        let key = if importance >= 0.8 {
            "high"
        } else if importance >= 0.5 {
            "medium"
        } else {
            "low"
        };

        self.memory_prompts
            .importance_indicators
            .get(key)
            .cloned()
            .unwrap_or_else(|| format!("importance {:.1}", importance))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bundled_default_config() {
        let config = PromptConfig::from_bundled_default().unwrap();
        assert!(config.system_prompts.contains_key("merchant"));
        assert!(config.system_prompts.contains_key("guard"));
        assert!(config.system_prompts.contains_key("villager"));
    }

    #[test]
    fn test_system_prompt_generation() {
        let config = PromptConfig::from_bundled_default().unwrap();
        let prompt = config.generate_system_prompt("Marcus", "merchant", &[]);
        assert!(prompt.contains("Marcus"));
        assert!(prompt.contains("merchant"));
    }

    #[test]
    fn test_emotional_modifier() {
        let config = PromptConfig::from_bundled_default().unwrap();
        let modifier = config.generate_emotional_modifier("happy", "friendly");
        assert!(!modifier.is_empty());
    }

    #[test]
    fn test_serialization() {
        let config = PromptConfig::from_bundled_default().unwrap();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: PromptConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.version, deserialized.version);
    }
    
    #[test]
    fn test_from_file_or_default_fallback() {
        // Try to load non-existent file, should fall back to bundled
        let config = PromptConfig::from_file_or_default("nonexistent.toml").unwrap();
        assert!(config.system_prompts.contains_key("merchant"));
    }
}
