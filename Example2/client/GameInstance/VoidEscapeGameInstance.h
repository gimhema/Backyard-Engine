// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "Engine/GameInstance.h"
#include "TCPSocketListener.h"
#include "UDPSocketWrapper.h"
#include "Containers/Queue.h"
#include "VoidEscapeGameInstance.generated.h"

/**
 * 
 */
UENUM()
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

UCLASS()
class VOIDESCAPE_API UVoidEscapeGameInstance : public UGameInstance
{
	GENERATED_BODY()

public:
	UVoidEscapeGameInstance();


	UFUNCTION(BlueprintCallable)
	void CreateSocket();

	UFUNCTION(BlueprintCallable, Category = "VoidEscape")
	void ConnectToServer();
	UFUNCTION(BlueprintCallable, Category = "VoidEscape")
	void DisconnectFromServer();
	UFUNCTION(BlueprintCallable, Category = "VoidEscape")
	void SendMessage(const FString& Message);

	UFUNCTION(BlueprintCallable)
	void CheckGameInstance();

public:
	TCPSocketListener* SocketListener = nullptr;
	UDPSocketWrapper* udpSocketWrapper;

public:
	void PrintOnScreenMessage(const FString& Message, float Duration, FColor TextColor);

public:
	// Blueprint properties
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "VoidEscape|Network")
	FString tcpServerIP;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "VoidEscape|Network")
	int32 tcpServerPort;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "VoidEscape|Network")
	FString udpServerIP;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "VoidEscape|Network")
	int32 udpServerPort;


public:
	// Game Instance Task Alloc
	void MessageActionAllocate(std::vector<uint8_t> Message);

	// Message Queue for Game Instance
	int32 queueSize = 10;

	TQueue<std::vector<uint8_t>, EQueueMode::Mpsc> MessageQueue; // Thread-safe queue for messages



	// 생산자가 호출
	FORCEINLINE void EnqueueMessage(std::vector<uint8_t>&& Msg)
	{
		PrintOnScreenMessage("EnqueueMessage", 3.0f, FColor::Red);
		MessageQueue.Enqueue(MoveTemp(Msg));   // 여러 스레드에서 안전
		// if (MessageQueue) { MsgEvent->Trigger(); }
	}

	// 소비자가 호출(게임 스레드 1곳)
	FORCEINLINE bool TryDequeue(std::vector<uint8_t>& Out)
	{
		// PrintOnScreenMessage("DequeueMessage", 3.0f, FColor::Red);
		return MessageQueue.Dequeue(Out); // 소비자 스레드에서만 호출 (문서 규약)
	}



	void PushMessageToQueue(std::vector<uint8_t> Message);
	void ProcessMessageQueue();

	UFUNCTION(BlueprintCallable)
	void SendVerifyAccount();
	
	void SendMessageBinary(const std::vector<uint8_t>& Data);

public:
	// Message Action
	void DoMessageAction(const std::vector<uint8_t>& Message);


};
