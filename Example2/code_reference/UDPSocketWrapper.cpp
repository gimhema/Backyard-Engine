

#include "UDPSocketWrapper.h"

FUDPSocketWrapper::FUDPSocketWrapper()
{
    ISocketSubsystem* SocketSubsystem = ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM);
    if (!SocketSubsystem)
    {
        UE_LOG(LogTemp, Error, TEXT("소켓 서브시스템을 찾을 수 없음!"));
        return;
    }


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
}

FUDPSocketWrapper::~FUDPSocketWrapper()
{
    if (UdpSocket)
    {
        UdpSocket->Close();
        ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->DestroySocket(UdpSocket);
    }
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
void FUDPSocketWrapper::ReceiveMessage()
{
    // if (!UdpSocket.IsValid()) return;

    uint8 Buffer[2048];
    int32 BytesRead = 0;
    TSharedRef<FInternetAddr> Sender = ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->CreateInternetAddr();

    // 데이터 수신
    if (UdpSocket->RecvFrom(Buffer, BufferSize, BytesRead, *Sender))
    {
        FString ReceivedMessage = FString(UTF8_TO_TCHAR(reinterpret_cast<const char*>(Buffer)));
        UE_LOG(LogTemp, Log, TEXT("UDP 메시지 수신: %s (%d bytes)"), *ReceivedMessage, BytesRead);
    }
}
