use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[allow(dead_code)]
pub struct OpenAIService {
    api_key: String,
    client: reqwest::Client,
}

impl OpenAIService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| "OPENAI_API_KEY environment variable not found")?;
        
        Ok(Self {
            api_key,
            client: reqwest::Client::new(),
        })
    }

    #[allow(dead_code)]
    pub async fn generate_npc_response(
        &self,
        npc_name: &str,
        npc_role: &str,
        player_message: &str,
        conversation_history: &[String],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let personality = self.get_npc_personality(npc_role);
        
        let mut messages = vec![
            OpenAIMessage {
                role: "system".to_string(),
                content: format!(
                    "You are {}, a {} in a fantasy RPG world. {}. Keep responses short (1-2 sentences), stay in character, and be engaging. Remember previous conversations.",
                    npc_name, npc_role, personality
                ),
            }
        ];

        // Add conversation history
        for (i, msg) in conversation_history.iter().enumerate() {
            let role = if i % 2 == 0 { "user" } else { "assistant" };
            messages.push(OpenAIMessage {
                role: role.to_string(),
                content: msg.clone(),
            });
        }

        // Add current player message
        messages.push(OpenAIMessage {
            role: "user".to_string(),
            content: player_message.to_string(),
        });

        let request = OpenAIRequest {
            model: "gpt-4o".to_string(), // the newest OpenAI model is "gpt-4o" which was released May 13, 2024. do not change this unless explicitly requested by the user
            messages,
            max_tokens: 150,
            temperature: 0.8,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("OpenAI API error: {}", response.status()).into());
        }

        let openai_response: OpenAIResponse = response.json().await?;
        
        Ok(openai_response
            .choices
            .first()
            .ok_or("No response from OpenAI")?
            .message
            .content
            .clone())
    }

    #[allow(dead_code)]
    fn get_npc_personality(&self, role: &str) -> &str {
        match role.to_lowercase().as_str() {
            "merchant" => "You're friendly and business-minded, always looking to make a deal. You know about valuable items and trade routes.",
            "guard" => "You're serious and protective, focused on keeping the town safe. You're suspicious of strangers but fair.",
            "villager" => "You're cheerful and gossipy, knowing all the local news and rumors. You're helpful but sometimes chatty.",
            _ => "You're a helpful NPC who assists adventurers."
        }
    }
}