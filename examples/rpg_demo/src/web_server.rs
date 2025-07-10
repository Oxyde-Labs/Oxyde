use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod openai_service;
mod llm_service;
mod emotion_engine;
mod goal_system;
use openai_service::OpenAIService;
use llm_service::{LLMService, select_optimal_provider};
use emotion_engine::EmotionEngine;
use goal_system::GoalEngine;

#[tokio::main]
async fn main() {
    let port = "5000";
    let addr = format!("0.0.0.0:{}", port);
    
    // Initialize OpenAI service
    let openai_service = match OpenAIService::new() {
        Ok(service) => Arc::new(service),
        Err(e) => {
            eprintln!("Failed to initialize OpenAI service: {}", e);
            std::process::exit(1);
        }
    };
    
    // Store conversation history for each NPC
    let conversation_history: Arc<Mutex<HashMap<String, Vec<String>>>> = Arc::new(Mutex::new(HashMap::new()));
    
    // Initialize emotion engine for dynamic NPC personalities
    let emotion_engine: Arc<Mutex<EmotionEngine>> = Arc::new(Mutex::new(EmotionEngine::new()));
    
    // Initialize goal engine for autonomous NPC behavior
    let goal_engine: Arc<Mutex<GoalEngine>> = Arc::new(Mutex::new(GoalEngine::new()));

    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind to {}: {}", addr, e);
            std::process::exit(1);
        }
    };

    println!("Web server running on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let openai_service = Arc::clone(&openai_service);
                let conversation_history = Arc::clone(&conversation_history);
                let emotion_engine = Arc::clone(&emotion_engine);
                let goal_engine = Arc::clone(&goal_engine);
                thread::spawn(move || {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async {
                        if let Err(e) = handle_connection(stream, openai_service, conversation_history, emotion_engine, goal_engine).await {
                            eprintln!("Error handling connection: {}", e);
                        }
                    });
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    openai_service: Arc<OpenAIService>,
    conversation_history: Arc<Mutex<HashMap<String, Vec<String>>>>,
    emotion_engine: Arc<Mutex<EmotionEngine>>,
    goal_engine: Arc<Mutex<GoalEngine>>
) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    
    let request = String::from_utf8_lossy(&buffer);
    
    // Handle AI chat requests
    if request.contains("POST /ai-chat") {
        return handle_ai_chat(stream, &request, openai_service, conversation_history, emotion_engine, goal_engine).await;
    }

    let html = r#"<!DOCTYPE html>
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
        
        // Memory for each NPC
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
                
                // Show initial greeting without sending a message
                dialogue.innerHTML = `<strong>${nearbyNPC.name}:</strong><br>"${nearbyNPC.greeting}"<br><em>Type your message below and press Enter or click Send.</em>`;
                dialogue.style.background = '#fff3cd';
                
                // Focus on the input box
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
            
            // Add player message to dialogue
            const currentDialogue = dialogue.innerHTML;
            dialogue.innerHTML = currentDialogue + `<br><strong>You:</strong> ${playerMessage}<br><strong>${currentNPC.name}:</strong> Thinking...`;
            dialogue.style.background = '#fff3cd';
            
            try {
                // Get AI response from foundation model
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
                    
                    // Handle conversation end if NPC is too emotional
                    if (data.conversation_ended) {
                        const updatedDialogue = dialogue.innerHTML.replace('Thinking...', `"${data.response}"`);
                        dialogue.innerHTML = updatedDialogue + `<br><em>The conversation has ended.</em>`;
                        endConversation();
                        return;
                    }
                    
                    // Update dialogue with AI response
                    const updatedDialogue = dialogue.innerHTML.replace('Thinking...', `"${data.response}"`);
                    dialogue.innerHTML = updatedDialogue;
                    
                    // Store in memory
                    npcMemories[currentNPC.id].push(`Player: ${playerMessage}`);
                    npcMemories[currentNPC.id].push(`${currentNPC.name}: ${data.response}`);
                    
                    // Keep memory manageable
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
            // Handle Enter key for sending messages
            if (e.key === 'Enter' && currentNPC && document.activeElement === document.getElementById('messageInput')) {
                e.preventDefault();
                sendMessage();
                return;
            }
            
            // Don't process movement keys if typing in chat
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
        
        // Initialize
        updatePosition();
    </script>
</body>
</html>"#;

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        html.len(),
        html
    );

    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

async fn handle_ai_chat(
    mut stream: TcpStream,
    request: &str,
    openai_service: Arc<OpenAIService>,
    conversation_history: Arc<Mutex<HashMap<String, Vec<String>>>>,
    emotion_engine: Arc<Mutex<EmotionEngine>>,
    goal_engine: Arc<Mutex<GoalEngine>>
) -> std::io::Result<()> {
    // Extract JSON body from request
    if let Some(body_start) = request.find("\r\n\r\n") {
        let body = &request[body_start + 4..];
        let body = body.trim_end_matches('\0');
        
        if let Ok(chat_request) = serde_json::from_str::<serde_json::Value>(body) {
            let npc_id = chat_request["npc_id"].as_str().unwrap_or("");
            let npc_name = chat_request["npc_name"].as_str().unwrap_or("");
            let npc_role = chat_request["npc_role"].as_str().unwrap_or("");
            let player_message = chat_request["player_message"].as_str().unwrap_or("");
            
            // Get conversation history
            let history = {
                let hist = conversation_history.lock().unwrap();
                hist.get(npc_id).cloned().unwrap_or_default()
            };
            
            // Initialize NPC goals if this is their first interaction
            {
                let mut goal_engine = goal_engine.lock().unwrap();
                if goal_engine.get_npc_goals(npc_id).is_empty() {
                    goal_engine.initialize_npc_goals(npc_id, npc_role);
                }
            }

            // Update NPC emotional state based on player interaction
            let (emotional_modifier, response_style) = {
                let mut emotion_engine = emotion_engine.lock().unwrap();
                emotion_engine.update_npc_emotion(npc_id, npc_role, player_message)
            };
            
            // Check if NPC wants to end conversation due to emotional state
            let should_end = {
                let emotion_engine = emotion_engine.lock().unwrap();
                emotion_engine.should_npc_end_conversation(npc_id)
            };
            
            if should_end {
                let end_response = match npc_role {
                    "Guard" => "I have more important duties to attend to.",
                    "Merchant" => "I need to focus on my business now.",
                    _ => "I need to be going now.",
                };
                
                let response_json = serde_json::json!({
                    "response": end_response,
                    "conversation_ended": true
                });
                
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
                    response_json.to_string().len(),
                    response_json
                );
                
                stream.write_all(response.as_bytes())?;
                return Ok(());
            }
            
            // Smart provider selection based on message complexity
            let provider = select_optimal_provider(player_message);
            let llm_service = match LLMService::new(provider) {
                Ok(service) => service,
                Err(e) => {
                    eprintln!("Failed to create LLM service: {}", e);
                    let error_response = serde_json::json!({"error": "LLM service unavailable"});
                    let response = format!(
                        "HTTP/1.1 500 Internal Server Error\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
                        error_response.to_string().len(),
                        error_response
                    );
                    stream.write_all(response.as_bytes())?;
                    return Ok(());
                }
            };
            
            println!("Using {} for NPC response (fast inference: {}) - Emotional state: {}", 
                llm_service.get_provider_name(), 
                llm_service.supports_fast_inference(),
                emotional_modifier);
            
            // Get NPC's current goals and motivation for context
            let (goal_context, story_event) = {
                let mut goal_engine = goal_engine.lock().unwrap();
                let emotion_engine = emotion_engine.lock().unwrap();
                let emotional_state = emotion_engine.npc_emotions.get(npc_id).cloned()
                    .unwrap_or_else(|| crate::emotion_engine::EmotionalState::new_for_role(npc_role));
                
                let goal_context = goal_engine.get_contextual_motivation(npc_id, &emotional_state);
                let story_event = goal_engine.generate_story_event(npc_id, player_message);
                
                (goal_context, story_event)
            };

            // Generate AI response with emotional and goal-driven context
            match llm_service.generate_goal_driven_response(npc_name, npc_role, player_message, &history, &emotional_modifier, &response_style.get_style_prompt(), &goal_context).await {
                Ok(ai_response) => {
                    // Update conversation history
                    {
                        let mut hist = conversation_history.lock().unwrap();
                        let npc_history = hist.entry(npc_id.to_string()).or_insert_with(Vec::new);
                        npc_history.push(format!("Player: {}", player_message));
                        npc_history.push(format!("{}: {}", npc_name, ai_response));
                        
                        // Keep memory manageable
                        if npc_history.len() > 10 {
                            *npc_history = npc_history.iter().rev().take(10).rev().cloned().collect();
                        }
                    }

                    // Update goal progress based on the interaction
                    {
                        let mut goal_engine = goal_engine.lock().unwrap();
                        goal_engine.update_goal_from_interaction(npc_id, player_message, &ai_response);
                    }
                    
                    let response_json = serde_json::json!({
                        "response": ai_response
                    });
                    
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
                        response_json.to_string().len(),
                        response_json
                    );
                    
                    stream.write_all(response.as_bytes())?;
                }
                Err(_) => {
                    let error_response = serde_json::json!({
                        "error": "Failed to generate AI response"
                    });
                    
                    let response = format!(
                        "HTTP/1.1 500 Internal Server Error\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
                        error_response.to_string().len(),
                        error_response
                    );
                    
                    stream.write_all(response.as_bytes())?;
                }
            }
        }
    }
    
    stream.flush()?;
    Ok(())
}
