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
class VOIDESCAPE_API FTCPSocketListener : public FRunnable
{
private:
    FSocket* ClientSocket;
    FRunnableThread* Thread;
    FThreadSafeBool bRunThread;
    const int32 BufferSize = 4096;

    UGameInstance* GameInstance;

public:
    FTCPSocketListener();
    virtual ~FTCPSocketListener();

    void SetGameInstance();

    /** 서버에 연결 시도 */
    bool ConnectToServer(const FString& IP, int32 Port);

    /** 서버로 메시지 전송 */
    bool SendMessage(const FString& Message);

    bool SendMessageBinary(const std::vector<uint8_t>& Data);

    /** 연결 종료 및 정리 */
    void Disconnect();

    void PrintOnScreenMessage(const FString& Message, float Duration = 2.0f, FColor TextColor = FColor::White);

private:
    /** 메시지 수신 스레드 진입점 */
    virtual uint32 Run() override;

    /** 메시지 수신 처리 */
    void ReceiveData();
};
