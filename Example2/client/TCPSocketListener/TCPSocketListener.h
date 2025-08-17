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

    explicit TCPSocketListener(class UVoidEscapeGameInstance* InGI);
	// TCPSocketListener();
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


    // UObject는 직접 오래 붙들면 위험하니 Weak로 보관 (안전하게 게임 스레드로 되돌려 호출할 것)
    TWeakObjectPtr<class UVoidEscapeGameInstance> GameInstanceWeak;
    // class UVoidEscapeGameInstance* GameInstance;

    static const int32 BufferSize = 4096;

    std::vector<uint8_t> AccumulatorBuffer;

    void ReceiveData();

    void PrintOnScreenMessage(const FString& Message, float Duration, FColor TextColor);


public:

};
