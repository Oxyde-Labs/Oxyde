//! Inference engine for the Oxyde SDK
//!
//! This module provides the inference capabilities for generating NPC responses
//! using either local models (via llm crate) or cloud API services.

use std::env;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tokio::time::timeout;

use crate::agent::AgentContext;
use crate::config::InferenceConfig;
use crate::memory::Memory;
use crate::{OxydeError, Result};

/// Inference provider types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderType {
    /// Local model inference
    Local,
    /// Cloud API inference
    Cloud,
}

/// Request to the inference engine
#[derive(Debug, Clone, Serialize)]
pub struct InferenceRequest {
    /// Input text
    pub input: String,
    
    /// System prompt
    pub system_prompt: String,
    
    /// Relevant memories
    pub memories: Vec<Memory>,
    
    /// Context data
    pub context: AgentContext,
    
    /// Maximum tokens to generate
    pub max_tokens: usize,
    
    /// Temperature
    pub temperature: f32,

    ///memory context
    pub memory_context: String,

    /// Language for the response
    pub language: String,
}

/// Response from the inference engine
#[derive(Debug, Clone, Deserialize)]
pub struct InferenceResponse {
    /// Generated text
    pub text: String,
    
    /// Time taken for inference in milliseconds
    pub time_ms: u64,
    
    /// Provider name or identifier
    pub provider_name: String,
    
    /// Tokens generated
    pub tokens: usize,
}

/// Inference engine for generating NPC responses
#[derive(Debug)]
pub struct InferenceEngine {
    /// Configuration for the inference engine
    config: InferenceConfig,
    
    /// Current inference provider type
    provider_type: RwLock<ProviderType>,
    
    /// Statistics about inference
    stats: RwLock<InferenceStats>,
}

/// Statistics about inference operations
#[derive(Debug, Default, Clone)]
pub struct InferenceStats {
    /// Total number of requests
    pub total_requests: usize,
    
    /// Number of successful requests
    pub successful_requests: usize,
    
    /// Number of failed requests
    pub failed_requests: usize,
    
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    
    /// Average tokens generated
    pub avg_tokens: f64,
}

/// Trait for inference providers
#[async_trait]
pub trait InferenceProvider {
    /// Generate a response for the given request
    async fn generate(&self, request: InferenceRequest) -> Result<InferenceResponse>;
}

/// Local model inference provider
pub struct LocalInferenceProvider {
    model_path: String,
}

#[async_trait]
impl InferenceProvider for LocalInferenceProvider {
    async fn generate(&self, request: InferenceRequest) -> Result<InferenceResponse> {
        // Simulate local model inference for now
        // In a real implementation, this would use llm crate to load and run the model
        
        log::info!("Generating response with local model: {}", self.model_path);
        
        let start_time = Instant::now();
        
        // Prepare the prompt
        let mut prompt = String::new();
        
        // Add system prompt
        prompt.push_str(&request.system_prompt);
        prompt.push_str("\n\n");
        
        // Add memories as context
        if !request.memory_context.is_empty() {
            prompt.push_str(&request.memory_context);
            prompt.push_str("\n\n");
        }
        
        // Add user input
        prompt.push_str(&format!("User: {}\n", request.input));
        prompt.push_str("Assistant: ");
        
        // TODO: Replace with actual local model inference
        let response = format!("This is a simulated response to: {}", request.input);
        let token_count = response.split_whitespace().count();
        
        let elapsed = start_time.elapsed();
        
        Ok(InferenceResponse {
            text: response,
            time_ms: elapsed.as_millis() as u64,
            provider_name: "local".to_string(),
            tokens: token_count,
        })
    }
}

/// Cloud API inference provider
pub struct CloudInferenceProvider {
    api_endpoint: String,
    api_key: String,
}

