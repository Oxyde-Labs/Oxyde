//! Agent module for the Oxyde SDK
//!
//! This module provides the core Agent type, which represents an AI-driven NPC
//! in a game environment. Agents have behaviors, memory, and can interact with players.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::config::AgentConfig;
use crate::inference::InferenceEngine;
use crate::memory::{Memory, MemorySystem, MemoryCategory};
use crate::oxyde_game::behavior::{Behavior, BehaviorResult};
use crate::oxyde_game::emotion::EmotionalState;
use crate::oxyde_game::intent::Intent;
use crate::Result;

/// Callback for agent events
pub type AgentCallback = Box<dyn Fn(&Agent, &str) + Send + Sync>;

/// Wrapper for agent callbacks to make them Debug-able
pub struct CallbackWrapper(AgentCallback);

impl std::fmt::Debug for CallbackWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<AgentCallback>")
    }
}

impl CallbackWrapper {
    /// Create a new callback wrapper
    pub fn new(callback: AgentCallback) -> Self {
        Self(callback)
    }

    /// Call the underlying callback
    pub fn call(&self, agent: &Agent, data: &str) {
        (self.0)(agent, data);
    }
}

/// Agent state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    /// Agent is initializing
    Initializing,
    /// Agent is idle
    Idle,
    /// Agent is processing input
    Processing,
    /// Agent is generating a response
    Generating,
    /// Agent is executing a behavior
    Executing,
    /// Agent is paused
    Paused,
    /// Agent is stopped
    Stopped,
    /// Agent has encountered an error
    Error,
}

/// Agent event types for callbacks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgentEvent {
    /// Agent has started
    Start,
    /// Agent has stopped
    Stop,
    /// Agent is performing an action
    Action,
    /// Agent has generated a response
    Response,
    /// Agent state has changed
    StateChange,
    /// Agent encountered an error
    Error,
}

impl AgentEvent {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Stop => "stop",
            Self::Action => "action",
            Self::Response => "response",
            Self::StateChange => "state_change",
            Self::Error => "error",
        }
    }

    /// Convert from string representation
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "start" => Some(Self::Start),
            "stop" => Some(Self::Stop),
            "action" => Some(Self::Action),
            "response" => Some(Self::Response),
            "state_change" | "statechange" => Some(Self::StateChange),
            "error" => Some(Self::Error),
            _ => None,
        }
    }
}

impl std::fmt::Display for AgentEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Context data for an agent
pub type AgentContext = HashMap<String, serde_json::Value>;

/// Agent represents an AI-powered NPC in a game
pub struct Agent {
    /// Unique identifier for the agent
    id: Uuid,

    /// Agent name
    name: String,

    /// Agent configuration
    config: AgentConfig,

    /// Current state of the agent
    state: RwLock<AgentState>,

    /// Inference engine for generating responses
    inference: Arc<InferenceEngine>,

    /// Memory system for storing and retrieving context
    memory: Arc<MemorySystem>,

    /// Context data (current environment state)
    context: RwLock<AgentContext>,

    /// Behaviors available to the agent
    behaviors: RwLock<Vec<Box<dyn Behavior>>>,

    /// Callbacks for agent events
    callbacks: Mutex<HashMap<String, Vec<CallbackWrapper>>>,

    /// Emotional state of the agent
    emotional_state: RwLock<EmotionalState>,
}

impl Agent {
    /// Create a new agent with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Agent configuration
    ///
    /// # Returns
    ///
    /// A new Agent instance
    pub fn new(config: AgentConfig) -> Self {
        let inference = Arc::new(InferenceEngine::new(&config.inference));
        let memory = Arc::new(MemorySystem::new(config.memory.clone()));

        Self {
            id: Uuid::new_v4(),
            name: config.agent.name.clone(),
            config,
            state: RwLock::new(AgentState::Initializing),
            inference,
            memory,
            context: RwLock::new(HashMap::new()),
            behaviors: RwLock::new(Vec::new()),
            callbacks: Mutex::new(HashMap::new()),
            emotional_state: RwLock::new(EmotionalState::new()),
        }
    }

    /// Get the agent's unique ID
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Get the agent's name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the agent's current state
    pub async fn state(&self) -> AgentState {
        *self.state.read().await
    }

