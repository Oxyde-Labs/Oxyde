// Copyright Epic Games, Inc. All Rights Reserved.

#include "OxydeAgentComponent.h"
#include "OxydeUnreal.h"

FOxydeEmotionVector UOxydeEmotionBP::GetAgentEmotionVector(const FString& AgentId)
{
    float joy, trust, fear, surprise, sadness, disgust, anger, anticipation;
    bool success = OxydeUnreal::GetAgentEmotionVector(
        TCHAR_TO_UTF8(*AgentId),
        &joy, &trust, &fear, &surprise, &sadness, &disgust, &anger, &anticipation
    );
    
    if (success)
    {
        return FOxydeEmotionVector(joy, trust, fear, surprise, sadness, disgust, anger, anticipation);
    }
    
    return FOxydeEmotionVector(); // Default: all 0.0
}

bool UOxydeEmotionBP::GetAgentEmotionValues(
    const FString& AgentId,
    float& Joy,
    float& Trust,
    float& Fear,
    float& Surprise,
    float& Sadness,
    float& Disgust,
    float& Anger,
    float& Anticipation
)
{
    return OxydeUnreal::GetAgentEmotionVector(
        TCHAR_TO_UTF8(*AgentId),
        &Joy, &Trust, &Fear, &Surprise, &Sadness, &Disgust, &Anger, &Anticipation
    );
}
