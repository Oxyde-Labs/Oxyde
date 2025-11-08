use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod emotion_engine;
mod goal_system;
mod llm_service;
mod openai_service;

use emotion_engine::EmotionEngine;
use goal_system::GoalEngine;
use llm_service::{select_optimal_provider, LLMService};
use openai_service::OpenAIService;

#[derive(Clone)]
struct AppState {
    _openai_service: Arc<OpenAIService>,
    conversation_history: Arc<Mutex<HashMap<String, Vec<String>>>>,
    emotion_engine: Arc<Mutex<EmotionEngine>>,
    goal_engine: Arc<Mutex<GoalEngine>>,
}

#[derive(Debug, Deserialize)]
struct ChatRequest {
    npc_id: String,
    npc_name: String,
    npc_role: String,
    player_message: String,
    #[allow(dead_code)]
    history: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ChatResponse {
    response: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_ended: Option<bool>,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let openai_service = match OpenAIService::new() {
        Ok(service) => Arc::new(service),
        Err(e) => {
            eprintln!("Failed to initialize OpenAI service: {}", e);
            std::process::exit(1);
        }
    };

    let app_state = AppState {
        _openai_service: openai_service,
        conversation_history: Arc::new(Mutex::new(HashMap::new())),
        emotion_engine: Arc::new(Mutex::new(EmotionEngine::new())),
        goal_engine: Arc::new(Mutex::new(GoalEngine::new())),
    };

    let app = Router::new()
        .route("/", get(serve_html))
        .route("/ai-chat", post(handle_ai_chat))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let addr = "0.0.0.0:5000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    tracing::info!("Web server running on {}", addr);
    
    axum::serve(listener, app).await.unwrap();
}

async fn serve_html() -> Html<&'static str> {
    Html(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Oxyde RPG Demo</title>
    <style>
        body { 
            font-family: Arial, sans-serif; 
            margin: 0; 
            padding: 20px;
            background: #f0f0f0;
        }
        .game-container {
            max-width: 800px;
            margin: 0 auto;
            background: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        .game-view {
            border: 1px solid #ccc;
            height: 400px;
            margin: 20px 0;
            position: relative;
            background: #fff;
        }
        .controls {
            margin-top: 20px;
            padding: 10px;
            background: #eee;
            border-radius: 4px;
        }
        #player {
            width: 20px;
            height: 20px;
            background: blue;
            position: absolute;
            border-radius: 50%;
        }
        .npc {
            width: 18px;
            height: 18px;
            position: absolute;
            border-radius: 50%;
            border: 2px solid #333;
        }
        .merchant { background: gold; }
        .guard { background: red; }
        .villager { background: green; }
    </style>
</head>
<body>
    <div class="game-container">
        <h1>Oxyde RPG Demo</h1>
        <div class="game-view" id="gameView">
            <div id="player"></div>
            <div id="marcus" class="npc merchant" style="left: 150px; top: 100px;" title="Marcus the Merchant"></div>
            <div id="gareth" class="npc guard" style="left: 300px; top: 250px;" title="Gareth the Guard"></div>
            <div id="velma" class="npc villager" style="left: 500px; top: 180px;" title="Velma the Villager"></div>
        </div>
        <div class="controls">
            <p>Use WASD keys to move your character</p>
            <p>Press E to interact with NPCs when nearby</p>
            <div id="dialogue" style="margin-top: 10px; min-height: 60px; padding: 10px; border: 1px solid #ccc; border-radius: 4px; background: #f9f9f9;"></div>
            <div id="chatInput" style="margin-top: 10px; display: none;">
                <input type="text" id="messageInput" placeholder="Type your message..." style="width: 70%; padding: 5px;">
                <button onclick="sendMessage()" style="padding: 5px 10px;">Send</button>
                <button onclick="endConversation()" style="padding: 5px 10px;">End Chat</button>
            </div>
        </div>
    </div>
    <script>
        const player = document.getElementById('player');
        const dialogue = document.getElementById('dialogue');
        let x = 200, y = 200;
        
        const npcs = [
            { id: 'marcus', name: 'Marcus the Merchant', x: 150, y: 100, role: 'merchant',
              greeting: "Welcome to my shop! I have the finest goods in town." },
            { id: 'gareth', name: 'Gareth the Guard', x: 300, y: 250, role: 'guard',
              greeting: "Halt! State your business in this town." },
            { id: 'velma', name: 'Velma the Villager', x: 500, y: 180, role: 'villager',
              greeting: "Oh hello there! Lovely weather we're having, isn't it?" }
        ];
        
        const npcMemories = {
            'marcus': [],
            'gareth': [],
            'velma': []
        };
        
        let currentNPC = null;
        
        function updatePosition() {
            player.style.left = x + 'px';
            player.style.top = y + 'px';
            checkNearbyNPCs();
        }
        
        function checkNearbyNPCs() {
            let nearestNPC = null;
            let nearestDistance = Infinity;
            
            npcs.forEach(npc => {
                const distance = Math.sqrt(Math.pow(x - npc.x, 2) + Math.pow(y - npc.y, 2));
                if (distance < nearestDistance && distance < 50) {
                    nearestDistance = distance;
                    nearestNPC = npc;
                }
            });
            
            if (nearestNPC) {
                dialogue.innerHTML = `<strong>Near ${nearestNPC.name}</strong><br>Press E to interact`;
                dialogue.style.background = '#e6ffe6';
            } else {
                dialogue.innerHTML = 'Move around with WASD. Get close to NPCs to interact.';
                dialogue.style.background = '#f9f9f9';
            }
        }
        
        async function interactWithNPC() {
            const nearbyNPC = npcs.find(npc => {
                const distance = Math.sqrt(Math.pow(x - npc.x, 2) + Math.pow(y - npc.y, 2));
                return distance < 50;
            });
            
            if (nearbyNPC) {
                currentNPC = nearbyNPC;
                document.getElementById('chatInput').style.display = 'block';
                
                dialogue.innerHTML = `<strong>${nearbyNPC.name}:</strong><br>"${nearbyNPC.greeting}"<br><em>Type your message below and press Enter or click Send.</em>`;
                dialogue.style.background = '#fff3cd';
                
                document.getElementById('messageInput').focus();
            }
        }
        
        async function sendMessage() {
            const messageInput = document.getElementById('messageInput');
            const message = messageInput.value.trim();
            
            if (message && currentNPC) {
                messageInput.value = '';
                await sendAIMessage(message);
            }
        }
        
        async function sendAIMessage(playerMessage) {
            if (!currentNPC) return;
            
            const currentDialogue = dialogue.innerHTML;
            dialogue.innerHTML = currentDialogue + `<br><strong>You:</strong> ${playerMessage}<br><strong>${currentNPC.name}:</strong> Thinking...`;
            dialogue.style.background = '#fff3cd';
            
            try {
                const response = await fetch('/ai-chat', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        npc_id: currentNPC.id,
                        npc_name: currentNPC.name,
                        npc_role: currentNPC.role,
                        player_message: playerMessage,
                        history: npcMemories[currentNPC.id]
                    })
                });
                
