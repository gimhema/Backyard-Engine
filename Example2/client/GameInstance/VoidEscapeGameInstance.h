// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "Engine/GameInstance.h"
#include "TCPSocketListener.h"
#include "UDPSocketWrapper.h"
#include "VoidEscapeGameInstance.generated.h"

/**
 * 
 */
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
	TCPSocketListener* SocketListener;
	UDPSocketWrapper* udpSocketWrapper;

public:
	void PrintOnScreenMessage(const FString& Message, float Duration, FColor TextColor);

public:
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "VoidEscape|Network")
	FString tcpServerIP;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "VoidEscape|Network")
	int32 tcpServerPort;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "VoidEscape|Network")
	FString udpServerIP;
	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "VoidEscape|Network")
	int32 udpServerPort;

	
};
