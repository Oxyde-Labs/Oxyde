// Copyright Epic Games, Inc. All Rights Reserved.

#include "OxydeUnreal.h"
#include "HAL/PlatformProcess.h"
#include "Misc/Paths.h"
#include "Misc/FileHelper.h"
#include "Logging/LogMacros.h"

DEFINE_LOG_CATEGORY_STATIC(LogOxyde, Log, All);

// Static members initialization
void* UOxydeLibrary::LibraryHandle = nullptr;
UOxydeLibrary::InitFuncPtr UOxydeLibrary::InitFunc = nullptr;
UOxydeLibrary::CreateAgentFuncPtr UOxydeLibrary::CreateAgentFunc = nullptr;
UOxydeLibrary::CreateAgentFromJsonFuncPtr UOxydeLibrary::CreateAgentFromJsonFunc = nullptr;
UOxydeLibrary::UpdateAgentFuncPtr UOxydeLibrary::UpdateAgentFunc = nullptr;
UOxydeLibrary::ProcessInputFuncPtr UOxydeLibrary::ProcessInputFunc = nullptr;
UOxydeLibrary::GetAgentStateFuncPtr UOxydeLibrary::GetAgentStateFunc = nullptr;
UOxydeLibrary::GetEmotionVectorFuncPtr UOxydeLibrary::GetEmotionVectorFunc = nullptr;
UOxydeLibrary::FreeStringFuncPtr UOxydeLibrary::FreeStringFunc = nullptr;

UOxydeLibrary::AddMemoryFuncPtr UOxydeLibrary::AddMemoryFunc = nullptr;
UOxydeLibrary::AddEmotionalMemoryFuncPtr UOxydeLibrary::AddEmotionalMemoryFunc = nullptr;
UOxydeLibrary::GetMemoryCountFuncPtr UOxydeLibrary::GetMemoryCountFunc = nullptr;
UOxydeLibrary::ClearMemoriesFuncPtr UOxydeLibrary::ClearMemoriesFunc = nullptr;
UOxydeLibrary::GetMemoriesByCategoryFuncPtr UOxydeLibrary::GetMemoriesByCategoryFunc = nullptr;
UOxydeLibrary::RetrieveRelevantMemoriesFuncPtr UOxydeLibrary::RetrieveRelevantMemoriesFunc = nullptr;
UOxydeLibrary::ForgetMemoryFuncPtr UOxydeLibrary::ForgetMemoryFunc = nullptr;
UOxydeLibrary::ForgetMemoriesByCategoryFuncPtr UOxydeLibrary::ForgetMemoriesByCategoryFunc = nullptr;

bool UOxydeLibrary::Init()
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

FString UOxydeLibrary::CreateAgent(FString ConfigPath)
{
    // Initialize function pointers if needed
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return FString();
    }

    // Call native create agent function
    const char* result = CreateAgentFunc(TCHAR_TO_UTF8(*ConfigPath));
    if (result == nullptr)
    {
        return FString();
    }

    FString agentId(UTF8_TO_TCHAR(result));
    FreeStringFunc(result);
    return agentId;
}

FString UOxydeLibrary::CreateAgentFromJson(FString JsonConfig)
{
    // Initialize function pointers if needed
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return FString();
    }

    // Call native create agent function
    const char* result = CreateAgentFromJsonFunc(TCHAR_TO_UTF8(*JsonConfig));
    if (result == nullptr)
    {
        return FString();
    }

    FString agentId(UTF8_TO_TCHAR(result));
    FreeStringFunc(result);
    return agentId;
}

FString UOxydeLibrary::CreateAgentFromContent(FString ContentPath)
{
    // Construct full path relative to Content directory
    FString FullPath = FPaths::ProjectContentDir() / ContentPath;
    
    FString JsonContent;
    if (!FFileHelper::LoadFileToString(JsonContent, *FullPath))
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to load agent config from content path: %s"), *FullPath);
        return FString();
    }
    
    // Convert JSON content to UTF-8 char* for the native function
    return CreateAgentFromJson(JsonContent);
}

