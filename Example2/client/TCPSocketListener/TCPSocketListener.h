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
public:
    FTCPSocketListener();
    ~FTCPSocketListener();

    // FRunnable 인터페이스 구현
    virtual bool Init() override { return true; } // 초기화, 특별한 초기화가 없으면 true 반환
    virtual uint32 Run() override;
    virtual void Stop() override { bRunThread = false; } // 스레드 중지 요청
    virtual void Exit() override; // 스레드 종료 시 호출

    // TCP 서버 연결
    bool ConnectToServer(const FString& IP, int32 Port);

    // TCP 연결 해제
    void Disconnect();

    // 메시지 전송 (FString)
    bool SendMessage(const FString& Message);

    // 메시지 전송 (바이너리 데이터)
    bool SendMessageBinary(const std::vector<uint8_t>& Data);

    // 게임 인스턴스 설정
    void SetGameInstance();

private:
    FSocket* ClientSocket;
    FRunnableThread* Thread;
    FThreadSafeBool bRunThread; // 스레드 안전한 불리언 플래그

    UGameInstance* GameInstance; // 게임 인스턴스 참조

    // 메시지 수신 버퍼 사이즈 (임의의 값, 필요에 따라 조정)
    static const int32 BufferSize = 8192; // 8KB 수신 버퍼

    // TCP 스트림에서 부분 메시지를 누적할 버퍼
    std::vector<uint8_t> AccumulatorBuffer;

    // 데이터 수신 처리 함수
    void ReceiveData();

    // 화면에 디버그 메시지 출력
    void PrintOnScreenMessage(const FString& Message, float Duration, FColor TextColor);
};
