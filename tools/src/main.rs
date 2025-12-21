//! CLI Tool for Oxyde SDK
//!
//! This command-line tool allows developers to create, test, and deploy
//! AI-powered NPC agents for games.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::Duration;

use clap::{Parser, Subcommand};
use oxyde::agent::{Agent, AgentState};
use oxyde::config::{AgentConfig, BehaviorConfig, InferenceConfig, MemoryConfig};
use oxyde::oxyde_game::behavior::factory;
use oxyde::oxyde_game::intent::Intent;
use oxyde::{OxydeError, Result};
use tokio::time::sleep;

/// CLI arguments parser
#[derive(Parser)]
#[clap(author, version, about = "CLI tool for Oxyde SDK")]
struct Cli {
    /// Subcommand to execute
    #[clap(subcommand)]
    command: Commands,
    
    /// Enable verbose output
    #[clap(short, long)]
    verbose: bool,
}

/// Subcommands for the CLI
#[derive(Subcommand)]
enum Commands {
    /// Create a new agent configuration
    Create {
        /// Name of the agent
        #[clap(short, long)]
        name: String,
        
        /// Role of the agent
        #[clap(short, long)]
        role: String,
        
        /// Output file path
        #[clap(short, long, default_value = "agent.json")]
        output: String,
    },
    
    /// Deploy agents to a game scene
    Deploy {
        /// Path to agent configuration file(s)
        #[clap(short, long)]
        config: Vec<String>,
        
        /// Path to scene configuration
        #[clap(short, long)]
        scene: String,
        
        /// Engine to deploy to (unity, unreal, wasm)
        #[clap(short, long, default_value = "unity")]
        engine: String,
        
        /// Output directory for generated files
        #[clap(short, long, default_value = "output")]
        output: String,
    },
    
    /// Test an agent with interactive chat
    Test {
        /// Path to agent configuration file
        #[clap(short, long)]
        config: String,
        
        /// Use local inference only
        #[clap(long)]
        local_only: bool,
        
        /// Enable memory persistence
        #[clap(long)]
        persistent_memory: bool,
    },
    
    /// Convert an agent between formats
    Convert {
        /// Input configuration file
        #[clap(short, long)]
        input: String,
        
        /// Output format (json, yaml)
        #[clap(short, long, default_value = "json")]
        format: String,
        
        /// Output file path
        #[clap(short, long)]
        output: String,
    },
}

/// Run the CLI tool
#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Configure logging level
    let log_level = if cli.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    
    // Initialize logging
    env_logger::Builder::new()
        .filter_level(log_level)
        .init();
    
    // Initialize Oxyde SDK
    oxyde::init()?;
    
    // Process commands
    match cli.command {
        Commands::Create { name, role, output } => {
            create_agent_config(&name, &role, &output).await?;
        }
        Commands::Deploy { config, scene, engine, output } => {
            deploy_agents(&config, &scene, &engine, &output).await?;
        }
        Commands::Test { config, local_only, persistent_memory } => {
            test_agent(&config, local_only, persistent_memory).await?;
        }
        Commands::Convert { input, format, output } => {
            convert_agent_config(&input, &format, &output).await?;
        }
    }
    
    Ok(())
}

/// Create a new agent configuration file
async fn create_agent_config(name: &str, role: &str, output: &str) -> Result<()> {
    println!("Creating new agent configuration for '{}' as a '{}'...", name, role);
    
    // Create a basic agent configuration
    let agent_config = AgentConfig {
        agent: oxyde::config::AgentPersonality {
            name: name.to_string(),
            role: role.to_string(),
            backstory: vec![
                format!("A {} with a rich history", role),
                "Has lived in this area for many years".to_string(),
                "Knowledgeable about local customs and events".to_string(),
            ],
            knowledge: vec![
                format!("Expert knowledge about {}", role),
                "Familiar with the local area".to_string(),
                "Knows common greetings and customs".to_string(),
            ],
        },
        memory: MemoryConfig::default(),
        inference: InferenceConfig::default(),
        behavior: create_default_behaviors(),
<<<<<<< HEAD
        tts: None, //defaulting to none
        prompts: None,
=======
        tts: None,
        moderation: oxyde::config::ModerationConfig {
            enabled: false,
            ..Default::default()
        }
>>>>>>> main
    };
    
    // Determine output format
    let path = Path::new(output);
    let is_json = path.extension().map_or(true, |ext| ext == "json");
    
    // Write the configuration to file
    if is_json {
        let json = serde_json::to_string_pretty(&agent_config)?;
        fs::write(output, json)?;
    } else {
        let yaml = serde_yaml::to_string(&agent_config)?;
        fs::write(output, yaml)?;
    }
    
    println!("Created agent configuration at: {}", output);
    Ok(())
}

/// Create default behaviors for a new agent
fn create_default_behaviors() -> HashMap<String, BehaviorConfig> {
    let mut behaviors = HashMap::new();
    
    // Greeting behavior
    let greeting = BehaviorConfig {
        trigger: "proximity".to_string(),
        cooldown: 60,
        priority: 10,
        parameters: HashMap::new(),
    };
    behaviors.insert("greeting".to_string(), greeting);
    
    // Dialogue behavior
    let dialogue = BehaviorConfig {
        trigger: "chat".to_string(),
        cooldown: 0,
        priority: 20,
        parameters: HashMap::new(),
    };
    behaviors.insert("dialogue".to_string(), dialogue);
    
    // Movement behavior
    let movement = BehaviorConfig {
        trigger: "movement".to_string(),
        cooldown: 0,
        priority: 5,
        parameters: HashMap::new(),
    };
    behaviors.insert("movement".to_string(), movement);
    
    behaviors
}