bool UOxydeLibrary::UpdateAgentContext(FString AgentId, FString ContextJson)
{
    // Initialize function pointers if needed
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return false;
    }

    // Call native update agent function
    return UpdateAgentFunc(TCHAR_TO_UTF8(*AgentId), TCHAR_TO_UTF8(*ContextJson));
}

FString UOxydeLibrary::ProcessInput(FString AgentId, FString Input)
{
    // Initialize function pointers if needed
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return FString();
    }

    // Call native process input function
    const char* result = ProcessInputFunc(TCHAR_TO_UTF8(*AgentId), TCHAR_TO_UTF8(*Input));
    if (result == nullptr)
    {
        return FString();
    }

    FString response(UTF8_TO_TCHAR(result));
    FreeStringFunc(result);
    return response;
}

FString UOxydeLibrary::GetAgentState(FString AgentId)
{
    // Initialize function pointers if needed
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return FString();
    }

    // Call native get agent state function
    const char* result = GetAgentStateFunc(TCHAR_TO_UTF8(*AgentId));
    if (result == nullptr)
    {
        return FString();
    }

    FString stateJson(UTF8_TO_TCHAR(result));
    FreeStringFunc(result);
    return stateJson;
}

bool UOxydeLibrary::InitializeFunctionPointers()
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
    CreateAgentFromJsonFunc = (CreateAgentFromJsonFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_create_agent_from_json"));
    UpdateAgentFunc = (UpdateAgentFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_update_agent"));
    ProcessInputFunc = (ProcessInputFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_process_input"));
    GetAgentStateFunc = (GetAgentStateFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_get_agent_state"));
    GetEmotionVectorFunc = (GetEmotionVectorFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_get_emotion_vector"));
    FreeStringFunc = (FreeStringFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_free_string"));

    // Memory system functions
    AddMemoryFunc = (AddMemoryFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_add_memory"));
    AddEmotionalMemoryFunc = (AddEmotionalMemoryFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_add_emotional_memory"));
    GetMemoryCountFunc = (GetMemoryCountFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_get_memory_count"));
    ClearMemoriesFunc = (ClearMemoriesFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_clear_memories"));
    GetMemoriesByCategoryFunc = (GetMemoriesByCategoryFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_get_memories_by_category"));
    RetrieveRelevantMemoriesFunc = (RetrieveRelevantMemoriesFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_retrieve_relevant_memories"));
    ForgetMemoryFunc = (ForgetMemoryFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_forget_memory"));
    ForgetMemoriesByCategoryFunc = (ForgetMemoriesByCategoryFuncPtr)FPlatformProcess::GetDllExport(LibraryHandle, TEXT("oxyde_unreal_forget_memories_by_category"));

    // Check that all functions were found
    if (InitFunc == nullptr ||
        CreateAgentFunc == nullptr ||
        CreateAgentFromJsonFunc == nullptr ||
        UpdateAgentFunc == nullptr ||
        ProcessInputFunc == nullptr ||
        GetAgentStateFunc == nullptr ||
        GetEmotionVectorFunc == nullptr ||
        FreeStringFunc == nullptr ||
        AddMemoryFunc == nullptr ||
        AddEmotionalMemoryFunc == nullptr ||
        GetMemoryCountFunc == nullptr ||
        ClearMemoriesFunc == nullptr ||
        GetMemoriesByCategoryFunc == nullptr ||
        RetrieveRelevantMemoriesFunc == nullptr ||
        ForgetMemoryFunc == nullptr ||
        ForgetMemoriesByCategoryFunc == nullptr)
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to load one or more Oxyde SDK functions"));
        FPlatformProcess::FreeDllHandle(LibraryHandle);
        LibraryHandle = nullptr;
        return false;
    }

    UE_LOG(LogOxyde, Log, TEXT("Oxyde SDK library loaded successfully"));
    return true;
}

bool UOxydeLibrary::GetAgentEmotionVector(
    FString AgentId,
    float& OutJoy,
    float& OutTrust,
    float& OutFear,
    float& OutSurprise,
    float& OutSadness,
    float& OutDisgust,
    float& OutAnger,
    float& OutAnticipation
)
{
    if (!GetEmotionVectorFunc)
    {
        UE_LOG(LogOxyde, Error, TEXT("GetEmotionVectorFunc not initialized"));
        return false;
    }

    return GetEmotionVectorFunc(
        TCHAR_TO_UTF8(*AgentId),
        &OutJoy,
        &OutTrust,
        &OutFear,
        &OutSurprise,
        &OutSadness,
        &OutDisgust,
        &OutAnger,
        &OutAnticipation
    );
}

// ==================== Memory System ====================

bool UOxydeLibrary::AddMemory(FString AgentId, FString Category, FString Content, float Importance)
{
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return false;
    }

    // Cast float to double for native function
    return AddMemoryFunc(TCHAR_TO_UTF8(*AgentId), TCHAR_TO_UTF8(*Category), TCHAR_TO_UTF8(*Content), (double)Importance);
}

bool UOxydeLibrary::AddEmotionalMemory(FString AgentId, FString Category, FString Content, 
    float Importance, float Valence, float Intensity)
{
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return false;
    }

    // Cast floats to doubles for native function
    return AddEmotionalMemoryFunc(
        TCHAR_TO_UTF8(*AgentId), 
        TCHAR_TO_UTF8(*Category), 
        TCHAR_TO_UTF8(*Content), 
        (double)Importance, 
        (double)Valence, 
        (double)Intensity
    );
}

int32 UOxydeLibrary::GetMemoryCount(FString AgentId)
{
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return 0;
    }

    return (int32)GetMemoryCountFunc(TCHAR_TO_UTF8(*AgentId));
}

int32 UOxydeLibrary::ClearMemories(FString AgentId)
{
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return 0;
    }

    return (int32)ClearMemoriesFunc(TCHAR_TO_UTF8(*AgentId));
}

FString UOxydeLibrary::GetMemoriesByCategory(FString AgentId, FString Category)
{
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return FString();
    }

    const char* result = GetMemoriesByCategoryFunc(TCHAR_TO_UTF8(*AgentId), TCHAR_TO_UTF8(*Category));
    if (result == nullptr)
    {
        return FString();
    }

    FString memoriesJson(UTF8_TO_TCHAR(result));
    FreeStringFunc(result);
    return memoriesJson;
}

FString UOxydeLibrary::RetrieveRelevantMemories(FString AgentId, FString Query, int32 Limit)
{
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return FString();
    }

    const char* result = RetrieveRelevantMemoriesFunc(TCHAR_TO_UTF8(*AgentId), TCHAR_TO_UTF8(*Query), (uint32)Limit);
    if (result == nullptr)
    {
        return FString();
    }

    FString memoriesJson(UTF8_TO_TCHAR(result));
    FreeStringFunc(result);
    return memoriesJson;
}

bool UOxydeLibrary::ForgetMemory(FString AgentId, FString MemoryId)
{
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return false;
    }

    return ForgetMemoryFunc(TCHAR_TO_UTF8(*AgentId), TCHAR_TO_UTF8(*MemoryId));
}

int32 UOxydeLibrary::ForgetMemoriesByCategory(FString AgentId, FString Category)
{
    if (!InitializeFunctionPointers())
    {
        UE_LOG(LogOxyde, Error, TEXT("Failed to initialize Oxyde SDK function pointers"));
        return 0;
    }

    return (int32)ForgetMemoriesByCategoryFunc(TCHAR_TO_UTF8(*AgentId), TCHAR_TO_UTF8(*Category));
}
