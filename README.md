# Oxyde: AI Agent SDK for Game NPCs

Oxyde is a revolutionary Rust-based SDK for creating autonomous, goal-driven NPCs with advanced AI and emotional intelligence. Build NPCs that pursue their own objectives, adapt to player interactions, and generate emergent storylines in real-time.

## 🚀 Revolutionary Features

- **Goal-Driven AI with Emergent Storytelling**: NPCs pursue personal objectives like "earn 1000 gold" or "uncover criminal networks", creating dynamic narratives that evolve based on player interactions
- **Autonomous NPC Behavior**: NPCs make independent decisions, form relationships, and adapt their strategies based on success/failure
- **Advanced Emotional Intelligence**: 6-dimensional emotional tracking (happiness, anger, fear, trust, energy, curiosity) that influences all NPC responses and decisions
- **Multi-LLM Architecture**: Smart provider selection across OpenAI, Anthropic Claude, Groq, xAI Grok, Perplexity, and local models for optimal performance and cost
- **Real-Time Adaptation**: NPCs learn from every interaction, updating goals and emotional states dynamically
- **Dynamic Story Generation**: Automatic story event creation based on NPC goal progress and emotional states

## 🎮 Engine Support

- **Unity**: Full C# bindings with memory/state management
- **Unreal Engine**: C++ compatibility with native UE types  
- **WebAssembly**: Browser-based games with async inference
- **Standalone**: Direct Rust integration for custom engines

## 🧠 Advanced AI Systems

- **Multi-Provider LLM Integration**: OpenAI GPT-4o, Anthropic Claude-3.5, Groq Llama3, xAI Grok-2, Perplexity Sonar, with intelligent provider selection
- **Sophisticated Memory System**: Vector embeddings, episodic/semantic memory, emotional context
- **Goal Management Engine**: NPCs track multiple objectives with priority systems and time pressure
- **Emotional Evolution**: Personality traits that change based on player relationships and experiences
- **Emergent Narrative Creation**: Dynamic story events generated from NPC interactions and goal progress

## 🎯 Live Demo Applications

### 1. **Interactive Web RPG Demo** (Recommended)
Experience goal-driven NPCs with emotional intelligence in your browser:
```bash
cd examples/rpg_demo
cargo run
# Visit http://localhost:5000
```

**What You'll Experience:**
- **Marcus the Merchant**: Pursuing his goal to "earn 1000 gold coins" - watch him become more excited about potential trades
- **Gareth the Guard**: Focused on "uncovering the local smuggling operation" - he'll probe for information and react suspiciously
- **Velma the Villager**: Working to "organize the harvest festival" - she'll share gossip and seek social connections
- **Real-time Emotional Evolution**: NPCs remember your interactions and adapt their personalities
- **Emergent Storylines**: NPCs generate dynamic story events based on their goal progress

### 2. **Standalone Console Demo**
Simple conversational interface for testing:
```bash
cargo run --example standalone_demo
```

### 3. **Minimal RPG Demo**
Lightweight version without web interface:
```bash
rustc -o rpg_demo_standalone rpg_demo_standalone.rs
./rpg_demo_standalone
```

## 🛠️ Quick Setup