/// Deploy agents to a game scene
async fn deploy_agents(
    configs: &[String],
    scene: &str,
    engine: &str,
    output: &str,
) -> Result<()> {
    println!("Deploying agents to scene: {}", scene);
    println!("Target engine: {}", engine);
    
    // Create output directory if it doesn't exist
    fs::create_dir_all(output)?;
    
    // Load scene configuration
    let scene_path = Path::new(scene);
    if !scene_path.exists() {
        return Err(OxydeError::CliError(format!("Scene file not found: {}", scene)));
    }
    
    let scene_config: serde_json::Value = serde_json::from_reader(fs::File::open(scene_path)?)?;
    
    // Load agent configurations
    let mut agents = Vec::new();
    for config_path in configs {
        println!("Loading agent from: {}", config_path);
        let config = AgentConfig::from_file(config_path)?;
        agents.push(config);
    }
    
    // Generate engine-specific files
    match engine.to_lowercase().as_str() {
        "unity" => deploy_unity_agents(&agents, &scene_config, output)?,
        "unreal" => deploy_unreal_agents(&agents, &scene_config, output)?,
        "wasm" => deploy_wasm_agents(&agents, &scene_config, output)?,
        _ => return Err(OxydeError::CliError(format!("Unsupported engine: {}", engine))),
    }
    
    println!("Deployment complete! Files generated in: {}", output);
    Ok(())
}

/// Deploy agents for Unity engine
fn deploy_unity_agents(
    agents: &[AgentConfig],
    scene_config: &serde_json::Value,
    output: &str,
) -> Result<()> {
    println!("Generating Unity-specific files...");
    
    // Create Unity-specific directories
    let scripts_dir = PathBuf::from(output).join("Scripts");
    let configs_dir = PathBuf::from(output).join("Resources/AgentConfigs");
    fs::create_dir_all(&scripts_dir)?;
    fs::create_dir_all(&configs_dir)?;
    
    // Generate agent manager script
    let manager_script = generate_unity_manager_script(agents);
    fs::write(scripts_dir.join("OxydeAgentManager.cs"), manager_script)?;
    
    // Generate agent controller scripts
    for (i, agent) in agents.iter().enumerate() {
        // Write agent configuration to Unity Resources folder
        let config_json = serde_json::to_string_pretty(agent)?;
        let config_filename = format!("agent_{}.json", i);
        fs::write(configs_dir.join(&config_filename), config_json)?;
        
        // Generate controller script
        let controller_script = generate_unity_agent_script(agent, &config_filename);
        let script_filename = format!("{}Controller.cs", agent.agent.name.replace(" ", ""));
        fs::write(scripts_dir.join(script_filename), controller_script)?;
    }
    
    // Generate demo scene setup script
    let scene_script = generate_unity_scene_script(agents, scene_config);
    fs::write(scripts_dir.join("OxydeSceneSetup.cs"), scene_script)?;
    
    println!("Generated Unity integration files in: {}", output);
    Ok(())
}

/// Generate Unity agent manager script
fn generate_unity_manager_script(agents: &[AgentConfig]) -> String {
    format!(
        r#"using UnityEngine;
using System.Collections.Generic;

namespace Oxyde.Unity
{{
    /// <summary>
    /// Manages all Oxyde AI agents in the scene
    /// </summary>
    public class OxydeAgentManager : MonoBehaviour
    {{
        // Singleton instance
        public static OxydeAgentManager Instance {{ get; private set; }}
        
        // List of all agents in the scene
        private List<OxydeAgent> agents = new List<OxydeAgent>();
        
        // Initialize the Oxyde SDK
        private void Awake()
        {{
            if (Instance == null)
            {{
                Instance = this;
                DontDestroyOnLoad(gameObject);
                Debug.Log("Initializing Oxyde Agent Manager");
                
                // Initialize native SDK
                OxydeUnity.Init();
            }}
            else
            {{
                Destroy(gameObject);
            }}
        }}
        
        // Register an agent with the manager
        public void RegisterAgent(OxydeAgent agent)
        {{
            if (!agents.Contains(agent))
            {{
                agents.Add(agent);
                Debug.Log($"Registered agent: {{agent.AgentName}}");
            }}
        }}
        
        // Unregister an agent from the manager
        public void UnregisterAgent(OxydeAgent agent)
        {{
            if (agents.Contains(agent))
            {{
                agents.Remove(agent);
                Debug.Log($"Unregistered agent: {{agent.AgentName}}");
            }}
        }}
        
        // Update all agents with player context
        public void UpdateAgentContext(Transform player, Dictionary<string, object> additionalContext = null)
        {{
            foreach (var agent in agents)
            {{
                agent.UpdatePlayerContext(player, additionalContext);
            }}
        }}
        
        // Process input for the nearest agent
        public string ProcessInputForNearestAgent(Transform player, string input, float maxDistance = 5f)
        {{
            OxydeAgent nearestAgent = null;
            float closestDistance = maxDistance;
            
            foreach (var agent in agents)
            {{
                float distance = Vector3.Distance(player.position, agent.transform.position);
                if (distance < closestDistance)
                {{
                    closestDistance = distance;
                    nearestAgent = agent;
                }}
            }}
            
            if (nearestAgent != null)
            {{
                return nearestAgent.ProcessInput(input);
            }}
            
            return "No one is close enough to hear you.";
        }}
    }}
}}
"#
    )
}

