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
class VOIDESCAPE_API TCPSocketListener : public FRunnable
{
public:
	TCPSocketListener();
	~TCPSocketListener();

public:
    virtual bool Init() override { return true; }
    virtual uint32 Run() override;
    virtual void Stop() override { bRunThread = false; }
    virtual void Exit() override;

    bool ConnectToServer(const FString& IP, int32 Port);

    void Disconnect();


    bool SendMessage(const FString& Message);


    bool SendMessageBinary(const std::vector<uint8_t>& Data);


    void SetGameInstance();

private:
    FSocket* ClientSocket;
    FRunnableThread* Thread;
    FThreadSafeBool bRunThread;

    class UVoidEscapeGameInstance* GameInstance;

    static const int32 BufferSize = 4096;

    std::vector<uint8_t> AccumulatorBuffer;

    void ReceiveData();

    void PrintOnScreenMessage(const FString& Message, float Duration, FColor TextColor);

};
