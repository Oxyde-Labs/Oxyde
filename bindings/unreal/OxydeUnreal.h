// Copyright Epic Games, Inc. All Rights Reserved.

#pragma once

#include "CoreMinimal.h"
#include "Kismet/BlueprintFunctionLibrary.h"
#include "OxydeUnreal.generated.h"

#ifndef OXYDE_API
#define OXYDE_API
#endif

/**
 * Blueprint Function Library for the Oxyde SDK in Unreal Engine
 */
UCLASS()
class OXYDE_API UOxydeLibrary : public UBlueprintFunctionLibrary
{
    GENERATED_BODY()

public:
    /**
     * Initialize the Oxyde SDK
     * @return True if initialization was successful
     */
    UFUNCTION(BlueprintCallable, Category = "Oxyde")
    static bool Init();

    /**
     * Create a new agent from a configuration file
     * @param ConfigPath Path to the agent configuration file
     * @return Agent ID string or empty if failed
     */
    UFUNCTION(BlueprintCallable, Category = "Oxyde")
    static FString CreateAgent(FString ConfigPath);

    /**
     * Create a new agent from a configuration JSON string
     * @param JsonConfig The agent configuration as a JSON string
     * @return Agent ID string or empty if failed
     */
    UFUNCTION(BlueprintCallable, Category = "Oxyde")
    static FString CreateAgentFromJson(FString JsonConfig);

    /**
     * Create a new agent from a configuration file relative to the Content directory
     * @param ContentPath Path to the config file relative to Content folder (e.g. "Configs/Agent.json")
     * @return Agent ID string or empty if failed
     */
    UFUNCTION(BlueprintCallable, Category = "Oxyde")
    static FString CreateAgentFromContent(FString ContentPath);

    /**
     * Update an agent with new context data
     * @param AgentId Agent ID string
     * @param ContextJson JSON string with context data
     * @return True if update was successful
     */
    UFUNCTION(BlueprintCallable, Category = "Oxyde")
    static bool UpdateAgentContext(FString AgentId, FString ContextJson);

    /**
     * Process input for an agent
     * @param AgentId Agent ID string
     * @param Input Input text
     * @return Agent's response
     */
    UFUNCTION(BlueprintCallable, Category = "Oxyde")
    static FString ProcessInput(FString AgentId, FString Input);

    /**
     * Get the current state of an agent
     * @param AgentId Agent ID string
     * @return JSON string with agent state
     */
    UFUNCTION(BlueprintCallable, Category = "Oxyde")
    static FString GetAgentState(FString AgentId);

    /**
     * Get the agent's emotion vector
     * @param AgentId Agent ID string
     * @param OutJoy Output for joy value (-1.0 to 1.0)
     * @param OutTrust Output for trust value (-1.0 to 1.0)
     * @param OutFear Output for fear value (-1.0 to 1.0)
     * @param OutSurprise Output for surprise value (-1.0 to 1.0)
     * @param OutSadness Output for sadness value (-1.0 to 1.0)
     * @param OutDisgust Output for disgust value (-1.0 to 1.0)
     * @param OutAnger Output for anger value (-1.0 to 1.0)
     * @param OutAnticipation Output for anticipation value (-1.0 to 1.0)
     * @return True if successful
     */
    UFUNCTION(BlueprintCallable, Category = "Oxyde")
    static bool GetAgentEmotionVector(
        FString AgentId,
        float& OutJoy,
        float& OutTrust,
        float& OutFear,
        float& OutSurprise,
        float& OutSadness,
        float& OutDisgust,
        float& OutAnger,
        float& OutAnticipation
    );

    // ==================== Memory System ====================

    /**
     * Add a memory to an agent's memory system
     * @param AgentId Agent ID string
     * @param Category Memory category: "episodic", "semantic", "procedural", or "emotional"
     * @param Content Content of the memory
     * @param Importance Importance score (0.0 - 1.0)
     * @return True if successful
     */
    UFUNCTION(BlueprintCallable, Category = "Oxyde|Memory")
    static bool AddMemory(FString AgentId, FString Category, FString Content, float Importance);

    /**
     * Add an emotional memory to an agent's memory system
     * @param AgentId Agent ID string
     * @param Category Memory category
     * @param Content Content of the memory
     * @param Importance Importance score
     * @param Valence Emotional valence (-1.0 to 1.0)
     * @param Intensity Emotional intensity (0.0 to 1.0)
     * @return True if successful
     */
    UFUNCTION(BlueprintCallable, Category = "Oxyde|Memory")
    static bool AddEmotionalMemory(FString AgentId, FString Category, FString Content, 
        float Importance, float Valence, float Intensity);

