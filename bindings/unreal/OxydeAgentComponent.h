// Copyright Epic Games, Inc. All Rights Reserved.

#pragma once

#include "CoreMinimal.h"
#include "Kismet/BlueprintFunctionLibrary.h"
#include "OxydeUnreal.h"
#include "OxydeAgentComponent.generated.h"

/**
 * Blueprint-accessible structure for emotion data
 */
USTRUCT(BlueprintType)
struct OXYDE_API FOxydeEmotionVector
{
    GENERATED_BODY()

    /** Joy value (-1.0 to 1.0) */
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Emotion")
    float Joy = 0.0f;

    /** Anger value (-1.0 to 1.0) */
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Emotion")
    float Anger = 0.0f;

    /** Fear value (-1.0 to 1.0) */
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Emotion")
    float Fear = 0.0f;

    FOxydeEmotionVector() = default;
    
    FOxydeEmotionVector(float InJoy, float InAnger, float InFear)
        : Joy(InJoy), Anger(InAnger), Fear(InFear) {}
};

/**
 * Simple Blueprint function library to get emotion vectors from agents
 */
UCLASS()
class OXYDE_API UOxydeEmotionBP : public UBlueprintFunctionLibrary
{
    GENERATED_BODY()

public:
    /** Get emotion vector from an agent by ID */
    UFUNCTION(BlueprintCallable, Category = "Oxyde Emotion")
    static FOxydeEmotionVector GetAgentEmotionVector(const FString& AgentId);

    /** Get individual emotion values from an agent by ID */
    UFUNCTION(BlueprintCallable, Category = "Oxyde Emotion")
    static bool GetAgentEmotionValues(const FString& AgentId, float& Joy, float& Anger, float& Fear);
};
