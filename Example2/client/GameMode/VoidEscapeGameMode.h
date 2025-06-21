// Copyright Epic Games, Inc. All Rights Reserved.

#pragma once

#include "CoreMinimal.h"
#include "GameFramework/GameModeBase.h"
#include "UDPSocketWrapper.h"
#include "TCPSocketListener.h"
#include "Containers/CircularQueue.h"
#include <vector>
#include <queue>
#include "VoidEscapeGameMode.generated.h"

UCLASS(minimalapi)
class AVoidEscapeGameMode : public AGameModeBase
{
	GENERATED_BODY()

public:
	AVoidEscapeGameMode();

public:
	FUDPSocketWrapper* UDPSocketWrapper;
	FTCPSocketListener* TCPSocketListener;

	std::queue<std::vector<uint8_t>> TCPMessageQueue;
	std::queue<std::vector<uint8_t>> UDPMessageQueue;

public:
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Network")
	FString ServerIP;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Network")
	int32 TCPPort;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "Network")
	int32 UDPPort;

public:
	UFUNCTION(BlueprintCallable, Category = "Network")
	void InitNetwork();

public:
	void SetUpTCPConnection();
	void SetUpUDPConnection();

public:
	void SendTCPSpin();
	void SendUDPSpin();

	void PushMessageToTCPQueue(const std::vector<uint8_t>& Message)
	{
		TCPMessageQueue.push(Message);
	}

	void PushMessageToUDPQueue(const std::vector<uint8_t>& Message)
	{
		UDPMessageQueue.push(Message);
	}

public:
	virtual void Tick(float DeltaTime) override;
};