                console.log('AI response status:', response.status);
                
                if (response.ok) {
                    const data = await response.json();
                    console.log('AI response data:', data);
                    
                    if (data.conversation_ended) {
                        const updatedDialogue = dialogue.innerHTML.replace('Thinking...', `"${data.response}"`);
                        dialogue.innerHTML = updatedDialogue + `<br><em>The conversation has ended.</em>`;
                        endConversation();
                        return;
                    }
                    
                    const updatedDialogue = dialogue.innerHTML.replace('Thinking...', `"${data.response}"`);
                    dialogue.innerHTML = updatedDialogue;
                    
                    npcMemories[currentNPC.id].push(`Player: ${playerMessage}`);
                    npcMemories[currentNPC.id].push(`${currentNPC.name}: ${data.response}`);
                    
                    if (npcMemories[currentNPC.id].length > 10) {
                        npcMemories[currentNPC.id] = npcMemories[currentNPC.id].slice(-10);
                    }
                } else {
                    console.error('AI response failed:', response.status, response.statusText);
                    const errorData = await response.text();
                    console.error('Error details:', errorData);
                    const updatedDialogue = dialogue.innerHTML.replace('Thinking...', `"${currentNPC.greeting}"`);
                    dialogue.innerHTML = updatedDialogue;
                }
            } catch (error) {
                console.error('AI chat error:', error);
                const updatedDialogue = dialogue.innerHTML.replace('Thinking...', `"I'm having trouble speaking right now."`);
                dialogue.innerHTML = updatedDialogue;
            }
        }
        
        function endConversation() {
            currentNPC = null;
            document.getElementById('chatInput').style.display = 'none';
            dialogue.innerHTML = 'Move around with WASD. Get close to NPCs to interact.';
            dialogue.style.background = '#f9f9f9';
        }
        
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Enter' && currentNPC && document.activeElement === document.getElementById('messageInput')) {
                e.preventDefault();
                sendMessage();
                return;
            }
            
            if (document.activeElement === document.getElementById('messageInput')) {
                return;
            }
            
            switch(e.key.toLowerCase()) {
                case 'w': y = Math.max(0, y - 10); break;
                case 's': y = Math.min(380, y + 10); break;
                case 'a': x = Math.max(0, x - 10); break;
                case 'd': x = Math.min(780, x + 10); break;
                case 'e': interactWithNPC(); return;
            }
            updatePosition();
        });
        
        updatePosition();
    </script>
