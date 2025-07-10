use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<ChatChoice>,
}

#[derive(Debug, Clone)]
pub enum LLMProvider {
    OpenAI,
    Groq,
    Anthropic,
    Perplexity,
    XAI,
    Local, // For future Llama.cpp integration
}

pub struct LLMService {
    client: reqwest::Client,
    provider: LLMProvider,
    api_key: String,
}

impl LLMService {
    pub fn new(provider: LLMProvider) -> Result<Self, Box<dyn std::error::Error>> {
        let api_key = match provider {
            LLMProvider::OpenAI => env::var("OPENAI_API_KEY")
                .map_err(|_| "OPENAI_API_KEY environment variable not set")?,
            LLMProvider::Groq => env::var("GROQ_API_KEY")
                .map_err(|_| "GROQ_API_KEY environment variable not set")?,
            LLMProvider::Anthropic => env::var("ANTHROPIC_API_KEY")
                .map_err(|_| "ANTHROPIC_API_KEY environment variable not set")?,
            LLMProvider::Perplexity => env::var("PERPLEXITY_API_KEY")
                .map_err(|_| "PERPLEXITY_API_KEY environment variable not set")?,
            LLMProvider::XAI => env::var("XAI_API_KEY")
                .map_err(|_| "XAI_API_KEY environment variable not set")?,
            LLMProvider::Local => String::new(), // No API key needed for local
        };

        Ok(Self {
            client: reqwest::Client::new(),
            provider,
            api_key,
        })
    }

    pub async fn generate_response(
        &self,
        npc_name: &str,
        npc_role: &str,
        player_message: &str,
        conversation_history: &[String],
    ) -> Result<String, Box<dyn std::error::Error>> {
        match self.provider {
            LLMProvider::OpenAI => self.openai_request(npc_name, npc_role, player_message, conversation_history).await,
            LLMProvider::Groq => self.groq_request(npc_name, npc_role, player_message, conversation_history).await,
            LLMProvider::Anthropic => self.openai_request(npc_name, npc_role, player_message, conversation_history).await,
            LLMProvider::Perplexity => self.groq_request(npc_name, npc_role, player_message, conversation_history).await,
            LLMProvider::XAI => self.groq_request(npc_name, npc_role, player_message, conversation_history).await,
            LLMProvider::Local => self.local_request(npc_name, npc_role, player_message, conversation_history).await,
        }
    }

    pub async fn generate_emotional_response(
        &self,
        npc_name: &str,
        npc_role: &str,
        player_message: &str,
        conversation_history: &[String],
        emotional_modifier: &str,
        response_style: &crate::emotion_engine::ResponseStyle,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match self.provider {
            LLMProvider::OpenAI => self.openai_emotional_request(npc_name, npc_role, player_message, conversation_history, emotional_modifier, response_style).await,
            LLMProvider::Groq => self.groq_emotional_request(npc_name, npc_role, player_message, conversation_history, emotional_modifier, response_style).await,
            LLMProvider::Anthropic => self.openai_emotional_request(npc_name, npc_role, player_message, conversation_history, emotional_modifier, response_style).await,
            LLMProvider::Perplexity => self.groq_emotional_request(npc_name, npc_role, player_message, conversation_history, emotional_modifier, response_style).await,
            LLMProvider::XAI => self.groq_emotional_request(npc_name, npc_role, player_message, conversation_history, emotional_modifier, response_style).await,
            LLMProvider::Local => self.local_request(npc_name, npc_role, player_message, conversation_history).await,
        }
    }

