// Copyright Epic Games, Inc. All Rights Reserved.

#pragma once

#include "CoreMinimal.h"

/**
 * C++ bindings for the Oxyde SDK in Unreal Engine
 */
class OXYDE_API OxydeUnreal
{
public:
    /**
     * Initialize the Oxyde SDK
     * @return True if initialization was successful
     */
    static bool Init();

    /**
     * Create a new agent from a configuration file
     * @param ConfigPath Path to the agent configuration file
     * @return Agent ID string or empty if failed
     */
    static FString CreateAgent(const char* ConfigPath);

    /**
     * Update an agent with new context data
     * @param AgentId Agent ID string
     * @param ContextJson JSON string with context data
     * @return True if update was successful
     */
    static bool UpdateAgentContext(const char* AgentId, const char* ContextJson);

    /**
     * Process input for an agent
     * @param AgentId Agent ID string
     * @param Input Input text
     * @return Agent's response
     */
    static FString ProcessInput(const char* AgentId, const char* Input);

    /**
     * Get the current state of an agent
     * @param AgentId Agent ID string
     * @return JSON string with agent state
     */
    static FString GetAgentState(const char* AgentId);

private:
    // Native function pointers
    typedef bool (*InitFuncPtr)();
    typedef const char* (*CreateAgentFuncPtr)(const char*);
    typedef bool (*UpdateAgentFuncPtr)(const char*, const char*);
    typedef const char* (*ProcessInputFuncPtr)(const char*, const char*);
    typedef const char* (*GetAgentStateFuncPtr)(const char*);
    typedef void (*FreeStringFuncPtr)(const char*);

    static InitFuncPtr InitFunc;
    static CreateAgentFuncPtr CreateAgentFunc;
    static UpdateAgentFuncPtr UpdateAgentFunc;
    static ProcessInputFuncPtr ProcessInputFunc;
    static GetAgentStateFuncPtr GetAgentStateFunc;
    static FreeStringFuncPtr FreeStringFunc;

    // Handle to the dynamic library
    static void* LibraryHandle;

    // Initialize function pointers
    static bool InitializeFunctionPointers();
};