    /// Get a copy of the agent's current emotional state
    pub async fn emotional_state(&self) -> EmotionalState {
        self.emotional_state.read().await.clone()
    }

    /// Get the agent's emotion vector as a float array
    pub async fn emotion_vector(&self) -> [f32; 8] {
        let emotion_state = self.emotional_state.read().await;
        emotion_state.as_vector()
    }

    /// Update a specific emotion by a delta value
    ///
    /// # Arguments
    ///
    /// * `emotion` - Name of the emotion to update (e.g., "joy", "fear")
    /// * `delta` - Amount to change the emotion by (-1.0 to 1.0)
    pub async fn update_emotion(&self, emotion: &str, delta: f32) {
        let mut state = self.emotional_state.write().await;
        state.update_emotion(emotion, delta);
    }

    /// Apply emotional decay to all emotions
    ///
    /// This should be called periodically (e.g., every frame or tick)
    /// to allow emotions to naturally fade over time
    pub async fn decay_emotions(&self) {
        let mut state = self.emotional_state.write().await;
        state.decay();
    }

    /// Get the current emotional valence (-1.0 to 1.0)
    ///
    /// Valence represents how positive or negative the agent feels
    pub async fn emotional_valence(&self) -> f32 {
        self.emotional_state.read().await.valence()
    }

    /// Get the current emotional arousal (0.0 to 1.0)
    ///
    /// Arousal represents how intensely the agent is feeling
    pub async fn emotional_arousal(&self) -> f32 {
        self.emotional_state.read().await.arousal()
    }