</body>
</html>"#)
}

async fn handle_ai_chat(
    State(state): State<AppState>,
    Json(req): Json<ChatRequest>,
) -> Response {
    let history = {
        let hist = state.conversation_history.lock().unwrap();
        hist.get(&req.npc_id).cloned().unwrap_or_default()
    };

    {
        let mut goal_engine = state.goal_engine.lock().unwrap();
        if goal_engine.get_npc_goals(&req.npc_id).is_empty() {
            goal_engine.initialize_npc_goals(&req.npc_id, &req.npc_role);
        }
    }

    let (emotional_modifier, response_style) = {
        let mut emotion_engine = state.emotion_engine.lock().unwrap();
        emotion_engine.update_npc_emotion(&req.npc_id, &req.npc_role, &req.player_message)
    };

    let should_end = {
        let emotion_engine = state.emotion_engine.lock().unwrap();
        emotion_engine.should_npc_end_conversation(&req.npc_id)
    };

    if should_end {
        let end_response = match req.npc_role.as_str() {
            "guard" => "I have more important duties to attend to.",
            "merchant" => "I need to focus on my business now.",
            _ => "I need to be going now.",
        };

        return (
            StatusCode::OK,
            Json(ChatResponse {
                response: end_response.to_string(),
                conversation_ended: Some(true),
            }),
        )
            .into_response();
    }

    let provider = select_optimal_provider(&req.player_message);
    let llm_service = match LLMService::new(provider) {
        Ok(service) => service,
        Err(e) => {
            tracing::error!("Failed to create LLM service: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "LLM service unavailable".to_string(),
                }),
            )
                .into_response();
        }
    };

    tracing::info!(
        "Using {} for NPC response (fast inference: {}) - Emotional state: {}",
        llm_service.get_provider_name(),
        llm_service.supports_fast_inference(),
        emotional_modifier
    );

    let goal_context = {
        let mut goal_engine = state.goal_engine.lock().unwrap();
        let emotion_engine = state.emotion_engine.lock().unwrap();
        let emotional_state = emotion_engine
            .npc_emotions
            .get(&req.npc_id)
            .cloned()
            .unwrap_or_else(|| emotion_engine::EmotionalState::new_for_role(&req.npc_role));

        let goal_context =
            goal_engine.get_contextual_motivation(&req.npc_id, &emotional_state);
        let _story_event = goal_engine.generate_story_event(&req.npc_id, &req.player_message);

        goal_context
    };

    match llm_service
        .generate_goal_driven_response(
            &req.npc_name,
            &req.npc_role,
            &req.player_message,
            &history,
            &emotional_modifier,
            &response_style.get_style_prompt(),
            &goal_context,
        )
        .await
    {
        Ok(ai_response) => {
            {
                let mut hist = state.conversation_history.lock().unwrap();
                let npc_history = hist.entry(req.npc_id.clone()).or_insert_with(Vec::new);
                npc_history.push(format!("Player: {}", req.player_message));
                npc_history.push(format!("{}: {}", req.npc_name, ai_response));

                if npc_history.len() > 10 {
                    *npc_history = npc_history.iter().rev().take(10).rev().cloned().collect();
                }
            }

            {
                let mut goal_engine = state.goal_engine.lock().unwrap();
                goal_engine.update_goal_from_interaction(
                    &req.npc_id,
                    &req.player_message,
                    &ai_response,
                );
            }

            (
                StatusCode::OK,
                Json(ChatResponse {
                    response: ai_response,
                    conversation_ended: None,
                }),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("Failed to generate AI response: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to generate AI response".to_string(),
                }),
            )
                .into_response()
        }
    }
}
