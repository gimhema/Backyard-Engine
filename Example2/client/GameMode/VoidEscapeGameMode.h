// Copyright Epic Games, Inc. All Rights Reserved.

#pragma once

#include "CoreMinimal.h"
#include "GameFramework/GameModeBase.h"
#include "UDPSocketWrapper.h"
#include "TCPSocketListener.h"
#include "VoidEscapeGameMode.generated.h"

UCLASS(minimalapi)
class AVoidEscapeGameMode : public AGameModeBase
{
	GENERATED_BODY()

public:
	AVoidEscapeGameMode();

public:

public:
	UFUNCTION(BlueprintCallable, Category = "Network")
	void InitNetwork();

public:

	void SetUpTCPConnection();
	void SetUpUDPConnection();
};



