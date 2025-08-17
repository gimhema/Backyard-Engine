// Copyright Epic Games, Inc. All Rights Reserved.

#pragma once

#include "CoreMinimal.h"
#include "GameFramework/GameModeBase.h"
#include "VoidEscapeGameMode.generated.h"

UCLASS(minimalapi)
class AVoidEscapeGameMode : public AGameModeBase
{
	GENERATED_BODY()

public:
	AVoidEscapeGameMode();

public:
	virtual void Tick(float DeltaSeconds) override;

public:
	void PrintOnScreenMessage(const FString& Message, float Duration, FColor TextColor);
};



