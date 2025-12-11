# Oxyde Crates

This directory contains separate publishable crates that make up the Oxyde SDK ecosystem.

## Architecture

The Oxyde SDK is being modularized into focused, standalone crates that can be used independently or together. This provides maximum flexibility for different use cases and allows the community to benefit from individual components.

## Crate Overview

### Priority 1: Core Foundation
These crates must be published first as they form the foundation:

- **[oxyde-core](./oxyde-core/)** - Core types, traits, and utilities
  - Status: Essential, must be published first
  - Dependencies: None (base crate)

### Priority 2: Standalone Components
These crates can be used independently and have broad appeal:

- **[oxyde-emotion](./oxyde-emotion/)** - Plutchik's Wheel emotion system
  - Status: Ready for publication
  - Dependencies: None (only serde)
  - Best candidate for first standalone publication

### Priority 3: Integrated Components
These crates work together to provide the full Oxyde experience:

- **[oxyde-behavior](./oxyde-behavior/)** - Emotion-aware behavior system
  - Status: Ready for publication
  - Dependencies: `oxyde-emotion`, `oxyde-core`

- **[oxyde-memory](./oxyde-memory/)** - Emotional memory system
  - Status: Requires refactoring
  - Dependencies: `oxyde-emotion`, `oxyde-core`

### Priority 4: Specialized Components
These crates serve specific use cases:

- **[oxyde-inference](./oxyde-inference/)** - LLM inference abstraction
  - Status: Lower priority
  - Dependencies: `oxyde-core`

- **[oxyde-bindings](./oxyde-bindings/)** - Unity/Unreal/WASM bindings
  - Status: Medium priority
  - Dependencies: `oxyde-core`, `ffi-support`

## Publication Strategy

### Phase 1: Foundation (Week 1)
1. Extract and publish `oxyde-core` with core types
2. Extract and publish `oxyde-emotion` as first standalone component

### Phase 2: Behavior System (Week 2-3)
3. Extract and publish `oxyde-behavior` with emotion integration
4. Update examples to use separate crates

### Phase 3: Memory System (Week 3-4)
5. Refactor and extract `oxyde-memory`
6. Ensure clean separation from agent core

### Phase 4: Specialized Components (Week 4+)
7. Extract `oxyde-inference` if needed
8. Package `oxyde-bindings` for Unity/Unreal

### Phase 5: Integration (Week 5+)
9. Update main `oxyde` crate to use sub-crates
10. Comprehensive integration testing

## Why Separate Crates?

### Benefits
- **Modularity**: Use only what you need
- **Reduced dependencies**: Smaller dependency trees
- **Broader adoption**: Standalone components have wider appeal
- **Faster compilation**: Smaller crates compile faster
- **Clear boundaries**: Better separation of concerns
- **Community contributions**: Easier to contribute to specific areas

### Use Cases
- Games that only need emotion simulation (oxyde-emotion)
- Chatbots that need emotional memory (oxyde-emotion + oxyde-memory)
- NPC systems that need behavior control (oxyde-behavior)
- Full AI agents (all crates together)

## Dependency Graph

```
oxyde-core (foundation)
    ↓
oxyde-emotion (standalone, minimal deps)
    ↓
    ├─→ oxyde-behavior (depends on emotion + core)
    ├─→ oxyde-memory (depends on emotion + core)
    └─→ oxyde-inference (depends on core)
         ↓
         oxyde-bindings (depends on core)
              ↓
              oxyde (main SDK, integrates all)
```

## Current Status

- **Directory Structure**: ✅ Created
- **Documentation**: ✅ Complete
- **Code Extraction**: ⏳ Pending
- **Testing**: ⏳ Pending
- **Publication**: ⏳ Pending

## Next Steps

1. Start with `oxyde-emotion` extraction (most standalone)
2. Create proper Cargo.toml for each crate
3. Extract code into separate crates
4. Set up workspace structure
5. Add comprehensive tests for each crate
6. Prepare for crates.io publication

## Contributing

Each crate will have its own contribution guidelines. For now, all development happens in the main Oxyde repository.

## License

All crates are MIT licensed, same as the main Oxyde SDK.
