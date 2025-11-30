# oxyde-bindings

Engine-specific bindings for Unity, Unreal, and other game engines.

## Overview

This crate provides FFI (Foreign Function Interface) bindings that allow game engines to integrate with Oxyde agents. It includes C-compatible APIs and engine-specific wrappers.

## Features

- **Unity Support**: C# bindings and Unity package
- **Unreal Support**: C++ bindings and Unreal plugin
- **C API**: Pure C bindings for maximum compatibility
- **WASM Support**: WebAssembly bindings for web games
- **Memory Safety**: Safe FFI with proper error handling
- **Threading Support**: Async operations from game engines

## Planned Exports

### C API
```c
// C bindings
typedef struct OxydeAgent OxydeAgent;

OxydeAgent* oxyde_agent_create(const char* config_json);
void oxyde_agent_destroy(OxydeAgent* agent);
int oxyde_agent_start(OxydeAgent* agent);
int oxyde_agent_process_input(OxydeAgent* agent, const char* input, char* output, size_t output_size);
int oxyde_agent_get_emotional_state(OxydeAgent* agent, char* json_out, size_t size);
```

### Unity C# Wrapper
```csharp
public class OxydeAgent : IDisposable
{
    public OxydeAgent(string configJson);
    public Task StartAsync();
    public Task<string> ProcessInputAsync(string input);
    public EmotionalState GetEmotionalState();
    public void Dispose();
}
```

### Unreal C++ Wrapper
```cpp
class OXYDEBINDINGS_API UOxydeAgent : public UObject
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable)
    void Initialize(const FString& ConfigJson);

    UFUNCTION(BlueprintCallable)
    void ProcessInput(const FString& Input, FString& Output);

    UFUNCTION(BlueprintCallable)
    FEmotionalState GetEmotionalState() const;
};
```

## Use Cases

- Unity NPC integration
- Unreal Engine character AI
- Godot engine bindings
- Web-based games
- Custom engine integration
- Cross-platform AI agents

## Status

**Medium Priority** - Basic Unity/Unreal scaffolding exists. Needs comprehensive testing and packaging.

## Dependencies

- `oxyde-core` (for core types)
- `ffi-support` (for FFI helpers)
- Optional: `wasm-bindgen` (for WASM)

## Publication Priority

**Medium** - Important for game engine users, but can be developed after core crates are stable.
