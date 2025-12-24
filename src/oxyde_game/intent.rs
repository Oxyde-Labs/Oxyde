//! Intent understanding for player interactions
//!
//! This module provides functionality for understanding player intent from
//! their actions, chat messages, and other interactions.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{inference::InferenceEngine, OxydeError, Result};

/// Type of player intent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IntentType {
    /// Player is asking a question
    Question,
    /// Player is greeting the NPC
    Greeting,
    /// Player is issuing a command
    Command,
    /// General chat/conversation
    Chat,
    /// Proximity-based intent (player approaching/nearby)
    Proximity,
    /// Custom/unknown intent type
    Custom,
}

impl IntentType {
    /// Convert from string representation
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "question" => Self::Question,
            "greeting" => Self::Greeting,
            "command" => Self::Command,
            "chat" => Self::Chat,
            "proximity" => Self::Proximity,
            _ => Self::Custom,
        }
    }

    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Question => "question",
            Self::Greeting => "greeting",
            Self::Command => "command",
            Self::Chat => "chat",
            Self::Proximity => "proximity",
            Self::Custom => "custom",
        }
    }
}

impl std::fmt::Display for IntentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Intent represents the player's intended action or request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    /// Type of intent
    pub intent_type: IntentType,

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
        intent_type: IntentType,
        confidence: f64,
        raw_input: &str,
        keywords: Vec<String>,
    ) -> Self {
        Self {
            intent_type,
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
            IntentType::Proximity,
            1.0,
            "",
            vec![format!("distance:{}", distance)],
        )
    }


    /// LLM-based intent analysis (language-agnostic)
    async fn llm_based_analysis(
        input: &str,
        engine: &InferenceEngine,
        language: &str,
    ) -> Result<Self> {
        let language_note = if language != "en" {
            format!(" (Note: Input may be in language code: {})", language)
        } else {
            String::new()
        };

        let prompt = format!(
            "Classify the intent of this message{}: \"{}\"\n\n\
            Respond with ONLY ONE of these exact English words, nothing else: greeting, question, command, chat\n\
            For Non English inputs, first translate to English, then classify as one of those four.\n\n\
            - greeting: if saying hello, hi, greetings, or similar\n\
            - question: if asking something (contains ?, what, who, where, when, why, how)\n\
            - command: if giving an order or request (follow me, attack, go, stop)\n\
            - chat: if making a statement or general conversation\n\n\
            Intent:",
            language_note,
            input
        );

        // Use inference engine to classify
        let response = engine.generate_response(
            &prompt,
            &[], 
            &std::collections::HashMap::new(), 
            &prompt, 
            "",
            Some(language)
        ).await?;

        let intent_type = IntentType::from_str(response.trim());
        let keywords = Self::extract_keywords(input);

        Ok(Self::new(
            intent_type,
            0.9, // High confidence from LLM
            input,
            keywords,
        ))
    }

    /// fallback when no LLM .... could be made more robust but focusing on LLM for now
    fn simple_analysis(input: &str) -> Result<Self> {
        let keywords = Self::extract_keywords(input);
        
        let intent_type = if input.ends_with('?') {
            IntentType::Question
        } else if ["follow", "go", "stop", "attack", "come"]
            .iter()
            .any(|cmd| input.to_lowercase().starts_with(cmd))
        {
            IntentType::Command
        } else if input.split_whitespace().count() <= 2 {
            IntentType::Greeting
        } else {
            IntentType::Chat
        };

        Ok(Self::new(
            intent_type,
            0.6, 
            input,
            keywords,
        ))
    }


    /// Analyze player input to determine intent
    ///
    /// # Arguments
    ///
    /// * `input` - Raw player input
    /// * `inference` - Optional inference engine for LLM-based analysis
    /// * `language` - Language code of the input (e.g., "en" for English)
    ///
    /// # Returns
    ///
    /// An Intent based on the input
    
    pub async fn analyze(input: &str, inference: Option<&InferenceEngine>, language: &str) -> Result<Self> {

    if input.trim().is_empty() {
        return Err(OxydeError::IntentError("Empty input".to_string()));
    }

    if let Some(engine) = inference {
        match Self::llm_based_analysis(input, engine, language).await {
            Ok(intent) => return Ok(intent),
            Err(err) => {
                log::warn!("LLM intent analysis failed: {}", err);
            }
        }
    }

    Self::simple_analysis(input)
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
        // Simple approach: take longer words, skip very common short words
        let common_short_words: HashSet<&str> = [
            // English
            "the", "a", "an", "and", "or", "but", "in", "on", "at", "to", "for",
            "with", "by", "from", "up", "about", "is", "are", "was", "were",
            "be", "been", "being", "have", "has", "had", "do", "does", "did",
            "will", "would", "could", "should", "may", "might", "must",
            "i", "you", "he", "she", "it", "we", "they", "me", "him", "her",
            "my", "your", "his", "its", "our", "their",
           
            // Spanish
            "el", "la", "los", "las", "un", "una", "de", "en", "es", "y", "o",
            
            // French  
            "le", "la", "les", "un", "une", "de", "en", "et", "est", "ou",
            
            // German
            "der", "die", "das", "ein", "eine", "und", "ist", "oder",
            
            // Japanese 
            "は", "が", "を", "に", "で", "と", "の", "も", "へ", "から",
        ].iter().cloned().collect();

        text.split_whitespace()
            .filter_map(|word| {
                let clean = word
                    .trim_matches(|c: char| !c.is_alphanumeric())
                    .to_lowercase();
                
           
                if !clean.is_empty() && !common_short_words.contains(clean.as_str()) {
                    // Check if it's CJK (Chinese, Japanese, Korean)
                    let is_cjk = clean.chars().any(|c| {
                        matches!(c, '\u{4E00}'..='\u{9FFF}' | '\u{3040}'..='\u{309F}' | '\u{30A0}'..='\u{30FF}' | '\u{AC00}'..='\u{D7AF}')
                    });
                    
                    if is_cjk || clean.len() >= 3 {
                        Some(clean)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::InferenceConfig;

    #[tokio::test]
    async fn test_intent_from_chat() -> Result<()> {
        let greeting = Intent::analyze("Hello there!", None, "en").await?;
        assert_eq!(greeting.intent_type, IntentType::Greeting);

        let question = Intent::analyze("What is your name?", None, "en").await?;
        assert_eq!(question.intent_type, IntentType::Question);

        let command = Intent::analyze("follow me", None, "en").await?;
        assert_eq!(command.intent_type, IntentType::Command);

        let chat = Intent::analyze("I like this village.", None, "en").await?;
        assert_eq!(chat.intent_type, IntentType::Chat);
        Ok(())
    }
    
    #[tokio::test]
    async fn test_llm_based_intent_detection() -> Result<()> {
        // Initialize the inference engine with cloud API configuration
        let config = InferenceConfig {
            use_local: false,
            api_endpoint: Some("https://api.openai.com/v1/chat/completions".to_string()),
            api_key: None, // Will use OXYDE_API_KEY or OPENAI_API_KEY from env
            local_model_path: None,
            max_tokens: 100,
            temperature: 0.5,
            fallback_api: None,
            model: "gpt-3.5-turbo".to_string(),
            timeout_ms: 10000,
        };
        
        let engine = crate::inference::InferenceEngine::new(&config);
        
        // Test various intents with LLM-based analysis
        let greeting = Intent::analyze("Hello there, friend!", Some(&engine), "en").await?;
        assert_eq!(greeting.intent_type, IntentType::Greeting);
        assert!(greeting.confidence > 0.8);
        
        let question = Intent::analyze("What is your name and where are you from?", Some(&engine), "en").await?;
        assert_eq!(question.intent_type, IntentType::Question);
        assert!(question.confidence > 0.8);
        
        let command = Intent::analyze("Follow me to the treasure!", Some(&engine), "en").await?;
        assert_eq!(command.intent_type, IntentType::Command);
        assert!(command.confidence > 0.8);
        
        let chat = Intent::analyze("I think this village is beautiful", Some(&engine), "en").await?;
        assert_eq!(chat.intent_type, IntentType::Chat);
        assert!(chat.confidence > 0.8);
        
        Ok(())
    }

      #[tokio::test]
    async fn test_llm_based_intent_multilingual() -> Result<()> {
        // Test LLM-based intent detection with different languages
        let config = InferenceConfig {
            use_local: false,
            api_endpoint: Some("https://api.openai.com/v1/chat/completions".to_string()),
            api_key: None,
            local_model_path: None,
            max_tokens: 1000,
            temperature: 0.5,
            fallback_api: None,
            model: "gpt-3.5-turbo".to_string(),
            timeout_ms: 10000,
        };
        
        let engine = crate::inference::InferenceEngine::new(&config);
        
        // Spanish greeting
        let spanish_greeting = Intent::analyze("¡Hola amigo!", Some(&engine), "es").await?;
        assert_eq!(spanish_greeting.intent_type, IntentType::Greeting);
        
        // Japanese question
        // let japanese_question = Intent::analyze("あなたの名前は何ですか？", Some(&engine), "ja").await?;
        // println!("Japanese  Question: {:?}",japanese_question );
        // assert_eq!(japanese_question.intent_type, IntentType::Question);
        
        // French command
        let french_command = Intent::analyze("Suis-moi!", Some(&engine), "fr").await?;
        assert_eq!(french_command.intent_type, IntentType::Command);
        
        Ok(())
    }

    #[test]
    fn test_keyword_extraction() {
        let keywords = Intent::extract_keywords("What is the capital of France?");
        assert!(keywords.contains(&"capital".to_string()));
        assert!(keywords.contains(&"france".to_string()));
        assert!(!keywords.contains(&"is".to_string())); 
    }
}
