// Copyright Epic Games, Inc. All Rights Reserved.

#include "OxydeUnreal.h"
#include "HAL/PlatformProcess.h"
#include "Misc/Paths.h"
#include "Misc/FileHelper.h"
#include "Logging/LogMacros.h"

DEFINE_LOG_CATEGORY_STATIC(LogOxyde, Log, All);

// Static members initialization
void* OxydeUnreal::LibraryHandle = nullptr;
OxydeUnreal::InitFuncPtr OxydeUnreal::InitFunc = nullptr;
OxydeUnreal::CreateAgentFuncPtr OxydeUnreal::CreateAgentFunc = nullptr;
OxydeUnreal::UpdateAgentFuncPtr OxydeUnreal::UpdateAgentFunc = nullptr;
OxydeUnreal::ProcessInputFuncPtr OxydeUnreal::ProcessInputFunc = nullptr;
OxydeUnreal::GetAgentStateFuncPtr OxydeUnreal::GetAgentStateFunc = nullptr;
OxydeUnreal::FreeStringFuncPtr OxydeUnreal::FreeStringFunc = nullptr;

bool OxydeUnreal::Init()
{
    // Initialize function pointers if needed
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return false;
    }

    // Call native init function
    return InitFunc();
}

FString OxydeUnreal::CreateAgent(const char* ConfigPath)
{
    // Initialize function pointers if needed
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return FString();
    }

    // Call native create agent function
    const char* result = CreateAgentFunc(ConfigPath);
    if (result == nullptr)
    {
        return FString();
    }

    FString agentId(UTF8_TO_TCHAR(result));
    FreeStringFunc(result);
    return agentId;
}

bool OxydeUnreal::UpdateAgentContext(const char* AgentId, const char* ContextJson)
{
    // Initialize function pointers if needed
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return false;
    }

    // Call native update agent function
    return UpdateAgentFunc(AgentId, ContextJson);
}

FString OxydeUnreal::ProcessInput(const char* AgentId, const char* Input)
{
    // Initialize function pointers if needed
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return FString();
    }

    // Call native process input function
    const char* result = ProcessInputFunc(AgentId, Input);
    if (result == nullptr)
    {
        return FString();
    }

    FString response(UTF8_TO_TCHAR(result));
    FreeStringFunc(result);
    return response;
}

FString OxydeUnreal::GetAgentState(const char* AgentId)
{
    // Initialize function pointers if needed
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return FString();
    }

    // Call native get agent state function
    const char* result = GetAgentStateFunc(AgentId);
    if (result == nullptr)
    {
        return FString();
    }

    FString stateJson(UTF8_TO_TCHAR(result));
    FreeStringFunc(result);
    return stateJson;
}

bool OxydeUnreal::InitializeFunctionPointers()
{
    // Skip if already initialized
    if (LibraryHandle != nullptr)
    {
        return true;
    }

    // Get path to the library
    FString libraryName;

#if PLATFORM_WINDOWS
    libraryName = TEXT("oxyde.dll");
#elif PLATFORM_MAC
    libraryName = TEXT("liboxyde.dylib");
#elif PLATFORM_LINUX
    libraryName = TEXT("liboxyde.so");
#else
    UE_LOG(LogOxyde, Error, TEXT("Unsupported platform for Oxyde SDK"));
    return false;
#endif

    // Find the plugin's binary directory
    FString binaryPath = FPaths::ProjectPluginsDir() / TEXT("Oxyde/Binaries/ThirdParty/");

#if PLATFORM_WINDOWS
    binaryPath = binaryPath / TEXT("Win64");
#elif PLATFORM_MAC
    binaryPath = binaryPath / TEXT("Mac");
#elif PLATFORM_LINUX
    binaryPath = binaryPath / TEXT("Linux");
#endif

    FString libraryPath = FPaths::ConvertRelativePathToFull(binaryPath / libraryName);
    
    // Load the library
    LibraryHandle = FPlatformProcess::GetDllHandle(*libraryPath);
    if (LibraryHandle == nullptr)
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to load Oxyde SDK library: %s"), *libraryPath);
        return false;
    }

    // Get function pointers
    InitFunc = (InitFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_init"));
    CreateAgentFunc = (CreateAgentFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_create_agent"));
    UpdateAgentFunc = (UpdateAgentFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_update_agent"));
    ProcessInputFunc = (ProcessInputFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_process_input"));
    GetAgentStateFunc = (GetAgentStateFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_get_agent_state"));
    FreeStringFunc = (FreeStringFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_free_string"));

    // Check that all functions were found
    if (InitFunc == nullptr ||
        CreateAgentFunc == nullptr ||
        UpdateAgentFunc == nullptr ||
        ProcessInputFunc == nullptr ||
        GetAgentStateFunc == nullptr ||
        FreeStringFunc == nullptr)
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to load one or more Oxyde SDK functions"));
        FPlatformProcess::FreeDllHandle(LibraryHandle);
        LibraryHandle = nullptr;
        return false;
    }

    UE_LOG(LogOxyde, Log, TEXT("Oxyde SDK library loaded successfully"));
    return true;
}
