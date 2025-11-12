# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- GitHub Actions CI/CD workflow for automated testing and linting
- Pull request template for consistent PR descriptions
- CHANGELOG.md for tracking project changes

### Changed
- Split `behavior.rs` (581 lines) into separate modules for better organization
  - Created `behavior/base.rs` with Behavior trait and BehaviorResult
  - Created `behavior/greeting.rs` with GreetingBehavior
  - Created `behavior/dialogue.rs` with DialogueBehavior
  - Created `behavior/pathfinding.rs` with PathfindingBehavior
  - Created `behavior/factory.rs` with helper functions
  - Created `behavior/mod.rs` to re-export all types

### Fixed
- Removed unsafe code from memory.rs (replaced with OnceCell)
- Fixed AgentBuilder ignoring provided behaviors
- Replaced unwraps with proper error handling (agent.rs, unity.rs, unreal.rs, behavior.rs)
- Added type-safe enums for IntentType and AgentEvent

## [0.1.4] - 2024-01-15

### Added
- Initial public release
- Core Agent system for AI-powered NPCs
- Memory system with categorization and importance scoring
- Behavior system (Greeting, Dialogue, Pathfinding)
- Intent detection and classification
- Game engine bindings (Unity, Unreal, WASM)
- Inference engine with local and cloud provider support
- Configuration system with JSON/YAML support