    /// Add a behavior to the agent
    ///
    /// # Arguments
    ///
    /// * `behavior` - A behavior to add to the agent
    pub async fn add_behavior<B: Behavior + 'static>(&self, behavior: B) {
        let mut behaviors = self.behaviors.write().await;
        behaviors.push(Box::new(behavior));
    }

    /// Add a boxed behavior to the agent
    ///
    /// # Arguments
    ///
    /// * `behavior` - A boxed behavior to add to the agent
    pub async fn add_boxed_behavior(&self, behavior: Box<dyn Behavior>) {
        let mut behaviors = self.behaviors.write().await;
        behaviors.push(behavior);
    }

    /// Update the agent's context with new data
    ///
    /// # Arguments
    ///
    /// * `context` - New context data to merge with existing context
    pub async fn update_context(&self, context: AgentContext) {
        let mut current_context = self.context.write().await;
        for (key, value) in context {
            current_context.insert(key, value);
        }
    }

    /// Start the agent
    ///
    /// This initializes the agent and prepares it for operation
    pub async fn start(&self) -> Result<()> {
        let mut state = self.state.write().await;
        *state = AgentState::Idle;
        log::info!("Agent {} started", self.name);

        // Initialize memory with agent's backstory and knowledge
        self.memory.add(Memory::new(
            MemoryCategory::Semantic,
            &serde_json::to_string(&self.config.agent.backstory)?,
            f64::INFINITY,
            None
        )).await?;

        self.trigger_event(AgentEvent::Start, "Agent started").await;

        Ok(())
    }

    /// Stop the agent
    pub async fn stop(&self) -> Result<()> {
        let mut state = self.state.write().await;
        *state = AgentState::Stopped;
        log::info!("Agent {} stopped", self.name);

        self.trigger_event(AgentEvent::Stop, "Agent stopped").await;

        Ok(())
    }

    /// Process player input and generate a response
    ///
    /// # Arguments
    ///
    /// * `input` - Player input to process
    ///
    /// # Returns
    ///
    /// A result containing the agent's response
    pub async fn process_input(&self, input: &str) -> Result<String> {
        {
            let mut state = self.state.write().await;
            *state = AgentState::Processing;
        }

        log::debug!("Agent {} processing input: {}", self.name, input);

        // Analyze player intent
        let intent = Intent::analyze(input).await?;

        // Update memory with player input, capturing current emotional state
        let emotional_state = self.emotional_state.read().await;
        self.memory.add(Memory::new_emotional(
                MemoryCategory::Episodic,
                input,
                1.0,
                emotional_state.valence() as f64,
                emotional_state.arousal() as f64,
                None
            )).await?;

        // Find behaviors that match the intent
        let behaviors = self.behaviors.read().await;
        let mut response = String::new();

        {
            let mut state = self.state.write().await;
            *state = AgentState::Executing;
        }

        // Execute matching behaviors
        for behavior in behaviors.iter() {
            if behavior.matches_intent(&intent).await {
                let context = self.context.read().await.clone();
                let behavior_result = behavior.execute(&intent, &context).await?;

                match behavior_result {
                    BehaviorResult::Response(text) => {
                        response = text;
                        break;
                    },
                    BehaviorResult::Action(action) => {
                        // Trigger action callback
                        self.trigger_event(AgentEvent::Action, &action).await;
                    },
                    BehaviorResult::None => {
                        // Continue to next behavior
                    }
                }
            }
        }

        // If no behavior provided a response, generate one with inference
        if response.is_empty() {
            {
                let mut state = self.state.write().await;
                *state = AgentState::Generating;
            }

            // Get relevant memories
            let memories = self.memory.retrieve_relevant(input, 5, None).await?;

            // Generate response using inference engine
            let context = self.context.read().await.clone();
            response = self.inference.generate_response(input, &memories, &context).await?;

            // Store the response in memory with current emotional state
            let emotional_state = self.emotional_state.read().await;
            self.memory.add(Memory::new_emotional(
                MemoryCategory::Semantic,
                &response,
                1.0,
                emotional_state.valence() as f64,
                emotional_state.arousal() as f64,
                None
            )).await?;
        }

        {
            let mut state = self.state.write().await;
            *state = AgentState::Idle;
        }


        // Trigger response callback
        self.trigger_event(AgentEvent::Response, &response).await;

        Ok(response)
    }

    /// Register a callback for agent events using typed events
    ///
    /// # Arguments
    ///
    /// * `event` - Event type to trigger the callback
    /// * `callback` - Callback function
    ///
    /// # Example
    ///
    /// ```ignore
    /// agent.on_event(AgentEvent::Start, |agent, data| {
    ///     println!("Agent {} started: {}", agent.name(), data);
    /// });
    /// ```
    pub fn on_event<F>(&self, event: AgentEvent, callback: F)
    where
        F: Fn(&Agent, &str) + Send + Sync + 'static,
    {
        self.register_callback(event.as_str(), callback);
    }

    /// Register a callback for agent events (deprecated, use on_event)
    ///
    /// # Arguments
    ///
    /// * `event` - Event name to trigger the callback
    /// * `callback` - Callback function
    #[deprecated(since = "0.1.5", note = "Use on_event() with AgentEvent enum instead")]
    pub fn register_callback<F>(&self, event: &str, callback: F)
    where
        F: Fn(&Agent, &str) + Send + Sync + 'static,
    {
        // Lock the callbacks mutex, recovering from poison if necessary
        let mut callbacks = self.callbacks.lock().unwrap_or_else(|poisoned| {
            log::warn!("Callback mutex was poisoned, recovering");
            poisoned.into_inner()
        });
        let event_callbacks = callbacks.entry(event.to_string()).or_insert(Vec::new());
        event_callbacks.push(CallbackWrapper::new(Box::new(callback)));
    }

    /// Trigger a callback for a typed event
    ///
    /// # Arguments
    ///
    /// * `event` - Event type
    /// * `data` - Event data
    async fn trigger_event(&self, event: AgentEvent, data: &str) {
        self.trigger_callback(event.as_str(), data).await;
    }

    /// Trigger a callback for an event
    ///
    /// # Arguments
    ///
    /// * `event` - Event name
    /// * `data` - Event data
    async fn trigger_callback(&self, event: &str, data: &str) {
        // Lock the callbacks mutex, recovering from poison if necessary
        let callbacks = self.callbacks.lock().unwrap_or_else(|poisoned| {
            log::warn!("Callback mutex was poisoned during trigger, recovering");
            poisoned.into_inner()
        });
        if let Some(event_callbacks) = callbacks.get(event) {
            for callback in event_callbacks {
                callback.call(self, data);
            }
        }
    }


    /// Create a new agent with the same configuration but new state
    /// 
    /// This is a simplified clone method that creates a new agent with the same
    /// configuration but with fresh state. This is useful for creating copies
    /// of agents for engine bindings.
    pub fn clone_for_binding(&self) -> Self {
        Self::new(self.config.clone())
    }
}