    async fn openai_request(
        &self,
        npc_name: &str,
        npc_role: &str,
        player_message: &str,
        conversation_history: &[String],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let system_prompt = self.create_system_prompt(npc_name, npc_role, conversation_history);
        
        let mut messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            }
        ];

        // Add conversation history
        for (i, msg) in conversation_history.iter().enumerate() {
            let role = if i % 2 == 0 { "user" } else { "assistant" };
            messages.push(ChatMessage {
                role: role.to_string(),
                content: msg.clone(),
            });
        }

        // Add current player message
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: player_message.to_string(),
        });

        let request = ChatRequest {
            model: "gpt-4o".to_string(), // Latest OpenAI model
            messages,
            max_tokens: Some(150),
            temperature: Some(0.8),
        };

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let chat_response: ChatResponse = response.json().await?;
        
        Ok(chat_response.choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "I'm having trouble speaking right now.".to_string()))
    }

    async fn groq_request(
        &self,
        npc_name: &str,
        npc_role: &str,
        player_message: &str,
        conversation_history: &[String],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let system_prompt = self.create_system_prompt(npc_name, npc_role, conversation_history);
        
        let mut messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            }
        ];

        // Add conversation history
        for (i, msg) in conversation_history.iter().enumerate() {
            let role = if i % 2 == 0 { "user" } else { "assistant" };
            messages.push(ChatMessage {
                role: role.to_string(),
                content: msg.clone(),
            });
        }

        // Add current player message
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: player_message.to_string(),
        });

        let request = ChatRequest {
            model: "llama3-8b-8192".to_string(), // Fast Groq model
            messages,
            max_tokens: Some(150),
            temperature: Some(0.8),
        };

        let response = self.client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let chat_response: ChatResponse = response.json().await?;
        
        Ok(chat_response.choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "I'm having trouble speaking right now.".to_string()))
    }

    async fn local_request(
        &self,
        _npc_name: &str,
        _npc_role: &str,
        _player_message: &str,
        _conversation_history: &[String],
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Placeholder for future Llama.cpp integration
        Ok("Local AI inference not implemented yet.".to_string())
    }

    pub async fn generate_goal_driven_response(
        &self,
        npc_name: &str,
        npc_role: &str,
        player_message: &str,
        conversation_history: &[String],
        emotional_modifier: &str,
        response_style: &str,
        goal_context: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match self.provider {
            LLMProvider::OpenAI => {
                self.openai_goal_driven_request(npc_name, npc_role, player_message, conversation_history, emotional_modifier, response_style, goal_context).await
            }
            LLMProvider::Groq => {
                self.groq_goal_driven_request(npc_name, npc_role, player_message, conversation_history, emotional_modifier, response_style, goal_context).await
            }
            LLMProvider::Anthropic => {
                self.anthropic_goal_driven_request(npc_name, npc_role, player_message, conversation_history, emotional_modifier, response_style, goal_context).await
            }
            LLMProvider::Perplexity => {
                self.perplexity_goal_driven_request(npc_name, npc_role, player_message, conversation_history, emotional_modifier, response_style, goal_context).await
            }
            LLMProvider::XAI => {
                self.xai_goal_driven_request(npc_name, npc_role, player_message, conversation_history, emotional_modifier, response_style, goal_context).await
            }
            LLMProvider::Local => {
                self.local_request(npc_name, npc_role, player_message, conversation_history).await
            }
        }
    }

    async fn openai_emotional_request(
        &self,
        npc_name: &str,
        npc_role: &str,
        player_message: &str,
        conversation_history: &[String],
        emotional_modifier: &str,
        response_style: &crate::emotion_engine::ResponseStyle,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let system_prompt = self.create_emotional_system_prompt(npc_name, npc_role, conversation_history, emotional_modifier, response_style);
        
        let mut messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            }
        ];

        // Add conversation history
        for (i, msg) in conversation_history.iter().enumerate() {
            let role = if i % 2 == 0 { "user" } else { "assistant" };
            messages.push(ChatMessage {
                role: role.to_string(),
                content: msg.clone(),
            });
        }

        // Add current player message
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: player_message.to_string(),
        });

        let request = ChatRequest {
            model: "gpt-4o".to_string(),
            messages,
            max_tokens: Some(150),
            temperature: Some(0.8),
        };

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let chat_response: ChatResponse = response.json().await?;
        
        Ok(chat_response.choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "I'm having trouble speaking right now.".to_string()))
    }

    async fn groq_emotional_request(
        &self,
        npc_name: &str,
        npc_role: &str,
        player_message: &str,
        conversation_history: &[String],
        emotional_modifier: &str,
        response_style: &crate::emotion_engine::ResponseStyle,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let system_prompt = self.create_emotional_system_prompt(npc_name, npc_role, conversation_history, emotional_modifier, response_style);
        
        let mut messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            }
        ];

        // Add conversation history
        for (i, msg) in conversation_history.iter().enumerate() {
            let role = if i % 2 == 0 { "user" } else { "assistant" };
            messages.push(ChatMessage {
                role: role.to_string(),
                content: msg.clone(),
            });
        }

        // Add current player message
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: player_message.to_string(),
        });

        let request = ChatRequest {
            model: "llama3-8b-8192".to_string(),
            messages,
            max_tokens: Some(150),
            temperature: Some(0.8),
        };

        let response = self.client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let chat_response: ChatResponse = response.json().await?;
        
        Ok(chat_response.choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "I'm having trouble speaking right now.".to_string()))
    }

    async fn openai_goal_driven_request(
        &self,
        npc_name: &str,
        npc_role: &str,
        player_message: &str,
        conversation_history: &[String],
        emotional_modifier: &str,
        response_style: &str,
        goal_context: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let system_prompt = self.create_goal_driven_system_prompt(npc_name, npc_role, conversation_history, emotional_modifier, response_style, goal_context);
        
        let mut messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            }
        ];

        // Add conversation history
        for (i, msg) in conversation_history.iter().enumerate() {
            let role = if i % 2 == 0 { "user" } else { "assistant" };
            messages.push(ChatMessage {
                role: role.to_string(),
                content: msg.clone(),
            });
        }

        // Add current player message
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: player_message.to_string(),
        });

        let request = ChatRequest {
            model: "gpt-4o".to_string(),
            messages,
            max_tokens: Some(150),
            temperature: Some(0.8),
        };

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let chat_response: ChatResponse = response.json().await?;
        
        Ok(chat_response.choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "I'm having trouble speaking right now.".to_string()))
    }

    async fn groq_goal_driven_request(
        &self,
        npc_name: &str,
        npc_role: &str,
        player_message: &str,
        conversation_history: &[String],
        emotional_modifier: &str,
        response_style: &str,
        goal_context: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let system_prompt = self.create_goal_driven_system_prompt(npc_name, npc_role, conversation_history, emotional_modifier, response_style, goal_context);
        
        let mut messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            }
        ];

        // Add conversation history
        for (i, msg) in conversation_history.iter().enumerate() {
            let role = if i % 2 == 0 { "user" } else { "assistant" };
            messages.push(ChatMessage {
                role: role.to_string(),
                content: msg.clone(),
            });
        }

        // Add current player message
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: player_message.to_string(),
        });

        let request = ChatRequest {
            model: "llama3-8b-8192".to_string(),
            messages,
            max_tokens: Some(150),
            temperature: Some(0.8),
        };

        let response = self.client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let chat_response: ChatResponse = response.json().await?;
        
        Ok(chat_response.choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "I'm having trouble speaking right now.".to_string()))
    }

    async fn anthropic_goal_driven_request(
        &self,
        npc_name: &str,
        npc_role: &str,
        player_message: &str,
        conversation_history: &[String],
        emotional_modifier: &str,
        response_style: &str,
        goal_context: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let system_prompt = self.create_goal_driven_system_prompt(npc_name, npc_role, conversation_history, emotional_modifier, response_style, goal_context);
        
        let mut messages = vec![];

        // Add conversation history
        for (i, msg) in conversation_history.iter().enumerate() {
            let role = if i % 2 == 0 { "user" } else { "assistant" };
            messages.push(ChatMessage {
                role: role.to_string(),
                content: msg.clone(),
            });
        }

        // Add current player message
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: player_message.to_string(),
        });

        // Anthropic API uses a different format with system message separate
        let anthropic_request = serde_json::json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens": 150,
            "system": system_prompt,
            "messages": messages
        });

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .json(&anthropic_request)
            .send()
            .await?;

        let response_text = response.text().await?;
        let anthropic_response: serde_json::Value = serde_json::from_str(&response_text)?;
        
        // Extract content from Anthropic's response format
        if let Some(content) = anthropic_response["content"].as_array() {
            if let Some(text_content) = content.first() {
                if let Some(text) = text_content["text"].as_str() {
                    return Ok(text.to_string());
                }
            }
        }
        
        Ok("I'm having trouble speaking right now.".to_string())
    }

    async fn perplexity_goal_driven_request(
        &self,
        npc_name: &str,
        npc_role: &str,
        player_message: &str,
        conversation_history: &[String],
        emotional_modifier: &str,
        response_style: &str,
        goal_context: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let system_prompt = self.create_goal_driven_system_prompt(npc_name, npc_role, conversation_history, emotional_modifier, response_style, goal_context);
        
        let mut messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            }
        ];

        // Add conversation history
        for (i, msg) in conversation_history.iter().enumerate() {
            let role = if i % 2 == 0 { "user" } else { "assistant" };
            messages.push(ChatMessage {
                role: role.to_string(),
                content: msg.clone(),
            });
        }

        // Add current player message
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: player_message.to_string(),
        });

        let request = ChatRequest {
            model: "llama-3.1-sonar-small-128k-online".to_string(),
            messages,
            max_tokens: Some(150),
            temperature: Some(0.8),
        };

        let response = self.client
            .post("https://api.perplexity.ai/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let chat_response: ChatResponse = response.json().await?;
        
        Ok(chat_response.choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "I'm having trouble speaking right now.".to_string()))
    }

    async fn xai_goal_driven_request(
        &self,
        npc_name: &str,
        npc_role: &str,
        player_message: &str,
        conversation_history: &[String],
        emotional_modifier: &str,
        response_style: &str,
        goal_context: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let system_prompt = self.create_goal_driven_system_prompt(npc_name, npc_role, conversation_history, emotional_modifier, response_style, goal_context);
        
        let mut messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            }
        ];

        // Add conversation history
        for (i, msg) in conversation_history.iter().enumerate() {
            let role = if i % 2 == 0 { "user" } else { "assistant" };
            messages.push(ChatMessage {
                role: role.to_string(),
                content: msg.clone(),
            });
        }

        // Add current player message
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: player_message.to_string(),
        });

        let request = ChatRequest {
            model: "grok-2-1212".to_string(),
            messages,
            max_tokens: Some(150),
            temperature: Some(0.8),
        };

        let response = self.client
            .post("https://api.x.ai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let chat_response: ChatResponse = response.json().await?;
        
        Ok(chat_response.choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "I'm having trouble speaking right now.".to_string()))
    }

    fn create_system_prompt(&self, npc_name: &str, npc_role: &str, conversation_history: &[String]) -> String {
        let context = if conversation_history.is_empty() {
            String::new()
        } else {
            format!("\n\nRecent conversation context:\n{}", conversation_history.join("\n"))
        };

        match npc_role {
            "Merchant" => format!(
                "You are {}, a friendly merchant in a fantasy RPG. You sell various goods and are always looking for business opportunities. You're knowledgeable about trade routes, valuable items, and local commerce. Keep responses under 50 words and stay in character. Be helpful but business-minded.{}",
                npc_name, context
            ),
            "Guard" => format!(
                "You are {}, a dutiful town guard in a fantasy RPG. You're responsible for keeping the peace and protecting citizens. You're professional, alert, and knowledgeable about local security and laws. Keep responses under 50 words and stay in character. Be authoritative but fair.{}",
                npc_name, context
            ),
            "Villager" => format!(
                "You are {}, a friendly village resident in a fantasy RPG. You know local gossip, daily life, and community happenings. You're curious about travelers and enjoy chatting. Keep responses under 50 words and stay in character. Be warm and conversational.{}",
                npc_name, context
            ),
            _ => format!(
                "You are {} in a fantasy RPG setting. Keep responses under 50 words and stay in character as a helpful NPC.{}",
                npc_name, context
            ),
        }
    }

    fn create_emotional_system_prompt(&self, npc_name: &str, npc_role: &str, conversation_history: &[String], emotional_modifier: &str, response_style: &crate::emotion_engine::ResponseStyle) -> String {
        let context = if conversation_history.is_empty() {
            String::new()
        } else {
            format!("\n\nRecent conversation context:\n{}", conversation_history.join("\n"))
        };

        let base_prompt = match npc_role {
            "Merchant" => format!(
                "You are {}, a merchant in a fantasy RPG. You sell various goods and are always looking for business opportunities. You're knowledgeable about trade routes, valuable items, and local commerce.",
                npc_name
            ),
            "Guard" => format!(
                "You are {}, a town guard in a fantasy RPG. You're responsible for keeping the peace and protecting citizens. You're professional, alert, and knowledgeable about local security and laws.",
                npc_name
            ),
            "Villager" => format!(
                "You are {}, a village resident in a fantasy RPG. You know local gossip, daily life, and community happenings. You're curious about travelers and enjoy chatting.",
                npc_name
            ),
            _ => format!(
                "You are {} in a fantasy RPG setting. You're a helpful NPC.",
                npc_name
            ),
        };

        format!(
            "{}. {} {} Keep responses under 50 words and stay in character.{}",
            base_prompt,
            emotional_modifier,
            response_style.get_style_prompt(),
            context
        )
    }

    fn create_goal_driven_system_prompt(&self, npc_name: &str, npc_role: &str, conversation_history: &[String], emotional_modifier: &str, response_style: &str, goal_context: &str) -> String {
        let context = if conversation_history.is_empty() {
            String::new()
        } else {
            format!("\n\nRecent conversation context:\n{}", conversation_history.join("\n"))
        };

        let base_prompt = match npc_role {
            "Merchant" => format!(
                "You are {}, a merchant in a fantasy RPG. You sell various goods and are always looking for business opportunities. You're knowledgeable about trade routes, valuable items, and local commerce.",
                npc_name
            ),
            "Guard" => format!(
                "You are {}, a town guard in a fantasy RPG. You're responsible for keeping the peace and protecting citizens. You're professional, alert, and knowledgeable about local security and laws.",
                npc_name
            ),
            "Villager" => format!(
                "You are {}, a village resident in a fantasy RPG. You know local gossip, daily life, and community happenings. You're curious about travelers and enjoy chatting.",
                npc_name
            ),
            _ => format!(
                "You are {} in a fantasy RPG setting. You're a helpful NPC.",
                npc_name
            ),
        };

        format!(
            "{}. {} You are driven by personal goals and motivations. {}. Your responses should reflect your current objectives and emotional state. Occasionally mention your goals or hint at your plans when relevant to the conversation. Keep responses under 50 words and stay in character.{}",
            base_prompt,
            emotional_modifier,
            goal_context,
            context
        )
    }

    pub fn get_provider_name(&self) -> &str {
        match self.provider {
            LLMProvider::OpenAI => "OpenAI",
            LLMProvider::Groq => "Groq",
            LLMProvider::Anthropic => "Anthropic",
            LLMProvider::Perplexity => "Perplexity",
            LLMProvider::XAI => "xAI",
            LLMProvider::Local => "Local",
        }
    }

    pub fn supports_fast_inference(&self) -> bool {
        matches!(self.provider, LLMProvider::Groq | LLMProvider::XAI | LLMProvider::Local)
    }
}

