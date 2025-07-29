// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "Subsystems/GameInstanceSubsystem.h"
#include "GameNetworkInstanceSubsystem.generated.h"

DECLARE_DELEGATE_OneParam(FMessageHandler, const TArray<uint8>&)

/**
 *      DEFAULT = 0,
        SEND_MESSAGE_TO_ALL = 1,
        SEND_MESSAGE_TO_TARGET = 2,
        ECHO_MESSAGE = 3,
        CHAT_MESSAGE = 4,
        PLAYER_MOVEMENT_UPDATE = 5,
        NEW_PLAYER = 6,
        MAKE_ACCOUNT = 7,
        VERIFY_ACCOUNT = 8,
        ENTER_NEW_PAYER = 9,
        DELETE_PLAYER = 10,
        ALLOW_CONNECT_GAME = 11,
        SERVER_RESPONSE = 12,
        ENTER_PLAYER_TO_GAME = 13,
        END = 14
 * 
 */
UENUM(BlueprintType)
enum class EServerMessageType : uint8
{
	DEFAULT        UMETA(DisplayName = "DEFAULT"),
	SEND_MESSAGE_TO_ALL        UMETA(DisplayName = "SEND_MESSAGE_TO_ALL"),
	SEND_MESSAGE_TO_TARGET        UMETA(DisplayName = "SEND_MESSAGE_TO_TARGET"),
	ECHO_MESSAGE        UMETA(DisplayName = "ECHO_MESSAGE"),
	CHAT      UMETA(DisplayName = "CHAT"),
    PLAYER_MOVEMENT        UMETA(DisplayName = "PLAYER_MOVEMENT"),
    NEW_PLAYER        UMETA(DisplayName = "NEW_PLAYER"),
	MAKE_ACCOUNT        UMETA(DisplayName = "MAKE_ACCOUNT"),
	VERIFY_ACCOUNT        UMETA(DisplayName = "VERIFY_ACCOUNT"),
	ENTER_NEW_PLAYER        UMETA(DisplayName = "ENTER_NEW_PLAYER"),
	DELETE_PLAYER        UMETA(DisplayName = "DELETE_PLAYER"),
	ALLOW_CONNECT_GAME        UMETA(DisplayName = "ALLOW_CONNECT_GAME"),
	SERVER_RESPONSE        UMETA(DisplayName = "SERVER_RESPONSE"),
	ENTER_PLAYER_TO_GAME        UMETA(DisplayName = "ENTER_PLAYER_TO_GAME"),
	END        UMETA(DisplayName = "END")
};

//class FGameMessageFuncHandler
//{
//public:
//	void ChatMessage(const TArray<uint8>& Payload);
//	void CreateNewPlayer(const TArray<uint8>& Payload);
//	void HandleMove(const TArray<uint8>& Payload);
//};

UCLASS()
class VOIDESCAPE_API UGameNetworkInstanceSubsystem : public UGameInstanceSubsystem
{
	GENERATED_BODY()
	
public:
	void RegisterHandler(EServerMessageType MessageType, FMessageHandler Handler);
	void DispatchMessage(EServerMessageType MessageType, std::vector<uint8_t>& Payload);
	void InitFunctionHandler();
	void PrintOnScreenMessage(const FString& Message, float Duration, FColor TextColor);

private:
	TMap<EServerMessageType, FMessageHandler> HandlerMap;


};