#[async_trait]
impl InferenceProvider for CloudInferenceProvider {
    async fn generate(&self, request: InferenceRequest) -> Result<InferenceResponse> {
        log::info!("Generating response with cloud API: {}", self.api_endpoint);
        
        let start_time = Instant::now();
        
        // Prepare the messages for the API
        let system_message = serde_json::json!({
            "role": "system",
            "content": request.system_prompt,
        });
        
        let mut messages = vec![system_message];
        
        // Add memories as context if available
        if !request.memory_context.is_empty() {
            let memories_content = request.memories.iter()
                .map(|m| format!("- {}", m.content))
                .collect::<Vec<_>>()
                .join("\n");
            
            let context_message = serde_json::json!({
                "role": "system",
                "content": format!("Relevant context:\n{}", memories_content),
            });
            
            messages.push(context_message);
            let context_message = serde_json::json!({
                "role": "system",
                "content": request.memory_context,
            });
            
            messages.push(context_message);
        }

        if request.language != "en" {
            let language_instruction = match request.language.as_str() {
                "es" => "Respond in Spanish.",
                "ja" => "Respond in Japanese (日本語で回答してください).",
                "fr" => "Respond in French.",
                "zh" => "Respond in Chinese (用中文回答).",
                "de" => "Respond in German.",
                "ru" => "Respond in Russian (Ответьте на русском языке).",
                _ => &format!("Respond in language code: {}", request.language),
            };
                
                // INSERT as first system message or append to existing system prompt
                let mut system_content = request.system_prompt.clone();
                system_content.push_str("\n\n");
                system_content.push_str(language_instruction);
                messages[0] = serde_json::json!({
                    "role": "system",
                    "content": system_content,
            });
        }
        
        // Add user message
        let user_message = serde_json::json!({
            "role": "user",
            "content": request.input,
        });
        
        messages.push(user_message);
        
        // Prepare the API request
        let client = reqwest::Client::new();
        let model_name = if self.api_endpoint.contains("openai") {
            "gpt-3.5-turbo"
        } else {
            "llama-2-7b"
        };
        let api_request = serde_json::json!({
            "model": model_name,
            "messages": messages,
            "temperature": request.temperature,
            "max_tokens": request.max_tokens,
        });
        
        // Set timeout for the request
        let duration = Duration::from_millis(request.context.get("timeout_ms")
            .and_then(|v| v.as_u64())
            .unwrap_or(5000));
        
        // Send the request to the API
        let api_response = timeout(duration, async {
            client.post(&self.api_endpoint)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .json(&api_request)
                .send()
                .await
                .map_err(|e| OxydeError::InferenceError(format!("API request failed: {}", e)))?
                .json::<serde_json::Value>()
                .await
                .map_err(|e| OxydeError::InferenceError(format!("Failed to parse API response: {}", e)))
        }).await.map_err(|_| OxydeError::InferenceError("API request timed out".to_string()))??;
        
        // Extract the response text
        let response_text = api_response["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| OxydeError::InferenceError("Invalid API response format".to_string()))?
            .to_string();
            
        // Count tokens before moving the string
        let token_count = response_text.split_whitespace().count();
        
        let elapsed = start_time.elapsed();
        
        Ok(InferenceResponse {
            text: response_text,
            time_ms: elapsed.as_millis() as u64,
            provider_name: "cloud".to_string(),
            tokens: token_count,
        })
    }
}

impl InferenceEngine {
    /// Create a new inference engine with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Inference engine configuration
    ///
    /// # Returns
    ///
    /// A new InferenceEngine instance
    pub fn new(config: &InferenceConfig) -> Self {
        let provider_type = if config.use_local {
            ProviderType::Local
        } else {
            ProviderType::Cloud
        };
        
        Self {
            config: config.clone(),
            provider_type: RwLock::new(provider_type),
            stats: RwLock::new(InferenceStats::default()),
        }
    }
    
    /// Generate a response for the given input
    ///
    /// # Arguments
    ///
    /// * `input` - User input to respond to
    /// * `memories` - Relevant memories for context
    /// * `context` - Additional context data
    ///
    /// # Returns
    ///
    /// The generated response text
    pub async fn generate_response(
        &self,
        input: &str,
        memories: &[Memory],
        context: &AgentContext,
        system_prompt: &str,
        memory_context: &str,
        language: Option<&str>,
    ) -> Result<String> {
        let request = self.prepare_request(input, memories, context, system_prompt, memory_context, language);
        
        // Try primary provider first
        let provider_type = *self.provider_type.read().await;
        let response = self.generate_with_provider(provider_type, request.clone()).await;
        
        // If primary fails and fallback is available, try fallback
        if response.is_err() && self.config.fallback_api.is_some() {
            log::warn!("Primary inference provider failed, trying fallback");
            
            let fallback_provider = match provider_type {
                ProviderType::Local => ProviderType::Cloud,
                ProviderType::Cloud => ProviderType::Local,
            };
            
            // Update stats for the failed request
            {
                let mut stats = self.stats.write().await;
                stats.total_requests += 1;
                stats.failed_requests += 1;
            }
            
            return self.generate_with_provider(fallback_provider, request).await
                .map(|response| response.text);
        }
        
        response.map(|response| response.text)
    }
    