// Smart provider selection based on context and capabilities
pub fn select_optimal_provider(context: &str) -> LLMProvider {
    // Use xAI Grok for creative and conversational scenarios
    if context.contains("creative") || context.contains("humor") || context.contains("story") {
        if env::var("XAI_API_KEY").is_ok() {
            return LLMProvider::XAI;
        }
    }
    
    // Use Anthropic Claude for complex reasoning and detailed analysis
    if context.contains("analysis") || context.contains("reasoning") || context.contains("complex") {
        if env::var("ANTHROPIC_API_KEY").is_ok() {
            return LLMProvider::Anthropic;
        }
    }
    
    // Use Perplexity for real-time knowledge and current events
    if context.contains("current") || context.contains("news") || context.contains("recent") {
        if env::var("PERPLEXITY_API_KEY").is_ok() {
            return LLMProvider::Perplexity;
        }
    }
    
    // Use Groq for quick reactions and simple responses
    if context.len() < 100 || context.contains("quick") || context.contains("fast") {
        if env::var("GROQ_API_KEY").is_ok() {
            return LLMProvider::Groq;
        }
    }
    
    // Use OpenAI for emotional scenarios and general purpose tasks
    if context.contains("emotion") || context.contains("goal") {
        if env::var("OPENAI_API_KEY").is_ok() {
            return LLMProvider::OpenAI;
        }
    }
    
    // Intelligent fallback priority based on availability and capabilities
    if env::var("OPENAI_API_KEY").is_ok() {
        LLMProvider::OpenAI
    } else if env::var("ANTHROPIC_API_KEY").is_ok() {
        LLMProvider::Anthropic
    } else if env::var("GROQ_API_KEY").is_ok() {
        LLMProvider::Groq
    } else if env::var("XAI_API_KEY").is_ok() {
        LLMProvider::XAI
    } else if env::var("PERPLEXITY_API_KEY").is_ok() {
        LLMProvider::Perplexity
    } else {
        LLMProvider::Local
    }
}