    /**
     * Get the number of memories stored by an agent
     * @param AgentId Agent ID string
     * @return Memory count
     */
    UFUNCTION(BlueprintCallable, Category = "Oxyde|Memory")
    static int32 GetMemoryCount(FString AgentId);

    /**
     * Clear all non-permanent memories from an agent
     * @param AgentId Agent ID string
     * @return Number of memories cleared
     */
    UFUNCTION(BlueprintCallable, Category = "Oxyde|Memory")
    static int32 ClearMemories(FString AgentId);

    /**
     * Get memories by category as JSON array
     * @param AgentId Agent ID string
     * @param Category Memory category
     * @return JSON array of memories
     */
    UFUNCTION(BlueprintCallable, Category = "Oxyde|Memory")
    static FString GetMemoriesByCategory(FString AgentId, FString Category);

    /**
     * Retrieve memories relevant to a query as JSON array
     * @param AgentId Agent ID string
     * @param Query Query text
     * @param Limit Maximum number of memories
     * @return JSON array of relevant memories
     */
    UFUNCTION(BlueprintCallable, Category = "Oxyde|Memory")
    static FString RetrieveRelevantMemories(FString AgentId, FString Query, int32 Limit);

    /**
     * Forget a specific memory by ID
     * @param AgentId Agent ID string
     * @param MemoryId Memory ID to forget
     * @return True if successful
     */
    UFUNCTION(BlueprintCallable, Category = "Oxyde|Memory")
    static bool ForgetMemory(FString AgentId, FString MemoryId);

    /**
     * Forget all memories of a specific category
     * @param AgentId Agent ID string
     * @param Category Memory category to forget
     * @return Number of memories forgotten
     */
    UFUNCTION(BlueprintCallable, Category = "Oxyde|Memory")
    static int32 ForgetMemoriesByCategory(FString AgentId, FString Category);

private:
    // Native function pointers
    typedef bool (*InitFuncPtr)();
    typedef const char* (*CreateAgentFuncPtr)(const char*);
    typedef const char* (*CreateAgentFromJsonFuncPtr)(const char*);
    typedef bool (*UpdateAgentFuncPtr)(const char*, const char*);
    typedef const char* (*ProcessInputFuncPtr)(const char*, const char*);
    typedef const char* (*GetAgentStateFuncPtr)(const char*);
    typedef bool (*GetEmotionVectorFuncPtr)(const char*, float*, float*, float*, float*, float*, float*, float*, float*);
    typedef void (*FreeStringFuncPtr)(const char*);

    // Memory system function pointers
    typedef bool (*AddMemoryFuncPtr)(const char*, const char*, const char*, double);
    typedef bool (*AddEmotionalMemoryFuncPtr)(const char*, const char*, const char*, double, double, double);
    typedef uint32 (*GetMemoryCountFuncPtr)(const char*);
    typedef uint32 (*ClearMemoriesFuncPtr)(const char*);
    typedef const char* (*GetMemoriesByCategoryFuncPtr)(const char*, const char*);
    typedef const char* (*RetrieveRelevantMemoriesFuncPtr)(const char*, const char*, uint32);
    typedef bool (*ForgetMemoryFuncPtr)(const char*, const char*);
    typedef uint32 (*ForgetMemoriesByCategoryFuncPtr)(const char*, const char*);

    static InitFuncPtr InitFunc;
    static CreateAgentFuncPtr CreateAgentFunc;
    static CreateAgentFromJsonFuncPtr CreateAgentFromJsonFunc;
    static UpdateAgentFuncPtr UpdateAgentFunc;
    static ProcessInputFuncPtr ProcessInputFunc;
    static GetAgentStateFuncPtr GetAgentStateFunc;
    static GetEmotionVectorFuncPtr GetEmotionVectorFunc;
    static FreeStringFuncPtr FreeStringFunc;

    static AddMemoryFuncPtr AddMemoryFunc;
    static AddEmotionalMemoryFuncPtr AddEmotionalMemoryFunc;
    static GetMemoryCountFuncPtr GetMemoryCountFunc;
    static ClearMemoriesFuncPtr ClearMemoriesFunc;
    static GetMemoriesByCategoryFuncPtr GetMemoriesByCategoryFunc;
    static RetrieveRelevantMemoriesFuncPtr RetrieveRelevantMemoriesFunc;
    static ForgetMemoryFuncPtr ForgetMemoryFunc;
    static ForgetMemoriesByCategoryFuncPtr ForgetMemoriesByCategoryFunc;

    // Handle to the dynamic library
    static void* LibraryHandle;

    // Initialize function pointers
    static bool InitializeFunctionPointers();
};
