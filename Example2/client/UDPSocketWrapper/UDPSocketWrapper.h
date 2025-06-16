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

    void SetUpUDPSocket();
    void SendMessage(const FString& Message, const FString& TargetIP, int32 TargetPort);
    void StopReceiving();

private:
    virtual uint32 Run() override;
};
