//! Configuration-based Agent Creation Demo
//!
//! This example demonstrates how to create and configure Oxyde agents using
//! configuration files in different formats (JSON, YAML, TOML).
//!
//! Run with: cargo run --example config_demo

use oxyde::{config::InferenceConfig, Agent, AgentConfig, PromptConfig};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║        Oxyde SDK - Configuration Demo                     ║");
    println!("║        Creating Agents from Config Files                  ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    loop {
        println!("\n Available Demos:");
        println!("  1. Create agent from JSON config");
        println!("  2. Create agent from YAML config");
        println!("  3. Create agent from TOML config");
        println!("  4. Create agent with custom prompts");
        println!("  5. Create agent with bundled defaults");
        println!("  6. Interactive chat with configured agent");
        println!("  7. Compare different agent personalities");
        println!("  0. Exit");

        print!("\n Select option: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => demo_json_config().await?,
            "2" => demo_yaml_config().await?,
            "3" => demo_toml_config().await?,
            "4" => demo_custom_prompts().await?,
            "5" => demo_bundled_defaults().await?,
            "6" => demo_interactive_chat().await?,
            "7" => demo_compare_personalities().await?,
            "0" => {
                println!("\n Goodbye!");
                break;
            }
            _ => println!(" Invalid option. Please try again."),
        }
    }

    Ok(())
}

/// Demo 1: Load agent from JSON configuration
async fn demo_json_config() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    println!("\n Demo 1: Loading Agent from JSON Config");
    println!("─────────────────────────────────────────\n");

    let config_path = "examples/config_demo/configs/merchant_agent.json";
    println!(" Loading config from: {}", config_path);

    let mut config = AgentConfig::from_file(config_path)?;
    config.inference = InferenceConfig::default();
    config.inference.api_key =
        Some(std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set in .env"));
    println!(" Config loaded successfully!");
    println!("   Agent Name: {}", config.agent.name);
    println!("   Agent Role: {}", config.agent.role);
    println!("   Backstory items: {}", config.agent.backstory.len());
    println!("   Knowledge items: {}", config.agent.knowledge.len());

    let agent = Agent::new(config);
    agent.start().await?;

    println!("\n Testing agent response...");
    let response = agent.process_input("Hello! What do you sell?").await?;
    println!(" {}: {}\n", agent.name(), response);

    agent.stop().await?;

    Ok(())
}

/// Demo 2: Load agent from YAML configuration
async fn demo_yaml_config() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n Demo 2: Loading Agent from YAML Config");
    println!("─────────────────────────────────────────\n");

    let config_path = "examples/config_demo/configs/guard_agent.yaml";
    println!(" Loading config from: {}", config_path);

    let mut config = AgentConfig::from_file(config_path)?;
    config.inference = InferenceConfig::default();
    config.inference.api_key =
        Some(std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set in .env"));

    println!(" Config loaded successfully!");
    println!("   Agent Name: {}", config.agent.name);
    println!("   Agent Role: {}", config.agent.role);

    let agent = Agent::new(config);
    agent.start().await?;

    println!("\n Testing agent response...");
    let response = agent
        .process_input("I need to report a disturbance.")
        .await?;
    println!(" {}: {}\n", agent.name(), response);

    agent.stop().await?;

    Ok(())
}

/// Demo 3: Load agent from TOML configuration
async fn demo_toml_config() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n Demo 3: Loading Agent from TOML Config");
    println!("─────────────────────────────────────────\n");

    let config_path = "examples/config_demo/configs/villager_agent.toml";
    println!(" Loading config from: {}", config_path);

    let mut config = AgentConfig::from_file(config_path)?;

    config.inference = InferenceConfig::default();
    config.inference.api_key =
        Some(std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set in .env"));

    println!(" Config loaded successfully!");
    println!("   Agent Name: {}", config.agent.name);
    println!("   Agent Role: {}", config.agent.role);

    let agent = Agent::new(config);
    agent.start().await?;

    println!("\n Testing agent response...");
    let response = agent.process_input("What's the latest news?").await?;
    println!(" {}: {}\n", agent.name(), response);

    agent.stop().await?;

    Ok(())
}

/// Demo 4: Create agent with custom prompt templates
async fn demo_custom_prompts() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n Demo 4: Agent with Custom Prompts");
    println!("─────────────────────────────────────────\n");

    let config_path = "examples/config_demo/configs/merchant_agent.json";
    let prompts_path = "examples/config_demo/configs/prompts/custom_prompts.toml";

    println!(" Loading config from: {}", config_path);
    println!(" Loading prompts from: {}", prompts_path);

    let mut config = AgentConfig::from_file(config_path)?;

    let custom_prompts = PromptConfig::from_file(prompts_path)?;
    config.prompts = Some(custom_prompts);

    config.inference = InferenceConfig::default();
    config.inference.api_key =
        Some(std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set in .env"));

    println!(" Config and custom prompts loaded!");

    let agent = Agent::new(config);
    agent.start().await?;

    println!("\n Testing with custom prompt style...");
    let response = agent.process_input("Greetings!").await?;
    println!(" {}: {}\n", agent.name(), response);

    agent.stop().await?;

    Ok(())
}