### Prerequisites
- Rust 1.70+ (install from https://rustup.rs/)
- At least one LLM API key for AI functionality:
  - **OpenAI API key** (recommended for general use)
  - **Anthropic API key** (for advanced reasoning)
  - **Groq API key** (for fast inference)
  - **xAI API key** (for creative dialogue)
  - **Perplexity API key** (for real-time knowledge)

### Installation
```bash
git clone <repository-url>
cd oxyde-ai-sdk

# Set up API keys (choose one or more):
export OPENAI_API_KEY="your-openai-key"           # General purpose AI
export ANTHROPIC_API_KEY="your-anthropic-key"     # Advanced reasoning
export GROQ_API_KEY="your-groq-key"               # Fast inference
export XAI_API_KEY="your-xai-key"                 # Creative dialogue
export PERPLEXITY_API_KEY="your-perplexity-key"   # Real-time knowledge

cargo build
```

### Run the Demo
```bash
cd examples/rpg_demo
cargo run
# Open http://localhost:5000 in your browser
```

### Integration in Your Game
Add Oxyde to your Rust project:
```toml
[dependencies]
oxyde = { path = "path/to/oxyde-ai-sdk" }
```

## 🎮 How to Experience the Demo

### Web Interface Controls
- **Click NPCs** to start conversations
- **Type messages** and press Enter to chat
- **Watch emotions evolve** as NPCs react to your words
- **Observe goal progress** mentioned in NPC responses
- **Experience emergent stories** as NPCs pursue their objectives

### What Makes This Revolutionary
- **Marcus the Merchant** will mention his gold-earning progress and become more business-focused over time
- **Gareth the Guard** grows suspicious and protective based on your interactions
- **Velma the Villager** becomes more social and gossipy as you build friendship
- **Story Events** generate automatically when NPCs make progress toward goals

## 🏗️ Architecture Overview

### Core SDK Structure
```
oxyde/
├── src/
│   ├── agent.rs       # Goal-driven agent with emotional intelligence
│   ├── inference.rs   # Multi-LLM provider abstraction
│   ├── memory.rs      # Vector embeddings + emotional context
│   └── oxyde_game/    # Game integration modules
│       ├── behavior.rs    # Autonomous NPC behaviors
│       ├── intent.rs      # Player intent detection
│       └── bindings/      # Engine integration layers
└── examples/
    └── rpg_demo/      # Full-featured web demo
        ├── emotion_engine.rs  # 6D emotional tracking
        ├── goal_system.rs     # Autonomous goal management
        ├── llm_service.rs     # Smart provider selection
        └── web_server.rs      # Interactive web interface
```

### Revolutionary Components

#### 1. **Goal-Driven Agent System**
- NPCs pursue multiple personal objectives simultaneously
- Dynamic goal generation based on role and personality
- Progress tracking with emotional reward systems
- Autonomous decision-making for goal prioritization

#### 2. **Advanced Emotional Intelligence**
- **6-Dimensional Emotional Tracking**: happiness, anger, fear, trust, energy, curiosity
- **Emotional Evolution**: Personality changes based on player interactions
- **Memory-Emotion Integration**: Past experiences influence current emotional state
- **Context-Aware Responses**: Emotions drive dialogue style and content

#### 3. **Multi-LLM Orchestration**
- **Intelligent Provider Selection**: Context-aware routing to optimal LLM for each scenario
- **OpenAI GPT-4o**: Emotional intelligence and general-purpose conversations
- **Anthropic Claude-3.5**: Complex reasoning and detailed analysis
- **Groq Llama3**: Ultra-fast inference for real-time interactions
- **xAI Grok-2**: Creative storytelling and conversational humor
- **Perplexity Sonar**: Real-time knowledge and current events
- **Smart Fallbacks**: Automatic provider switching based on availability and context

#### 4. **Emergent Storytelling Engine**
- **Dynamic Story Events**: Generated automatically from NPC goal progress
- **Relationship Tracking**: NPCs remember and reference past interactions
- **Consequence Systems**: Player actions have lasting effects on NPC behavior
- **Narrative Coherence**: Story events maintain consistency with established character goals

## 🔧 Adding New LLM Providers

### Step-by-Step Integration Guide

The Oxyde SDK is designed for easy extensibility. Here's how to add a new LLM provider:

#### 1. **Define the Provider Enum**
```rust
// In examples/rpg_demo/src/llm_service.rs
#[derive(Debug, Clone, PartialEq)]
pub enum LLMProvider {
    OpenAI,
    Anthropic,
    Groq,
    XAI,
    Perplexity,
    YourNewProvider,  // Add your provider here
    Local,
}
```

#### 2. **Add Provider Configuration**
```rust
// Add API endpoint and model configuration
impl LLMProvider {
    pub fn api_endpoint(&self) -> &'static str {
        match self {
            LLMProvider::OpenAI => "https://api.openai.com/v1/chat/completions",
            LLMProvider::Anthropic => "https://api.anthropic.com/v1/messages",
            LLMProvider::Groq => "https://api.groq.com/openai/v1/chat/completions",
            LLMProvider::XAI => "https://api.x.ai/v1/chat/completions",
            LLMProvider::Perplexity => "https://api.perplexity.ai/chat/completions",
            LLMProvider::YourNewProvider => "https://api.yourprovider.com/v1/chat",
            LLMProvider::Local => "http://localhost:8080/chat/completions",
        }
    }

    pub fn default_model(&self) -> &'static str {
        match self {
            LLMProvider::OpenAI => "gpt-4o",
            LLMProvider::Anthropic => "claude-3-5-sonnet-20241022",
            LLMProvider::Groq => "llama-3.1-8b-instant",
            LLMProvider::XAI => "grok-2-1212",
            LLMProvider::Perplexity => "llama-3.1-sonar-small-128k-online",
            LLMProvider::YourNewProvider => "your-model-name",
            LLMProvider::Local => "local-model",
        }
    }
}
```

#### 3. **Implement Request Logic**
```rust
// Add provider-specific request handling in LLMService impl
async fn your_provider_request(
    &self,
    messages: Vec<serde_json::Value>,
    system_prompt: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let api_key = env::var("YOUR_PROVIDER_API_KEY")
        .map_err(|_| "YOUR_PROVIDER_API_KEY not found")?;

    let request_body = json!({
        "model": self.provider.default_model(),
        "messages": messages,
        "max_tokens": 1000,
        "temperature": 0.7
    });

    let response = self.client
        .post(self.provider.api_endpoint())
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    // Parse response based on your provider's format
    let response_json: serde_json::Value = response.json().await?;
    let content = response_json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No response")
        .to_string();

    Ok(content)
}
```

#### 4. **Add to Main Request Handler**
```rust
// In generate_goal_driven_response method, add your provider case
match self.provider {
    LLMProvider::OpenAI => self.openai_goal_driven_request(messages, system_prompt, goal_context).await,
    LLMProvider::Anthropic => self.anthropic_goal_driven_request(messages, system_prompt, goal_context).await,
    LLMProvider::Groq => self.groq_goal_driven_request(messages, system_prompt, goal_context).await,
    LLMProvider::XAI => self.xai_goal_driven_request(messages, system_prompt, goal_context).await,
    LLMProvider::Perplexity => self.perplexity_goal_driven_request(messages, system_prompt, goal_context).await,
    LLMProvider::YourNewProvider => self.your_provider_request(messages, system_prompt).await,
    LLMProvider::Local => self.local_request(messages, system_prompt).await,
}
```

#### 5. **Update Smart Routing Logic**
```rust
// In select_optimal_provider function
pub fn select_optimal_provider(context: &str) -> LLMProvider {
    // Add your provider's specialty
    if context.contains("your_specialty_keyword") {
        if env::var("YOUR_PROVIDER_API_KEY").is_ok() {
            return LLMProvider::YourNewProvider;
        }
    }
    
    // ... existing routing logic ...
    
    // Add to fallback chain
    if env::var("YOUR_PROVIDER_API_KEY").is_ok() {
        LLMProvider::YourNewProvider
    } else {
        LLMProvider::Local
    }
}
```

#### 6. **Add Environment Variable**
```bash
export YOUR_PROVIDER_API_KEY="your-api-key-here"
```

### Provider Integration Examples

#### **OpenAI-Compatible APIs**
Most modern LLM providers use OpenAI-compatible formats. Simply update the endpoint and authentication:

```rust
LLMProvider::YourProvider => "https://api.yourprovider.com/v1/chat/completions",
```

#### **Custom API Formats**
For providers with unique formats (like Anthropic), implement custom request/response handling:

```rust
// Custom request format for unique providers
let request_body = json!({
    "model": "your-model",
    "messages": transform_messages_for_provider(messages),
    "custom_param": "value"
});
```

### Testing Your Integration

1. **Set Environment Variable**: `export YOUR_PROVIDER_API_KEY="test-key"`
2. **Update Context Routing**: Add keywords that trigger your provider
3. **Test in Demo**: Use keywords in conversations to trigger provider selection
4. **Verify Fallbacks**: Ensure graceful degradation when your provider is unavailable

### Best Practices

- **Error Handling**: Always implement comprehensive error handling with fallbacks
- **Rate Limiting**: Respect provider rate limits and implement backoff strategies  
- **Cost Optimization**: Consider token costs when implementing routing logic
- **Authentication**: Support multiple auth methods (API keys, OAuth, etc.)
- **Model Selection**: Allow dynamic model selection based on use case
- **Context Awareness**: Define clear routing triggers for optimal provider selection

## 📊 Implementation Status

| Component | Status | Description |
|-----------|--------|-------------|
| Goal-Driven AI System | ✅ Complete | Autonomous NPCs with personal objectives and motivation tracking |
| Emotional Intelligence Engine | ✅ Complete | 6-dimensional emotional tracking with personality evolution |
| Multi-LLM Architecture | ✅ Complete | OpenAI + Anthropic + Groq + xAI + Perplexity integration with intelligent provider selection |
| Emergent Storytelling | ✅ Complete | Dynamic story event generation from NPC goal progress |
| Web RPG Demo | ✅ Complete | Interactive browser-based demo with real-time AI |
| Memory System | ✅ Complete | Vector embeddings with emotional context integration |
| Agent System | ✅ Complete | Core agent implementation with advanced state management |
| Behavior System | ✅ Complete | Autonomous behaviors (dialogue, goal pursuit, adaptation) |
| Engine Bindings | ⚠️ Partial | Unity and WASM bindings available, Unreal in development |

### Current Capabilities

✅ **Fully Operational:**
- NPCs pursue personal goals ("earn gold", "solve mysteries", "build relationships")
- Real-time emotional evolution based on player interactions
- Multi-LLM provider switching for optimal performance
- Dynamic story generation from NPC objectives
- Persistent conversation memory with emotional context
- Web-based interactive demo with authentic AI responses

⚠️ **In Development:**
- Additional engine integrations (Unreal, Godot)
- Persistent storage for long-term NPC evolution
- Advanced goal completion rewards and consequences
- Multi-NPC relationship networks and conflicts

## 🚀 Why This Matters

Oxyde represents a breakthrough in game AI - moving beyond scripted responses to truly autonomous NPCs that:
- **Think independently** about their own goals and motivations
- **Evolve emotionally** based on player relationships
- **Generate stories** dynamically without pre-written content
- **Adapt strategies** when goals succeed or fail
- **Remember everything** with emotional context intact

This creates gameplay experiences that are genuinely unpredictable and personally meaningful to each player.

## 🔬 Technical Innovation

- **First** Rust-based SDK for goal-driven game AI
- **First** implementation of multi-dimensional NPC emotional evolution
- **First** dynamic story generation from autonomous NPC objectives  
- **Production-ready** multi-LLM architecture with cost optimization
- **Real-time** personality adaptation without performance penalties

## 📈 Future Roadmap

### Phase 1: Enhanced Autonomy
- Multi-NPC collaboration and conflict systems
- Advanced goal hierarchies with sub-objectives
- Economic simulation integration for merchant NPCs

### Phase 2: Extended Engine Support
- Complete Unreal Engine integration
- Godot and Game Maker Studio bindings
- Mobile game platform optimization

### Phase 3: Advanced AI Features
- Local LLM integration (Llama.cpp, Ollama)
- Voice synthesis and recognition
- Computer vision for NPC environmental awareness