    /// Prepare an inference request
    fn prepare_request(
        &self,
        input: &str,
        memories: &[Memory],
        context: &AgentContext,
        system_prompt: &str,
        memory_context: &str,
        language: Option<&str>,
    ) -> InferenceRequest {
        // Create system prompt for the agent
        // let system_prompt = format!(
        //     "You are an NPC named {} who is a {}. \
        //     Respond in character with brief, concise answers.",
        //     context.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown"),
        //     context.get("role").and_then(|v| v.as_str()).unwrap_or("character"),
        // );
        
        InferenceRequest {
            input: input.to_string(),
            system_prompt: system_prompt.to_string(),
            memories: memories.to_vec(),
            context: context.clone(),
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            memory_context: memory_context.to_string(),
            language: language.unwrap_or("en").to_string(),
        }
    }
    
    /// Generate a response with the specified provider type
    async fn generate_with_provider(
        &self,
        provider_type: ProviderType,
        request: InferenceRequest,
    ) -> Result<InferenceResponse> {
        dotenvy::dotenv().ok();

        let response = match provider_type {
            ProviderType::Local => {
                if let Some(model_path) = &self.config.local_model_path {
                    let local_provider = LocalInferenceProvider {
                        model_path: model_path.clone(),
                    };
                    local_provider.generate(request).await
                } else {
                    return Err(OxydeError::InferenceError(
                        "No local model path configured".to_string()
                    ));
                }
            },
            ProviderType::Cloud => {
                let api_endpoint = self.config.api_endpoint.clone()
                    .ok_or_else(|| OxydeError::InferenceError(
                        "No API endpoint configured".to_string()
                    ))?;
                
                let api_key = self.config.api_key.clone()
                    .or_else(|| env::var("OXYDE_API_KEY").ok())
                    .ok_or_else(|| OxydeError::InferenceError(
                        "No API key configured. Set OXYDE_API_KEY environment variable or configure in InferenceConfig".to_string()
                    ))?;

                
                let cloud_provider = CloudInferenceProvider {
                    api_endpoint,
                    api_key,
                };
                
                cloud_provider.generate(request).await
            }
        };
        
        // Update stats on success
        if let Ok(ref resp) = response {
            let mut stats = self.stats.write().await;
            stats.total_requests += 1;
            stats.successful_requests += 1;
            
            // Update moving average for latency and tokens
            let count = stats.successful_requests as f64;
            stats.avg_latency_ms = (stats.avg_latency_ms * (count - 1.0) + resp.time_ms as f64) / count;
            stats.avg_tokens = (stats.avg_tokens * (count - 1.0) + resp.tokens as f64) / count;
        }
        
        response
    }
    
    /// Switch to a different inference provider type
    ///
    /// # Arguments
    ///
    /// * `provider_type` - The provider type to switch to
    pub async fn switch_provider(&self, provider_type: ProviderType) {
        let mut current = self.provider_type.write().await;
        *current = provider_type;
        
        log::info!("Switched to {:?} inference provider", provider_type);
    }
    
    /// Get current inference statistics
    pub async fn get_stats(&self) -> InferenceStats {
        self.stats.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_inference_engine_creation() {
        let config = InferenceConfig::default();
        let engine = InferenceEngine::new(&config);
        
        let provider_type = *engine.provider_type.read().await;
        assert_eq!(provider_type, ProviderType::Cloud);
        
        let stats = engine.get_stats().await;
        assert_eq!(stats.total_requests, 0);
    }

    // NOTE: This test bypasses PromptConfig intentionally.
    // It only validates generate_response wiring.
    #[tokio::test]
    async fn test_inference_with_prompts() {
        let config = InferenceConfig::default();
        let engine = InferenceEngine::new(&config);
        
        let memories = vec![];
        let context = AgentContext::new();
        let system_prompt = "You are a test NPC.";
        let memory_context = "Previous interactions: none";
        
        // This will fail without API key, but tests the signature
        let result = engine.generate_response(
            "Hello",
            &memories,
            &context,
            system_prompt,
            memory_context,
            Some("en"),
        ).await;
        
        // We expect an error due to missing API key, not a panic ..... fix
        assert!(result.is_err());
    }
}