impl std::fmt::Debug for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let callbacks_count = self.callbacks.lock()
            .map(|cb| cb.len())
            .unwrap_or(0);

        f.debug_struct("Agent")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("config", &self.config)
            // Don't debug the behaviors or callbacks directly as they don't implement Debug
            .field("behaviors_count", &format!("<{} behaviors>", self.behaviors.try_read().map(|b| b.len()).unwrap_or(0)))
            .field("callbacks_count", &format!("<{} callback types>", callbacks_count))
            .finish()
    }
}

/// AgentBuilder for fluent construction of Agents
#[derive(Default)]
pub struct AgentBuilder {
    config: Option<AgentConfig>,
    behaviors: Vec<Box<dyn Behavior>>,
}

impl AgentBuilder {
    /// Create a new AgentBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the agent configuration
    pub fn with_config(mut self, config: AgentConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Add a behavior to the agent
    pub fn with_behavior<B: Behavior + 'static>(mut self, behavior: B) -> Self {
        self.behaviors.push(Box::new(behavior));
        self
    }

    /// Build the agent
    pub async fn build(self) -> Result<Agent> {
        let config = self.config.ok_or_else(|| {
            crate::OxydeError::ConfigurationError("Agent configuration is required".to_string())
        })?;

        // Validate the configuration before building
        config.validate()?;

        let agent = Agent::new(config);

        // Add all behaviors provided via the builder
        for behavior in self.behaviors {
            agent.add_boxed_behavior(behavior).await;
        }

        Ok(agent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AgentPersonality, InferenceConfig, MemoryConfig};

    #[tokio::test]
    async fn test_agent_creation() {
        let config = AgentConfig {
            agent: AgentPersonality {
                name: "Test Agent".to_string(),
                role: "Tester".to_string(),
                backstory: vec!["A test agent".to_string()],
                knowledge: vec!["Testing knowledge".to_string()],
            },
            memory: MemoryConfig::default(),
            inference: InferenceConfig::default(),
            behavior: HashMap::new(),
        };

        let agent = Agent::new(config);
        assert_eq!(agent.name(), "Test Agent");

        agent.start().await.unwrap();
        assert_eq!(agent.state().await, AgentState::Idle);

        agent.stop().await.unwrap();
        assert_eq!(agent.state().await, AgentState::Stopped);
    }

    #[tokio::test]
    async fn test_agent_builder_with_behaviors() {
        use crate::oxyde_game::behavior::GreetingBehavior;

        let config = AgentConfig {
            agent: AgentPersonality {
                name: "Builder Test".to_string(),
                role: "Tester".to_string(),
                backstory: vec!["Built with builder".to_string()],
                knowledge: vec![],
            },
            memory: MemoryConfig::default(),
            inference: InferenceConfig::default(),
            behavior: HashMap::new(),
        };

        // Create agent with builder and add behaviors
        let greeting1 = GreetingBehavior::new("Hello!");
        let greeting2 = GreetingBehavior::new("Greetings!");

        let agent = AgentBuilder::new()
            .with_config(config)
            .with_behavior(greeting1)
            .with_behavior(greeting2)
            .build()
            .await
            .unwrap();

        assert_eq!(agent.name(), "Builder Test");

        // Verify behaviors were added (check the count)
        let behaviors = agent.behaviors.read().await;
        assert_eq!(behaviors.len(), 2, "Builder should add all provided behaviors");
    }

    #[tokio::test]
    async fn test_agent_builder_without_config_fails() {
        use crate::oxyde_game::behavior::GreetingBehavior;

        let greeting = GreetingBehavior::new("Hello!");

        // Attempt to build without config should fail
        let result = AgentBuilder::new()
            .with_behavior(greeting)
            .build()
            .await;

        assert!(result.is_err(), "Building without config should fail");
        if let Err(crate::OxydeError::ConfigurationError(msg)) = result {
            assert!(msg.contains("required"), "Error should mention config is required");
        } else {
            panic!("Expected ConfigurationError");
        }
    }
}