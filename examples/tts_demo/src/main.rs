use oxyde::audio::{AudioFormat, EmotionalState, TTSConfig, TTSProvider};
use oxyde::config::{AgentPersonality, InferenceConfig, MemoryConfig};
use oxyde::{Agent, AgentConfig};

use oxyde::oxyde_game::behavior::{DialogueBehavior, GreetingBehavior};
use std::collections::HashMap;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up your ElevenLabs API key
    std::env::set_var(
        "ELEVENLABS_API_KEY",
        "sk_ca0c02fef5b161403e1bfbfa1f6b6561aa6586c11416099b", //or set this in your environment variables and remove this line
    );

    // Configure TTS settings
    let tts_config = TTSConfig {
        default_provider: TTSProvider::ElevenLabs,
        cache_enabled: true,
        cache_max_size_mb: 50,
        voice_speed: 1.0,
        voice_pitch: 1.0,
        enable_ssml: true,
        output_format: AudioFormat::MP3,
    };

    // Create agent configuration
    let agent_config = AgentConfig {
        agent: AgentPersonality {
            name: "Innkeeper Tom".to_string(),
            role: "Friendly tavern keeper".to_string(),
            backstory: vec![
                "Tom has run the Golden Griffin tavern for 20 years".to_string(),
                "He knows all the local gossip and loves chatting with travelers".to_string(),
                "Tom is cheerful but becomes protective when trouble brews".to_string(),
            ],
            knowledge: vec![
                "Local rumors and tavern news".to_string(),
                "Information about rooms and meals".to_string(),
                "Stories about local adventures".to_string(),
            ],
        },
        memory: MemoryConfig::default(),
        inference: InferenceConfig::default(),
        behavior: HashMap::new(),
        tts: Some(tts_config), // Enable TTS
        prompts: None
    };

    // Create agent with TTS enabled
    let agent = Agent::new_with_tts(agent_config);

    // Add behaviors like the guard does

    // Create greeting behavior for Tom
    let greeting_behavior = GreetingBehavior::new(
            "Welcome to the Golden Griffin! I'm Tom, your friendly innkeeper."        
    );

    // Create dialogue behavior for Tom
    let mut tavern_topics = std::collections::HashMap::new();
    tavern_topics.insert(
        "room".to_string(),
        vec![
            "Our rooms are clean and comfortable. Two gold pieces per night.".to_string(),
            "I've got a nice room upstairs overlooking the courtyard.".to_string(),
        ],
    );
    tavern_topics.insert(
        "ale".to_string(),
        vec![
            "Our ale is the finest in three kingdoms! Brewed right here.".to_string(),
            "Try our special brew - it'll put hair on your chest!".to_string(),
        ],
    );

    let dialogue_behavior = DialogueBehavior::new(
        tavern_topics,
        vec![
            "What can I help you with today?".to_string(),
            "Let me know if you need anything!".to_string(),
        ],
    );

    // Add behaviors to agent
    agent.add_behavior(greeting_behavior).await;
    agent.add_behavior(dialogue_behavior).await;

    // Start the agent
    agent.start().await?;

    println!("Innkeeper Tom is ready to chat!");
    println!(" Type your message (or 'quit' to exit):");
    println!(" Audio files will be saved as 'response_X.mp3'\n");

    let mut response_count = 0;

    // Interactive loop
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            break;
        }

        if input.is_empty() {
            continue;
        }

        println!(" Tom is thinking...");

        // Generate response
        match agent.process_input(input).await {
            Ok(response) => {
                println!("Tom says: \"{}\"", response);

                // Create emotional state based on response content
                let emotions = create_emotions_for_response(&response, input);

                // Generate speech with current emotions
                match agent.speak(&response, &emotions, 0.5).await {
                    Ok(audio_data) => {
                        response_count += 1;
                        let filename = format!("response_{}.mp3", response_count);

                        // Save audio to file
                        std::fs::write(&filename, &audio_data.data)?;

                        println!("🔊 Audio saved to: {}", filename);
                        println!(
                            " Audio info: {} bytes, {}ms duration",
                            audio_data.size_bytes(),
                            audio_data.duration_ms
                        );
                        println!(" Emotions: {}", format_emotions(&emotions));
                    }
                    Err(e) => {
                        println!("  Speech generation failed: {}", e);
                        println!(" (Text response still available above)");
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        println!(); // Add spacing
    }

    println!("Thanks for chatting with Tom!");
    Ok(())
}

/// Create emotional state based on response content and user input
fn create_emotions_for_response(response: &str, user_input: &str) -> EmotionalState {
    let mut emotions = EmotionalState::neutral();

    let response_lower = response.to_lowercase();
    let input_lower = user_input.to_lowercase();

    // Happiness triggers
    if response_lower.contains("welcome")
        || response_lower.contains("great")
        || response_lower.contains("wonderful")
        || response_lower.contains("excellent")
    {
        emotions.happiness = 0.7;
    }

    // Curiosity triggers
    if response_lower.contains("tell me")
        || response_lower.contains("interesting")
        || response_lower.contains("what")
        || response_lower.contains("how")
    {
        emotions.curiosity = 0.6;
    }

    // Energy based on excitement
    if response_lower.contains("!") || input_lower.contains("adventure") {
        emotions.energy = 0.5;
    }

    // Trust building
    if input_lower.contains("thank") || response_lower.contains("help") {
        emotions.trust = 0.4;
    }

    // Fear/concern
    if input_lower.contains("danger") || input_lower.contains("trouble") {
        emotions.fear = 0.3;
        emotions.trust = 0.2;
    }

    emotions.clamp(); // Ensure values are in valid range
    emotions
}

/// Format emotions for display
fn format_emotions(emotions: &EmotionalState) -> String {
    let (dominant, level) = emotions.dominant_emotion();
    let intensity = emotions.intensity();

    format!(
        "{} ({:.1}/1.0), Overall intensity: {:.1}/1.0",
        dominant, level, intensity
    )
}
