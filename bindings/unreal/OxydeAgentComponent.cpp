// Copyright Epic Games, Inc. All Rights Reserved.

#include "OxydeAgentComponent.h"
#include "OxydeUnreal.h"

FOxydeEmotionVector UOxydeEmotionBP::GetAgentEmotionVector(const FString& AgentId)
{
    float joy, anger, fear;
    bool success = OxydeUnreal::GetAgentEmotionVector(TCHAR_TO_UTF8(*AgentId), &joy, &anger, &fear);
    
    if (success)
    {
        return FOxydeEmotionVector(joy, anger, fear);
    }
    
    return FOxydeEmotionVector(); // Default: 0, 0, 0
}

bool UOxydeEmotionBP::GetAgentEmotionValues(const FString& AgentId, float& Joy, float& Anger, float& Fear)
{
    return OxydeUnreal::GetAgentEmotionVector(TCHAR_TO_UTF8(*AgentId), &Joy, &Anger, &Fear);
}
