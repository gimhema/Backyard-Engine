// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "Sockets.h"
#include "SocketSubsystem.h"
#include "IPAddress.h"
#include "HAL/Runnable.h"
#include "HAL/RunnableThread.h"

/**
 * 
 */

class VOIDESCAPE_API FUDPSocketWrapper : public FRunnable
{

private:
    FSocket* UdpSocket;
    TSharedPtr<FInternetAddr> RemoteAddress;
    FRunnableThread* Thread;
    bool bRunThread;
    const int32 BufferSize = 4096; // 2KB 버퍼

    UGameInstance* GameInstance;

public:
    FUDPSocketWrapper();
    virtual ~FUDPSocketWrapper();

    void SetGameInstance();

    void SetUpUDPSocket(const FString& IP, int32 Port);
    void SendMessage(const FString& Message, const FString& TargetIP, int32 TargetPort);
    void SendMessageBinary(const std::vector<uint8_t>& Data);
    void StopReceiving();

    void PrintOnScreenMessage(const FString& Message, float Duration = 2.0f, FColor TextColor = FColor::White);

private:
    virtual uint32 Run() override;
};