/// Demo 5: Create agent using bundled default prompts
async fn demo_bundled_defaults() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n Demo 5: Agent with Bundled Default Prompts");
    println!("─────────────────────────────────────────\n");

    println!(" Using bundled default prompt templates");

    // Load configuration without prompts field
    let config_path = "examples/config_demo/configs/merchant_agent.json";
    let mut config = AgentConfig::from_file(config_path)?;

    // Set prompts to None to use bundled defaults
    config.prompts = None;

    config.inference = InferenceConfig::default();
    config.inference.api_key =
        Some(std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set in .env"));

    println!(" Config loaded, will use bundled defaults");

    // Create agent - will automatically use bundled defaults
    let agent = Agent::new(config);
    agent.start().await?;

    println!("\n Testing with bundled prompt style...");
    let response = agent
        .process_input("What can you tell me about your wares?")
        .await?;
    println!(" {}: {}\n", agent.name(), response);

    agent.stop().await?;

    Ok(())
}

/// Demo 6: Interactive chat session with configured agent
async fn demo_interactive_chat() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n Demo 6: Interactive Chat with Agent");
    println!("─────────────────────────────────────────\n");

    println!("Select an agent to chat with:");
    println!("  1. Marcus the Merchant (JSON)");
    println!("  2. Captain Roderick (YAML)");
    println!("  3. Elara the Villager (TOML)");

    print!("\n Choose (1-3): ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    let config_path = match choice.trim() {
        "1" => "examples/config_demo/configs/merchant_agent.json",
        "2" => "examples/config_demo/configs/guard_agent.yaml",
        "3" => "examples/config_demo/configs/villager_agent.toml",
        _ => {
            println!(" Invalid choice, defaulting to merchant");
            "examples/config_demo/configs/merchant_agent.json"
        }
    };

    let mut config = AgentConfig::from_file(config_path)?;
    config.inference = InferenceConfig::default();
    config.inference.api_key =Some(std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY must be set in .env"));

    let agent = Agent::new(config);
    agent.start().await?;

    println!("\n Agent '{}' is ready to chat!", agent.name());
    println!(" Type 'exit' or 'quit' to end the conversation\n");

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("\n Ending conversation...");
            break;
        }

        if input.is_empty() {
            continue;
        }

        match agent.process_input(input).await {
            Ok(response) => {
                println!("{}: {}\n", agent.name(), response);
            }
            Err(e) => {
                println!(" Error: {}\n", e);
            }
        }
    }

    agent.stop().await?;

    Ok(())
}

/// Demo 7: Compare different agent personalities
async fn demo_compare_personalities() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n Demo 7: Comparing Agent Personalities");
    println!("─────────────────────────────────────────\n");

    println!("Creating three different agents with distinct personalities...\n");

    // Create merchant
    let mut merchant_config =
        AgentConfig::from_file("examples/config_demo/configs/merchant_agent.json")?;
    merchant_config.inference = InferenceConfig::default();
    merchant_config.inference.api_key =Some(std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY must be set in .env"));

    let merchant = Agent::new(merchant_config);
    merchant.start().await?;

    // Create guard
    let mut guard_config = AgentConfig::from_file("examples/config_demo/configs/guard_agent.yaml")?;
    guard_config.inference = InferenceConfig::default();
    guard_config.inference.api_key =Some(std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY must be set in .env"));

    let guard = Agent::new(guard_config);
    guard.start().await?;

    // Create villager
    let mut villager_config =
        AgentConfig::from_file("examples/config_demo/configs/villager_agent.toml")?;

    villager_config.inference = InferenceConfig::default();
    villager_config.inference.api_key =Some(std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY must be set in .env"));

    let villager = Agent::new(villager_config);
    villager.start().await?;

    // Test same question with all three
    let question = "What do you think about the weather today?";

    println!(" Question: '{}'\n", question);

    println!(" {}:", merchant.name());
    let merchant_response = merchant.process_input(question).await?;
    println!("   {}\n", merchant_response);

    println!("  {}:", guard.name());
    let guard_response = guard.process_input(question).await?;
    println!("   {}\n", guard_response);

    println!("  {}:", villager.name());
    let villager_response = villager.process_input(question).await?;
    println!("   {}\n", villager_response);

    println!(" Notice how each agent responds differently based on their role!");

    merchant.stop().await?;
    guard.stop().await?;
    villager.stop().await?;

    Ok(())
}