/// Generate Unity agent controller script
fn generate_unity_agent_script(agent: &AgentConfig, config_filename: &str) -> String {
    format!(
        r#"using UnityEngine;
using System.Collections.Generic;

namespace Oxyde.Unity
{{
    /// <summary>
    /// Controller for the {} agent
    /// </summary>
    public class {}Controller : OxydeAgent
    {{
        // Agent configuration
        [SerializeField] private string configResourcePath = "AgentConfigs/{}";
        
        // Agent movement
        [SerializeField] private float moveSpeed = 1.5f;
        [SerializeField] private Transform[] waypoints;
        private int currentWaypoint = 0;
        
        // Dialogue UI references
        [SerializeField] private GameObject dialogueBubble;
        [SerializeField] private TMPro.TextMeshProUGUI dialogueText;
        
        // NPC state
        private bool isPlayerNearby = false;
        private float lastGreetingTime = -999f;
        private const float GREETING_COOLDOWN = 60f;
        
        protected override void Start()
        {{
            base.Start();
            
            // Set agent name
            AgentName = "{}";
            
            // Initialize the agent with configuration
            InitializeAgent(configResourcePath);
            
            // Hide dialogue bubble initially
            if (dialogueBubble != null)
            {{
                dialogueBubble.SetActive(false);
            }}
        }}
        
        protected override void Update()
        {{
            base.Update();
            
            // Move between waypoints if player is not nearby
            if (!isPlayerNearby && waypoints != null && waypoints.Length > 0)
            {{
                MoveTowardsWaypoint();
            }}
            
            // Auto-greet player when nearby
            if (isPlayerNearby)
            {{
                if (Time.time - lastGreetingTime > GREETING_COOLDOWN)
                {{
                    TryGreetPlayer();
                }}
            }}
        }}
        
        private void MoveTowardsWaypoint()
        {{
            if (currentWaypoint < waypoints.Length)
            {{
                Vector3 targetPosition = waypoints[currentWaypoint].position;
                targetPosition.y = transform.position.y; // Keep same height
                
                // Move towards waypoint
                transform.position = Vector3.MoveTowards(
                    transform.position,
                    targetPosition,
                    moveSpeed * Time.deltaTime
                );
                
                // Look towards movement direction
                Vector3 direction = (targetPosition - transform.position).normalized;
                if (direction != Vector3.zero)
                {{
                    transform.forward = direction;
                }}
                
                // Check if reached waypoint
                if (Vector3.Distance(transform.position, targetPosition) < 0.1f)
                {{
                    // Move to next waypoint
                    currentWaypoint = (currentWaypoint + 1) % waypoints.Length;
                }}
            }}
        }}
        
        private void TryGreetPlayer()
        {{
            // Process a proximity "greeting" intent
            string response = ProcessIntent("proximity");
            
            if (!string.IsNullOrEmpty(response))
            {{
                ShowDialogue(response);
                lastGreetingTime = Time.time;
            }}
        }}
        
        // Show dialogue bubble with text
        public void ShowDialogue(string text)
        {{
            if (dialogueBubble != null && dialogueText != null)
            {{
                dialogueText.text = text;
                dialogueBubble.SetActive(true);
                
                // Hide dialogue after a delay
                CancelInvoke(nameof(HideDialogue));
                Invoke(nameof(HideDialogue), 4.0f);
            }}
        }}
        
        // Hide dialogue bubble
        private void HideDialogue()
        {{
            if (dialogueBubble != null)
            {{
                dialogueBubble.SetActive(false);
            }}
        }}
        
        // Process player input and display response
        public override string ProcessInput(string input)
        {{
            string response = base.ProcessInput(input);
            
            // Show response in dialogue bubble
            ShowDialogue(response);
            
            return response;
        }}
        
        // Process a specific intent type
        private string ProcessIntent(string intentType)
        {{
            // Create a context JSON with the intent type
            Dictionary<string, object> intentContext = new Dictionary<string, object>()
            {{
                {{ "intent_type", intentType }}
            }};
            
            // Create an empty message for proximity intents
            string inputText = intentType == "proximity" ? "" : "Hello";
            
            // Update context with intent
            UpdateContext(intentContext);
            
            // Process the input
            return ProcessInput(inputText);
        }}
        
        // Called when player enters detection range
        private void OnTriggerEnter(Collider other)
        {{
            if (other.CompareTag("Player"))
            {{
                isPlayerNearby = true;
            }}
        }}
        
        // Called when player exits detection range
        private void OnTriggerExit(Collider other)
        {{
            if (other.CompareTag("Player"))
            {{
                isPlayerNearby = false;
                HideDialogue();
            }}
        }}
    }}
}}
"#,
        agent.agent.role,
        agent.agent.name.replace(" ", ""),
        config_filename,
        agent.agent.name
    )
}

