// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/GameInstanceSubsystem.h"
#include "GameNetworkInstanceSubsystem.generated.h"

DECLARE_DELEGATE_OneParam(FMessageHandler, const TArray<uint8>&)

/**
 * 
 */
UENUM(BlueprintType)
enum class EServerMessageType : uint8
{
	DEFAULT        UMETA(DisplayName = "DEFAULT"),
	CHAT      UMETA(DisplayName = "CHAT"),
	NEW_PLAYER        UMETA(DisplayName = "NEW_PLAYER"),
	PLAYER_MOVEMENT        UMETA(DisplayName = "PLAYER_MOVEMENT"),
};

class FMyHandler
{
public:
	void ChatMessage(const TArray<uint8>& Payload);
	void CreateNewPlayer(const TArray<uint8>& Payload);
	void HandleMove(const TArray<uint8>& Payload);
};


UCLASS()
class VOIDESCAPE_API UGameNetworkInstanceSubsystem : public UGameInstanceSubsystem
{
	GENERATED_BODY()
	
public:
	void RegisterHandler(EServerMessageType MessageType, FMessageHandler Handler);
	void DispatchMessage(EServerMessageType MessageType, const TArray<uint8>& Payload);
	void InitFunctionHandler();

private:
	TMap<EServerMessageType, FMessageHandler> HandlerMap;
};
