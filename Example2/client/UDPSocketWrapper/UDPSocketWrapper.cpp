#include "UDPSocketWrapper.h"
#include "QSM/QSM_ChatMessage.hpp"
#include "QSM/QSM_NewPlayer.hpp"
#include "QSM/QSM_PlayerMovement.hpp"

FUDPSocketWrapper::FUDPSocketWrapper()
    : UdpSocket(nullptr), Thread(nullptr), bRunThread(true)
{
    ISocketSubsystem* SocketSubsystem = ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM);
    if (!SocketSubsystem)
    {
        UE_LOG(LogTemp, Error, TEXT("소켓 서브시스템을 찾을 수 없음!"));
        return;
    }

    // UDP 소켓 생성
    UdpSocket = SocketSubsystem->CreateSocket(NAME_DGram, TEXT("MyUdpSocket"), false);
    if (!UdpSocket)
    {
        UE_LOG(LogTemp, Error, TEXT("UDP 소켓 생성 실패!"));
        return;
    }

    int32 ActualBufferSize = BufferSize;
    UdpSocket->SetNonBlocking(true);
    UdpSocket->SetReuseAddr(true);
    UdpSocket->SetRecvErr(true);
    UdpSocket->SetSendBufferSize(BufferSize, ActualBufferSize);
    UdpSocket->SetReceiveBufferSize(BufferSize, ActualBufferSize);

    // 바인딩할 주소 생성
    TSharedPtr<FInternetAddr> LocalAddress = SocketSubsystem->CreateInternetAddr();
    bool bIsValid;
    LocalAddress->SetIp(TEXT("0.0.0.0"), bIsValid);
    LocalAddress->SetPort(7777);

    if (!UdpSocket->Bind(*LocalAddress))
    {
        UE_LOG(LogTemp, Error, TEXT("UDP 소켓 바인딩 실패!"));
        return;
    }

    UE_LOG(LogTemp, Log, TEXT("UDP 소켓이 포트 7777에서 실행 중"));

    // 수신을 위한 스레드 시작
    Thread = FRunnableThread::Create(this, TEXT("UDPReceiverThread"), 0, TPri_BelowNormal);
}

FUDPSocketWrapper::~FUDPSocketWrapper()
{
    StopReceiving();
}

void FUDPSocketWrapper::StopReceiving()
{
    bRunThread = false;

    if (Thread)
    {
        Thread->Kill(true);
        delete Thread;
        Thread = nullptr;
    }

    if (UdpSocket)
    {
        UdpSocket->Close();
        ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->DestroySocket(UdpSocket);
        UdpSocket = nullptr;
    }
}

uint32 FUDPSocketWrapper::Run()
{
    uint8 Buffer[2048];
    int32 BytesRead = 0;
    TSharedPtr<FInternetAddr> Sender = ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->CreateInternetAddr();

    while (bRunThread)
    {
        if (!UdpSocket) break;

        // 데이터 수신
        if (UdpSocket->RecvFrom(Buffer, BufferSize, BytesRead, *Sender))
        {
            if (BytesRead > 0)
            {
                FString ReceivedMessage = FString(UTF8_TO_TCHAR(reinterpret_cast<const char*>(Buffer)));
                UE_LOG(LogTemp, Log, TEXT("UDP 메시지 수신: %s (%d bytes)"), *ReceivedMessage, BytesRead);
            }
        }

        // CPU 점유율 방지를 위해 약간의 대기
        FPlatformProcess::Sleep(0.01f);
    }

    return 0;
}

void FUDPSocketWrapper::SendMessage(const FString& Message, const FString& TargetIP, int32 TargetPort)
{
    if (!UdpSocket) return;

    ISocketSubsystem* SocketSubsystem = ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM);
    TSharedPtr<FInternetAddr> TargetAddress = SocketSubsystem->CreateInternetAddr();
    bool bIsValid;
    TargetAddress->SetIp(*TargetIP, bIsValid);
    TargetAddress->SetPort(TargetPort);

    if (!bIsValid)
    {
        UE_LOG(LogTemp, Error, TEXT("잘못된 대상 IP 주소!"));
        return;
    }

    // 문자열을 UTF-8 바이트 배열로 변환 후 전송
    FTCHARToUTF8 Convert(*Message);
    int32 BytesSent = 0;
    UdpSocket->SendTo((uint8*)Convert.Get(), Convert.Length(), BytesSent, *TargetAddress);

    UE_LOG(LogTemp, Log, TEXT("UDP 메시지 전송: %s (%d bytes)"), *Message, BytesSent);
}