/// Generate Unity scene setup script
fn generate_unity_scene_script(agents: &[AgentConfig], scene_config: &serde_json::Value) -> String {
    // This is a simplified version; a real implementation would use scene_config
    format!(
        r#"using UnityEngine;
using System.Collections.Generic;

namespace Oxyde.Unity
{{
    /// <summary>
    /// Sets up the demo scene with Oxyde agents
    /// </summary>
    public class OxydeSceneSetup : MonoBehaviour
    {{
        [Header("Agent Prefabs")]
        [SerializeField] private GameObject[] agentPrefabs;
        
        [Header("Scene Setup")]
        [SerializeField] private Transform playerTransform;
        
        void Start()
        {{
            Debug.Log("Setting up Oxyde RPG demo scene");
            
            // Make sure we have the agent manager
            OxydeAgentManager manager = FindObjectOfType<OxydeAgentManager>();
            if (manager == null)
            {{
                GameObject managerObject = new GameObject("Oxyde Agent Manager");
                manager = managerObject.AddComponent<OxydeAgentManager>();
            }}
            
            // Spawn agents if none exist yet
            OxydeAgent[] existingAgents = FindObjectsOfType<OxydeAgent>();
            if (existingAgents.Length == 0 && agentPrefabs.Length > 0)
            {{
                SpawnAgents();
            }}
        }}
        
        void Update()
        {{
            // Update all agents with player position
            if (playerTransform != null)
            {{
                OxydeAgentManager.Instance.UpdateAgentContext(playerTransform);
            }}
        }}
        
        private void SpawnAgents()
        {{
            // In a real implementation, this would use the scene configuration
            // to determine positions and agent types
            
            // Spawn NPCs at predefined positions
            Vector3[] positions = new Vector3[]
            {{
                new Vector3(5, 0, 3),   // Shopkeeper
                new Vector3(-5, 0, -2), // Guard
                new Vector3(2, 0, -4)   // Villager
            }};
            
            // Spawn agents
            for (int i = 0; i < Mathf.Min(agentPrefabs.Length, positions.Length); i++)
            {{
                GameObject agentObject = Instantiate(agentPrefabs[i], positions[i], Quaternion.identity);
                agentObject.name = $"NPC_{i}";
            }}
            
            Debug.Log($"Spawned {{Mathf.Min(agentPrefabs.Length, positions.Length)}} agents");
        }}
    }}
}}
"#
    )
}

/// Deploy agents for Unreal engine
fn deploy_unreal_agents(
    agents: &[AgentConfig],
    scene_config: &serde_json::Value,
    output: &str,
) -> Result<()> {
    println!("Generating Unreal-specific files...");
    
    // Create Unreal-specific directories
    let include_dir = PathBuf::from(output).join("Public");
    let source_dir = PathBuf::from(output).join("Private");
    let configs_dir = PathBuf::from(output).join("Content/Oxyde/Configs");
    fs::create_dir_all(&include_dir)?;
    fs::create_dir_all(&source_dir)?;
    fs::create_dir_all(&configs_dir)?;
    
    // Generate header files
    let oxyde_header = generate_unreal_oxyde_header();
    fs::write(include_dir.join("OxydeNPC.h"), oxyde_header)?;
    
    let agent_header = generate_unreal_agent_header(agents);
    fs::write(include_dir.join("OxydeAgentTypes.h"), agent_header)?;
    
    // Generate source files
    let oxyde_source = generate_unreal_oxyde_source();
    fs::write(source_dir.join("OxydeNPC.cpp"), oxyde_source)?;
    
    // Write agent configurations
    for (i, agent) in agents.iter().enumerate() {
        let config_json = serde_json::to_string_pretty(agent)?;
        let config_filename = format!("Agent_{}.json", agent.agent.name.replace(" ", ""));
        fs::write(configs_dir.join(config_filename), config_json)?;
    }
    
    println!("Generated Unreal Engine integration files in: {}", output);
    Ok(())
}

/// Generate Unreal Engine header file
fn generate_unreal_oxyde_header() -> String {
    r#"// Copyright Epic Games, Inc. All Rights Reserved.

#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Character.h"
#include "OxydeAgentTypes.h"
#include "OxydeNPC.generated.h"

UCLASS()
class OXYDE_API AOxydeNPC : public ACharacter
{
    GENERATED_BODY()

public:
    // Sets default values for this character's properties
    AOxydeNPC();

    // Called every frame
    virtual void Tick(float DeltaTime) override;

    // Called to bind functionality to input
    virtual void SetupPlayerInputComponent(class UInputComponent* PlayerInputComponent) override;

    // Initialize the NPC agent
    UFUNCTION(BlueprintCallable, Category = "Oxyde")
    bool InitializeAgent(FString ConfigPath);

    // Process input for the agent
    UFUNCTION(BlueprintCallable, Category = "Oxyde")
    FString ProcessInput(FString Input);

    // Update agent context
    UFUNCTION(BlueprintCallable, Category = "Oxyde")
    void UpdateContext(FString ContextJSON);

    // Get agent name
    UFUNCTION(BlueprintPure, Category = "Oxyde")
    FString GetAgentName() const;

    // Get agent role
    UFUNCTION(BlueprintPure, Category = "Oxyde")
    FString GetAgentRole() const;

protected:
    // Called when the game starts or when spawned
    virtual void BeginPlay() override;

    // Agent configuration
    UPROPERTY(EditDefaultsOnly, BlueprintReadOnly, Category = "Oxyde")
    FString ConfigPath;

    // Agent state
    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Oxyde")
    FOxydeAgentState AgentState;

    // Dialogue component
    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Oxyde")
    class UDialogueComponent* DialogueComponent;

private:
    // Agent ID returned by the Oxyde SDK
    FString AgentId;

    // Generate context JSON from the current state
    FString GenerateContextJSON();
};
"#.to_string()
}

/// Generate Unreal Engine agent header
fn generate_unreal_agent_header(agents: &[AgentConfig]) -> String {
    let mut agent_enum_values = String::new();
    
    for agent in agents {
        let enum_name = agent.agent.name.replace(" ", "");
        agent_enum_values.push_str(&format!("    {}Agent,\n", enum_name));
    }
    
    format!(
        r#"// Copyright Epic Games, Inc. All Rights Reserved.

#pragma once

#include "CoreMinimal.h"
#include "OxydeAgentTypes.generated.h"

// Agent types available in the game
UENUM(BlueprintType)
enum class EOxydeAgentType : uint8
{{
{}
}};

// Agent state information
USTRUCT(BlueprintType)
struct FOxydeAgentState
{{
    GENERATED_BODY()

    // Agent ID
    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Oxyde")
    FString Id;

    // Agent name
    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Oxyde")
    FString Name;

    // Agent role
    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Oxyde")
    FString Role;

    // Current state
    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Oxyde")
    FString State;

    // Last response
    UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Oxyde")
    FString LastResponse;
}};
"#,
        agent_enum_values
    )
}

