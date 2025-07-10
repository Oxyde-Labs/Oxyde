//! Intent understanding for player interactions
//!
//! This module provides functionality for understanding player intent from
//! their actions, chat messages, and other interactions.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{OxydeError, Result};

/// Intent represents the player's intended action or request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    /// Type of intent (e.g., "question", "greeting", "command")
    pub intent_type: String,
    
    /// Confidence score for the intent classification (0.0 - 1.0)
    pub confidence: f64,
    
    /// Raw input from the player
    pub raw_input: String,
    
    /// Keywords extracted from the input
    pub keywords: Vec<String>,
}

impl Intent {
    /// Create a new intent
    ///
    /// # Arguments
    ///
    /// * `intent_type` - Type of intent
    /// * `confidence` - Confidence score
    /// * `raw_input` - Raw input from the player
    /// * `keywords` - Keywords extracted from the input
    ///
    /// # Returns
    ///
    /// A new Intent instance
    pub fn new(
        intent_type: &str,
        confidence: f64,
        raw_input: &str,
        keywords: Vec<String>,
    ) -> Self {
        Self {
            intent_type: intent_type.to_string(),
            confidence: confidence.clamp(0.0, 1.0),
            raw_input: raw_input.to_string(),
            keywords,
        }
    }
    
    /// Create a proximity intent
    ///
    /// # Arguments
    ///
    /// * `distance` - Distance to the player
    ///
    /// # Returns
    ///
    /// A proximity Intent
    pub fn proximity(distance: f32) -> Self {
        Self::new(
            "proximity",
            1.0,
            "",
            vec![format!("distance:{}", distance)],
        )
    }
    
    /// Create an intent from player chat
    ///
    /// # Arguments
    ///
    /// * `text` - Player's chat message
    ///
    /// # Returns
    ///
    /// An Intent based on the chat message
    pub fn from_chat(text: &str) -> Self {
        // Extract keywords from the text
        let keywords = Self::extract_keywords(text);
        
        // Determine intent type
        let intent_type = if text.ends_with("?") {
            "question"
        } else if Self::is_greeting(text) {
            "greeting"
        } else if Self::is_command(text) {
            "command"
        } else {
            "chat"
        };
        
        Self::new(
            intent_type,
            0.8, // Confidence score
            text,
            keywords,
        )
    }
    
    /// Analyze player input to determine intent
    ///
    /// # Arguments
    ///
    /// * `input` - Raw player input
    ///
    /// # Returns
    ///
    /// An Intent based on the input
    pub async fn analyze(input: &str) -> Result<Self> {
        if input.is_empty() {
            return Err(OxydeError::IntentError("Empty input".to_string()));
        }
        
        // Simple rule-based intent classification
        // In a real implementation, this would use more sophisticated NLP
        Ok(Self::from_chat(input))
    }
    
    /// Extract keywords from text
    ///
    /// # Arguments
    ///
    /// * `text` - Text to extract keywords from
    ///
    /// # Returns
    ///
    /// Vector of extracted keywords
    pub fn extract_keywords(text: &str) -> Vec<String> {
        let mut keywords = Vec::new();
        let stopwords: HashSet<&str> = [
            "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for",
            "with", "by", "about", "against", "between", "into", "through",
            "is", "are", "was", "were", "be", "been", "being",
            "i", "you", "he", "she", "it", "we", "they",
            "my", "your", "his", "her", "its", "our", "their",
        ].iter().cloned().collect();
        
        for word in text.split_whitespace() {
            let clean_word = word.trim().to_lowercase();
            if clean_word.len() > 2 && !stopwords.contains(clean_word.as_str()) {
                keywords.push(clean_word);
            }
        }
        
        keywords
    }
    
    /// Check if text is a greeting
    ///
    /// # Arguments
    ///
    /// * `text` - Text to check
    ///
    /// # Returns
    ///
    /// Whether the text is a greeting
    fn is_greeting(text: &str) -> bool {
        let greetings = [
            "hello", "hi", "hey", "greetings", "good morning",
            "good afternoon", "good evening", "howdy", "sup",
            "what's up", "hiya",
        ];
        
        let text_lower = text.to_lowercase();
        greetings.iter().any(|g| text_lower.contains(g))
    }
    
    /// Check if text is a command
    ///
    /// # Arguments
    ///
    /// * `text` - Text to check
    ///
    /// # Returns
    ///
    /// Whether the text is a command
    fn is_command(text: &str) -> bool {
        let command_prefixes = [
            "follow", "go", "attack", "defend", "run", "wait",
            "stop", "help", "give", "take", "use", "open",
            "close", "find", "look", "examine", "talk",
        ];
        
        let text_lower = text.to_lowercase();
        command_prefixes.iter().any(|c| text_lower.starts_with(c))
    }
    
    /// Check if the intent has a specific keyword
    ///
    /// # Arguments
    ///
    /// * `keyword` - Keyword to check for
    ///
    /// # Returns
    ///
    /// Whether the intent contains the keyword
    pub fn has_keyword(&self, keyword: &str) -> bool {
        self.keywords.iter().any(|k| k == keyword)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_intent_from_chat() {
        let greeting = Intent::from_chat("Hello there!");
        assert_eq!(greeting.intent_type, "greeting");
        
        let question = Intent::from_chat("What is your name?");
        assert_eq!(question.intent_type, "question");
        
        let command = Intent::from_chat("follow me");
        assert_eq!(command.intent_type, "command");
        
        let chat = Intent::from_chat("I like this village.");
        assert_eq!(chat.intent_type, "chat");
    }
    
    #[test]
    fn test_keyword_extraction() {
        let keywords = Intent::extract_keywords("What is the capital of France?");
        assert!(keywords.contains(&"capital".to_string()));
        assert!(keywords.contains(&"france".to_string()));
        assert!(!keywords.contains(&"is".to_string())); // Stopword should be filtered
    }
}
