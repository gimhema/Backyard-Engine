// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "Sockets.h"
#include "SocketSubsystem.h"
#include "IPAddress.h"
#include "HAL/Runnable.h"
#include "HAL/RunnableThread.h"

/**
 * TCP 클라이언트 클래스
 */
class VOIDESCAPE_API FTCPSocketClient : public FRunnable
{
private:
    FSocket* ClientSocket;
    FRunnableThread* Thread;
    FThreadSafeBool bRunThread;
    const int32 BufferSize = 4096;

public:
    FTCPSocketClient();
    virtual ~FTCPSocketClient();

    /** 서버에 연결 시도 */
    bool ConnectToServer(const FString& IP, int32 Port);

    /** 서버로 메시지 전송 */
    bool SendMessage(const FString& Message);

    /** 연결 종료 및 정리 */
    void Disconnect();

private:
    /** 메시지 수신 스레드 진입점 */
    virtual uint32 Run() override;

    /** 메시지 수신 처리 */
    void ReceiveData();
};