/// Generate Unreal Engine source file
fn generate_unreal_oxyde_source() -> String {
    r#"// Copyright Epic Games, Inc. All Rights Reserved.

#include "OxydeNPC.h"
#include "OxydeUnreal.h"
#include "Components/TextRenderComponent.h"
#include "Components/WidgetComponent.h"
#include "Kismet/GameplayStatics.h"
#include "Kismet/KismetSystemLibrary.h"
#include "Json.h"

// Sets default values
AOxydeNPC::AOxydeNPC()
{
    // Set this character to call Tick() every frame
    PrimaryActorTick.bCanEverTick = true;

    // Create dialogue component
    DialogueComponent = CreateDefaultSubobject<UWidgetComponent>(TEXT("DialogueComponent"));
    DialogueComponent->SetupAttachment(RootComponent);
    DialogueComponent->SetRelativeLocation(FVector(0, 0, 100));
    DialogueComponent->SetWidgetSpace(EWidgetSpace::Screen);
    DialogueComponent->SetDrawSize(FVector2D(200, 100));
}

// Called when the game starts or when spawned
void AOxydeNPC::BeginPlay()
{
    Super::BeginPlay();
    
    // Initialize Oxyde SDK
    OxydeUnreal::Init();
    
    // Initialize agent if config path is set
    if (!ConfigPath.IsEmpty())
    {
        InitializeAgent(ConfigPath);
    }
}

// Called every frame
void AOxydeNPC::Tick(float DeltaTime)
{
    Super::Tick(DeltaTime);

    // Update agent context with player distance
    APawn* PlayerPawn = UGameplayStatics::GetPlayerPawn(GetWorld(), 0);
    if (PlayerPawn && !AgentId.IsEmpty())
    {
        float Distance = FVector::Dist(GetActorLocation(), PlayerPawn->GetActorLocation());
        
        // Create context JSON
        TSharedPtr<FJsonObject> ContextObj = MakeShareable(new FJsonObject);
        ContextObj->SetNumberField("player_distance", Distance);
        ContextObj->SetNumberField("player_x", PlayerPawn->GetActorLocation().X);
        ContextObj->SetNumberField("player_y", PlayerPawn->GetActorLocation().Y);
        ContextObj->SetNumberField("player_z", PlayerPawn->GetActorLocation().Z);
        ContextObj->SetNumberField("npc_x", GetActorLocation().X);
        ContextObj->SetNumberField("npc_y", GetActorLocation().Y);
        ContextObj->SetNumberField("npc_z", GetActorLocation().Z);
        
        FString ContextJSON;
        TSharedRef<TJsonWriter<>> Writer = TJsonWriterFactory<>::Create(&ContextJSON);
        FJsonSerializer::Serialize(ContextObj.ToSharedRef(), Writer);
        
        // Update agent context
        UpdateContext(ContextJSON);
        
        // Auto greet player if close enough
        static float LastGreetingTime = -999.0f;
        const float GreetingCooldown = 60.0f;
        
        if (Distance < 300.0f && (GetWorld()->GetTimeSeconds() - LastGreetingTime > GreetingCooldown))
        {
            FString Response = ProcessInput("");
            if (!Response.IsEmpty())
            {
                LastGreetingTime = GetWorld()->GetTimeSeconds();
                AgentState.LastResponse = Response;
                
                // Display dialogue (would use widget in real implementation)
                UE_LOG(LogTemp, Display, TEXT("%s: %s"), *AgentState.Name, *Response);
            }
        }
    }
}

// Called to bind functionality to input
void AOxydeNPC::SetupPlayerInputComponent(UInputComponent* PlayerInputComponent)
{
    Super::SetupPlayerInputComponent(PlayerInputComponent);
}

// Initialize the agent
bool AOxydeNPC::InitializeAgent(FString ConfigPath)
{
    if (ConfigPath.IsEmpty())
    {
        UE_LOG(LogTemp, Error, TEXT("Cannot initialize agent: Configuration path is empty"));
        return false;
    }

    // Check if config file exists
    if (!FPaths::FileExists(ConfigPath))
    {
        UE_LOG(LogTemp, Error, TEXT("Cannot initialize agent: Configuration file not found: %s"), *ConfigPath);
        return false;
    }

    // Create agent using Oxyde SDK
    AgentId = OxydeUnreal::CreateAgent(TCHAR_TO_UTF8(*ConfigPath));
    
    if (AgentId.IsEmpty())
    {
        UE_LOG(LogTemp, Error, TEXT("Failed to create agent from config: %s"), *ConfigPath);
        return false;
    }
    
    // Get agent info
    FString AgentJSON = OxydeUnreal::GetAgentState(TCHAR_TO_UTF8(*AgentId));
    TSharedPtr<FJsonObject> AgentObj;
    TSharedRef<TJsonReader<>> Reader = TJsonReaderFactory<>::Create(AgentJSON);
    
    if (FJsonSerializer::Deserialize(Reader, AgentObj) && AgentObj.IsValid())
    {
        AgentState.Id = AgentId;
        AgentState.Name = AgentObj->GetStringField("name");
        AgentState.Role = AgentObj->GetStringField("role");
        AgentState.State = AgentObj->GetStringField("state");
    }
    
    UE_LOG(LogTemp, Display, TEXT("Initialized agent: %s (Role: %s)"), *AgentState.Name, *AgentState.Role);
    return true;
}

// Process input for the agent
FString AOxydeNPC::ProcessInput(FString Input)
{
    if (AgentId.IsEmpty())
    {
        UE_LOG(LogTemp, Warning, TEXT("Cannot process input: Agent not initialized"));
        return FString();
    }

    // Call Oxyde SDK to process input
    FString Response = OxydeUnreal::ProcessInput(TCHAR_TO_UTF8(*AgentId), TCHAR_TO_UTF8(*Input));
    AgentState.LastResponse = Response;
    
    return Response;
}

// Update agent context
void AOxydeNPC::UpdateContext(FString ContextJSON)
{
    if (AgentId.IsEmpty() || ContextJSON.IsEmpty())
    {
        return;
    }

    // Call Oxyde SDK to update context
    OxydeUnreal::UpdateAgentContext(TCHAR_TO_UTF8(*AgentId), TCHAR_TO_UTF8(*ContextJSON));
}

// Get agent name
FString AOxydeNPC::GetAgentName() const
{
    return AgentState.Name;
}

// Get agent role
FString AOxydeNPC::GetAgentRole() const
{
    return AgentState.Role;
}
"#.to_string()
}

