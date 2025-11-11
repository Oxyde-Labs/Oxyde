//! # Oxyde
//! 
//! Oxyde is a Rust-based SDK for integrating AI-powered NPCs into games.
//! It provides real-time inference capabilities and supports multiple game engines
//! including Unity, Unreal Engine, and WebAssembly for browser-based games.
//!
//! ## Core Features
//!
//! - AI-driven NPCs that understand player intent and adapt behavior
//! - Cross-platform engine support (Unity, Unreal, WASM)
//! - Real-time inference layer with async support
//! - Developer-facing tools for testing and configuration
//!
//! ## Example
//!
//! ```no_run
//! use oxyde::agent::Agent;
//! use oxyde::config::AgentConfig;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Load agent configuration
//!     let config = AgentConfig::from_file("npc_config.json").unwrap();
//!     
//!     // Create and initialize agent
//!     let mut agent = Agent::new(config);
//!     
//!     // Start the agent
//!     agent.start().await.unwrap();
//! }
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

// Re-exports
pub use agent::Agent;
pub use config::AgentConfig;
pub use inference::InferenceEngine;
pub use memory::MemorySystem;

// Modules
pub mod audio;
pub mod agent;
pub mod config;
pub mod inference;
pub mod memory;
pub mod oxyde_game;

// Internal modules
mod error;
mod utils;

pub use error::OxydeError;
/// Type alias for Results that use OxydeError
pub type Result<T> = std::result::Result<T, OxydeError>;

/// Current version of the Oxyde SDK
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the Oxyde SDK
///
/// This function sets up logging and prepares the SDK for use.
pub fn init() -> Result<()> {
    env_logger::init();
    log::info!("Initializing Oxyde SDK v{}", VERSION);
    Ok(())
}
