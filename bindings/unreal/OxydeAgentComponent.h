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

    /** Trust value (-1.0 to 1.0) */
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Emotion")
    float Trust = 0.0f;

    /** Fear value (-1.0 to 1.0) */
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Emotion")
    float Fear = 0.0f;

    /** Surprise value (-1.0 to 1.0) */
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Emotion")
    float Surprise = 0.0f;

    /** Sadness value (-1.0 to 1.0) */
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Emotion")
    float Sadness = 0.0f;

    /** Disgust value (-1.0 to 1.0) */
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Emotion")
    float Disgust = 0.0f;

    /** Anger value (-1.0 to 1.0) */
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Emotion")
    float Anger = 0.0f;

    /** Anticipation value (-1.0 to 1.0) */
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Emotion")
    float Anticipation = 0.0f;

    FOxydeEmotionVector() = default;
    
    FOxydeEmotionVector(float InJoy, float InTrust, float InFear, float InSurprise, 
                       float InSadness, float InDisgust, float InAnger, float InAnticipation)
        : Joy(InJoy), Trust(InTrust), Fear(InFear), Surprise(InSurprise),
          Sadness(InSadness), Disgust(InDisgust), Anger(InAnger), Anticipation(InAnticipation) {}
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
    static bool GetAgentEmotionValues(
        const FString& AgentId,
        float& Joy,
        float& Trust,
        float& Fear,
        float& Surprise,
        float& Sadness,
        float& Disgust,
        float& Anger,
        float& Anticipation
    );
};
