#include "UDPSocketWrapper.h"
#include "QSM/QSM_ChatMessage.hpp"
#include "QSM/QSM_NewPlayer.hpp"
#include "QSM/QSM_PlayerMovement.hpp"
#include "QSM/QSM_BaseMessage.h"
#include "GameNetworkInstanceSubsystem.h"


FUDPSocketWrapper::FUDPSocketWrapper()
    : UdpSocket(nullptr), Thread(nullptr), bRunThread(true)
{
    if (!GameInstance)
    {
        GameInstance = GEngine->GetWorld()->GetGameInstance();
        // UE_LOG(LogTemp, Error, TEXT("게임 인스턴스를 찾을 수 없습니다."));
    }
}

void FUDPSocketWrapper::SetGameInstance()
{
    if (!GameInstance)
    {
        GameInstance = GEngine->GetWorld()->GetGameInstance();
        // UE_LOG(LogTemp, Error, TEXT("게임 인스턴스를 찾을 수 없습니다."));
    }
}

void FUDPSocketWrapper::SetUpUDPSocket(const FString& IP, int32 Port)
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
    LocalAddress->SetIp(*IP, bIsValid);
    LocalAddress->SetPort(Port);

    if (!UdpSocket->Bind(*LocalAddress))
    {
        // UE_LOG(LogTemp, Error, TEXT("UDP 소켓 바인딩 실패!"));
        return;
    }

    // UE_LOG(LogTemp, Log, TEXT("UDP 소켓이 포트 7777에서 실행 중"));

	PrintOnScreenMessage(TEXT("SetUp UDP Socket Completed."), 5.0f, FColor::Green);

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
    std::vector<uint8_t> Buffer;
    int32 BytesRead = 0;
    TSharedPtr<FInternetAddr> Sender = ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->CreateInternetAddr();

    while (bRunThread)
    {
        if (!UdpSocket) break;

        // 데이터 수신
        if (UdpSocket->RecvFrom(Buffer.data(), BufferSize, BytesRead, *Sender))
        {
            if (BytesRead > 0)
            {
                // FString Received = FString(UTF8_TO_TCHAR(reinterpret_cast<const char*>(Buffer)));
                BaseMessage _recvMessage = BaseMessage::deserialize(Buffer);

                EServerMessageType _msgType = static_cast<EServerMessageType>(_recvMessage.id);

                UGameNetworkInstanceSubsystem* MsgSubsystem = GameInstance->GetSubsystem<UGameNetworkInstanceSubsystem>();
                if (MsgSubsystem)
                {
                    MsgSubsystem->DispatchMessage(_msgType, Buffer);
                }
                // UE_LOG(LogTemp, Log, TEXT("서버로부터 수신된 메시지: %s"), *Received);
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

void FUDPSocketWrapper::SendMessageBinary(const std::vector<uint8_t>& Data)
{
    if (!UdpSocket) return;
	TSharedPtr<FInternetAddr> TargetAddress = RemoteAddress; // 이미 설정된 원격 주소 사용
	if (!TargetAddress.IsValid())
	{
		UE_LOG(LogTemp, Error, TEXT("원격 주소가 설정되지 않았습니다."));
		return;
	}
	int32 BytesSent = 0;
	const uint8* RawData = Data.data();
	int32 DataSize = static_cast<int32>(Data.size());
	bool bSuccess = UdpSocket->SendTo(RawData, DataSize, BytesSent, *TargetAddress);
//	return bSuccess && BytesSent == DataSize;
}

void FUDPSocketWrapper::PrintOnScreenMessage(const FString& Message, float Duration, FColor TextColor)
{
    if (GEngine)
    {
        GEngine->AddOnScreenDebugMessage(-1, Duration, TextColor, Message);
    }
}