/// Deploy agents for WebAssembly (browser-based games)
fn deploy_wasm_agents(
    agents: &[AgentConfig],
    scene_config: &serde_json::Value,
    output: &str,
) -> Result<()> {
    println!("Generating WebAssembly-specific files...");
    
    // Create WebAssembly-specific directories
    let js_dir = PathBuf::from(output).join("js");
    let config_dir = PathBuf::from(output).join("configs");
    fs::create_dir_all(&js_dir)?;
    fs::create_dir_all(&config_dir)?;
    
    // Generate JavaScript wrapper
    let js_wrapper = generate_wasm_js_wrapper();
    fs::write(js_dir.join("oxyde-wasm.js"), js_wrapper)?;
    
    // Generate demo HTML
    let demo_html = generate_wasm_demo_html(agents);
    fs::write(PathBuf::from(output).join("index.html"), demo_html)?;
    
    // Write agent configurations
    for agent in agents {
        let config_json = serde_json::to_string_pretty(agent)?;
        let config_filename = format!("{}.json", agent.agent.name.to_lowercase().replace(" ", "_"));
        fs::write(config_dir.join(config_filename), config_json)?;
    }
    
    println!("Generated WebAssembly integration files in: {}", output);
    Ok(())
}

/// Generate WebAssembly JavaScript wrapper
fn generate_wasm_js_wrapper() -> String {
    r#"// Oxyde WebAssembly SDK wrapper

class OxydeAgent {
  constructor(id, name, role) {
    this.id = id;
    this.name = name;
    this.role = role;
    this.position = { x: 0, y: 0 };
    this.lastResponse = "";
  }
}

class OxydeSDK {
  constructor() {
    this.initialized = false;
    this.agents = new Map();
    this.wasmInstance = null;
  }

  // Initialize the Oxyde SDK
  async init() {
    if (this.initialized) return true;
    
    try {
      // Import the WASM module
      const oxyde = await import('./oxyde_bg.wasm');
      
      // Create the instance
      this.wasmInstance = new oxyde.OxydeWasm();
      
      // Initialize the SDK
      const result = this.wasmInstance.init();
      this.initialized = result;
      
      console.log("Oxyde SDK initialized:", result);
      return result;
    } catch (error) {
      console.error("Failed to initialize Oxyde SDK:", error);
      return false;
    }
  }

  // Create a new agent from configuration
  async createAgent(configPath) {
    if (!this.initialized) {
      await this.init();
    }
    
    try {
      const agentId = await this.wasmInstance.create_agent(configPath);
      
      // Fetch the configuration to get agent details
      const response = await fetch(configPath);
      const config = await response.json();
      
      // Create agent object
      const agent = new OxydeAgent(
        agentId,
        config.agent.name,
        config.agent.role
      );
      
      // Store in our registry
      this.agents.set(agentId, agent);
      
      console.log(`Created agent: ${agent.name} (${agentId})`);
      return agent;
    } catch (error) {
      console.error("Failed to create agent:", error);
      return null;
    }
  }

  // Update agent context
  async updateAgentContext(agentId, context) {
    if (!this.initialized || !this.agents.has(agentId)) {
      return false;
    }
    
    try {
      // Convert context to JSON string
      const contextJSON = JSON.stringify(context);
      
      // Update agent context
      await this.wasmInstance.update_agent(agentId, contextJSON);
      
      // Update position in our record if provided
      if (context.position) {
        const agent = this.agents.get(agentId);
        agent.position = context.position;
      }
      
      return true;
    } catch (error) {
      console.error("Failed to update agent context:", error);
      return false;
    }
  }

  // Process input for an agent
  async processInput(agentId, input) {
    if (!this.initialized || !this.agents.has(agentId)) {
      return "Agent not found";
    }
    
    try {
      // Process input through WASM
      const response = await this.wasmInstance.process_input(agentId, input);
      
      // Update last response
      const agent = this.agents.get(agentId);
      agent.lastResponse = response;
      
      return response;
    } catch (error) {
      console.error("Failed to process input:", error);
      return "Error processing input";
    }
  }

  // Get all agents
  getAgents() {
    return Array.from(this.agents.values());
  }

  // Get an agent by ID
  getAgent(agentId) {
    return this.agents.get(agentId);
  }

  // Get nearest agent to a position
  getNearestAgent(position, maxDistance = Infinity) {
    let nearest = null;
    let minDistance = maxDistance;
    
    for (const agent of this.agents.values()) {
      const dx = agent.position.x - position.x;
      const dy = agent.position.y - position.y;
      const distance = Math.sqrt(dx*dx + dy*dy);
      
      if (distance < minDistance) {
        minDistance = distance;
        nearest = agent;
      }
    }
    
    return { agent: nearest, distance: minDistance };
  }
}

// Export singleton instance
const oxyde = new OxydeSDK();
export default oxyde;
"#.to_string()
}

/// Generate WebAssembly demo HTML
fn generate_wasm_demo_html(agents: &[AgentConfig]) -> String {
    let mut agent_buttons = String::new();
    
    for agent in agents {
        let id = agent.agent.name.to_lowercase().replace(" ", "_");
        agent_buttons.push_str(&format!(
            r#"<button onclick="loadAgent('configs/{}.json')">Load {}</button>"#,
            id, agent.agent.name
        ));
    }
    
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Oxyde RPG Demo</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 0; padding: 0; }}
        .game-container {{ 
            display: flex; 
            flex-direction: column; 
            width: 800px; 
            margin: 0 auto; 
            padding: 20px; 
        }}
        .game-view {{ 
            position: relative; 
            width: 100%; 
            height: 500px; 
            background-color: #f0f0f0; 
            border: 1px solid #ccc; 
            overflow: hidden; 
        }}
        .player {{ 
            position: absolute; 
            width: 20px; 
            height: 20px; 
            background-color: blue; 
            border-radius: 50%; 
        }}
        .npc {{ 
            position: absolute; 
            width: 20px; 
            height: 20px; 
            background-color: green; 
            border-radius: 50%; 
        }}
        .dialogue {{ 
            position: absolute; 
            background-color: white; 
            border: 1px solid #333; 
            border-radius: 5px; 
            padding: 5px; 
            max-width: 150px; 
            transform: translateX(-50%); 
            visibility: hidden; 
        }}
        .controls {{ margin-top: 20px; }}
        .console {{ 
            height: 200px; 
            margin-top: 20px; 
            border: 1px solid #ccc; 
            padding: 10px; 
            overflow-y: scroll; 
            background-color: #f9f9f9; 
        }}
        .chat-input {{ 
            display: flex; 
            margin-top: 10px; 
        }}
        .chat-input input {{ flex-grow: 1; padding: 5px; }}
        .chat-input button {{ padding: 5px 10px; }}
    </style>
</head>
<body>
    <div class="game-container">
        <h1>Oxyde RPG Demo</h1>
        
        <div class="game-view" id="gameView">
            <div class="player" id="player"></div>
            <!-- NPCs will be added dynamically -->
        </div>
        
        <div class="controls">
            <p>Use WASD keys to move. Press E near an NPC to interact.</p>
            <div id="agentButtons">
                {}
            </div>
        </div>
        
        <div class="chat-input">
            <input type="text" id="chatInput" placeholder="Type message...">
            <button onclick="sendChat()">Send</button>
        </div>
        
        <div class="console" id="console"></div>
    </div>
    
    <script type="module">
        import oxyde from './js/oxyde-wasm.js';
        
        // Make oxyde available globally
        window.oxyde = oxyde;
        window.agents = [];
        window.playerPos = {{ x: 400, y: 250 }};
        window.activeNpc = null;
        
        // Initialize the game
        window.initGame = async function() {{
            await oxyde.init();
            log("Oxyde SDK initialized");
            updatePlayerPosition();
            
            // Set up key controls
            document.addEventListener('keydown', handleKeyDown);
        }};
        
        // Load an agent
        window.loadAgent = async function(configPath) {{
            const agent = await oxyde.createAgent(configPath);
            if (agent) {{
                log(`Loaded agent: ${{agent.name}} (${{agent.role}})`);
                
                // Create visual representation
                createNpcElement(agent);
                
                // Set initial position - random in the game view
                const randomX = Math.floor(Math.random() * 700) + 50;
                const randomY = Math.floor(Math.random() * 400) + 50;
                updateNpcPosition(agent.id, randomX, randomY);
                
                window.agents.push(agent);
            }}
        }};
        
        // Create NPC visual element
        function createNpcElement(agent) {{
            const gameView = document.getElementById('gameView');
            
            // Create NPC element
            const npcElement = document.createElement('div');
            npcElement.id = `npc-${{agent.id}}`;
            npcElement.className = 'npc';
            npcElement.title = agent.name;
            
            // Create dialogue bubble
            const dialogueElement = document.createElement('div');
            dialogueElement.id = `dialogue-${{agent.id}}`;
            dialogueElement.className = 'dialogue';
            dialogueElement.style.bottom = '25px';
            
            // Add to game view
            gameView.appendChild(npcElement);
            gameView.appendChild(dialogueElement);
        }}
        
        // Update NPC position
        window.updateNpcPosition = function(agentId, x, y) {{
            const agent = oxyde.getAgent(agentId);
            if (agent) {{
                // Update agent's record
                agent.position = {{ x, y }};
                
                // Update visual position
                const npcElement = document.getElementById(`npc-${{agentId}}`);
                if (npcElement) {{
                    npcElement.style.left = `${{x}}px`;
                    npcElement.style.top = `${{y}}px`;
                }}
                
                // Update dialogue position
                const dialogueElement = document.getElementById(`dialogue-${{agentId}}`);
                if (dialogueElement) {{
                    dialogueElement.style.left = `${{x}}px`;
                }}
                
                // Update context with player distance
                const dx = window.playerPos.x - x;
                const dy = window.playerPos.y - y;
                const distance = Math.sqrt(dx*dx + dy*dy);
                
                oxyde.updateAgentContext(agentId, {{
                    position: {{ x, y }},
                    player_x: window.playerPos.x,
                    player_y: window.playerPos.y,
                    player_distance: distance
                }});
                
                // Auto-greet if player is close
                if (distance < 50) {{
                    checkProximityGreeting(agentId);
                }}
            }}
        }};
        
        // Update player position
        window.updatePlayerPosition = function() {{
            const playerElement = document.getElementById('player');
            playerElement.style.left = `${{window.playerPos.x}}px`;
            playerElement.style.top = `${{window.playerPos.y}}px`;
            
            // Update NPCs to react to player position
            window.agents.forEach(agent => {{
                updateNpcPosition(agent.id, agent.position.x, agent.position.y);
            }});
            
            // Find nearest NPC for interaction
            const {{ agent, distance }} = oxyde.getNearestAgent(window.playerPos, 70);
            window.activeNpc = distance < 70 ? agent : null;
        }};
        
        // Send chat message
        window.sendChat = async function() {{
            const chatInput = document.getElementById('chatInput');
            const message = chatInput.value.trim();
            
            if (!message) return;
            
            // Clear input
            chatInput.value = '';
            
            // Log player message
            log(`Player: ${{message}}`);
            
            // Process with nearest NPC if one is active
            if (window.activeNpc) {{
                const response = await oxyde.processInput(window.activeNpc.id, message);
                log(`${{window.activeNpc.name}}: ${{response}}`);
                showDialogue(window.activeNpc.id, response);
            }} else {{
                log("No one is close enough to hear you...");
            }}
        }};
        
        // Show dialogue bubble
        function showDialogue(agentId, text) {{
            const dialogueElement = document.getElementById(`dialogue-${{agentId}}`);
            if (dialogueElement) {{
                dialogueElement.textContent = text;
                dialogueElement.style.visibility = 'visible';
                
                // Hide after 5 seconds
                setTimeout(() => {{
                    dialogueElement.style.visibility = 'hidden';
                }}, 5000);
            }}
        }}
        
        // Check for proximity greeting
        async function checkProximityGreeting(agentId) {{
            const agent = oxyde.getAgent(agentId);
            if (!agent) return;
            
            // Check if we should greet (would have cooldown logic in real implementation)
            const now = Date.now();
            if (!agent.lastGreetingTime || now - agent.lastGreetingTime > 60000) {{
                const response = await oxyde.processInput(agentId, "");
                if (response && response.trim()) {{
                    log(`${{agent.name}}: ${{response}}`);
                    showDialogue(agentId, response);
                    agent.lastGreetingTime = now;
                }}
            }}
        }}
        
        // Handle keyboard input
        function handleKeyDown(event) {{
            const speed = 10;
            let dx = 0, dy = 0;
            
            switch(event.key.toLowerCase()) {{
                case 'w': dy = -speed; break;
                case 'a': dx = -speed; break;
                case 's': dy = speed; break;
                case 'd': dx = speed; break;
                case 'e': 
                    // Interact with nearest NPC
                    if (window.activeNpc) {{
                        document.getElementById('chatInput').focus();
                    }}
                    break;
            }}
            
            if (dx !== 0 || dy !== 0) {{
                // Update player position
                window.playerPos.x = Math.max(10, Math.min(790, window.playerPos.x + dx));
                window.playerPos.y = Math.max(10, Math.min(490, window.playerPos.y + dy));
                updatePlayerPosition();
                
                // Prevent default scrolling behavior
                event.preventDefault();
            }}
        }}
        
        // Log message to console
        window.log = function(message) {{
            const consoleElement = document.getElementById('console');
            const entry = document.createElement('div');
            entry.textContent = message;
            consoleElement.appendChild(entry);
            consoleElement.scrollTop = consoleElement.scrollHeight;
        }};
        
        // Initialize game when page loads
        document.addEventListener('DOMContentLoaded', initGame);
    </script>
</body>
</html>
"#,
        agent_buttons
    )
}

/// Test an agent with interactive chat
async fn test_agent(
    config_path: &str,
    local_only: bool,
    persistent_memory: bool,
) -> Result<()> {
    println!("Loading agent from: {}", config_path);
    
    // Load agent configuration
    let mut config = AgentConfig::from_file(config_path)?;
    
    // Override configuration based on command-line flags
    if local_only {
        config.inference.use_local = true;
    }
    
    if persistent_memory {
        config.memory.persistence = true;
    }
    
    // Create agent
    let agent = Agent::new(config);
    
    // Start agent
    agent.start().await?;
    
    println!("\n=== Agent Chat Test ===");
    println!("Agent: {}", agent.name());
    println!("Type your messages and press Enter. Type 'exit' to quit.\n");
    
    // Interactive chat loop
    loop {
        print!("> ");
        let _ = std::io::Write::flush(&mut std::io::stdout());
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            break;
        }
        
        // Process input
        match agent.process_input(input).await {
            Ok(response) => {
                println!("{}: {}", agent.name(), response);
            },
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
    
    // Stop agent
    agent.stop().await?;
    
    println!("Chat test completed");
    Ok(())
}

/// Convert agent configuration between formats
async fn convert_agent_config(
    input_path: &str,
    format: &str,
    output_path: &str,
) -> Result<()> {
    println!("Converting agent configuration: {} -> {}", input_path, output_path);
    
    // Load input configuration
    let config = AgentConfig::from_file(input_path)?;
    
    // Write in the specified format
    match format.to_lowercase().as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&config)?;
            fs::write(output_path, json)?;
        },
        "yaml" | "yml" => {
            let yaml = serde_yaml::to_string(&config)?;
            fs::write(output_path, yaml)?;
        },
        _ => {
            return Err(OxydeError::CliError(format!("Unsupported output format: {}", format)));
        }
    }
    
    println!("Conversion complete");
    Ok(())
}
