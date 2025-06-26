#include "TCPSocketListener.h"
#include "../QSM/QSM_VerifyAccount.hpp"
#include "../QSM/QSM_MessageEnum.h"
#include "../QSM/QSM_BaseMessage.h"
#include "GameNetworkInstanceSubsystem.h"

FTCPSocketListener::FTCPSocketListener()
    : ClientSocket(nullptr), Thread(nullptr), bRunThread(true)
{

	ClientSocket = nullptr;
}

void FTCPSocketListener::SetGameInstance()
{
    if (!GameInstance)
    {
        GameInstance = GEngine->GetWorld()->GetGameInstance();
        // UE_LOG(LogTemp, Error, TEXT("게임 인스턴스를 찾을 수 없습니다."));
    }
}

FTCPSocketListener::~FTCPSocketListener()
{
    Disconnect();
}

bool FTCPSocketListener::ConnectToServer(const FString& IP, int32 Port)
{
    ISocketSubsystem* SocketSubsystem = ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM);
    if (!SocketSubsystem)
    {
//        // UE_LOG(LogTemp, Error, TEXT("소켓 서브시스템을 찾을 수 없습니다."));
        return false;
    }

    ClientSocket = SocketSubsystem->CreateSocket(NAME_Stream, TEXT("TCPClient"), false);
    if (!ClientSocket)
    {
 //       // UE_LOG(LogTemp, Error, TEXT("클라이언트 소켓 생성 실패"));
        return false;
    }

    TSharedRef<FInternetAddr> ServerAddr = SocketSubsystem->CreateInternetAddr();
    bool bIsValid;
    ServerAddr->SetIp(*IP, bIsValid);
    ServerAddr->SetPort(Port);

    if (!bIsValid)
    {
   //     // UE_LOG(LogTemp, Error, TEXT("유효하지 않은 IP 주소: %s"), *IP);
        return false;
    }

    if (!ClientSocket->Connect(*ServerAddr))
    {
//        // UE_LOG(LogTemp, Error, TEXT("서버에 연결할 수 없습니다."));
        return false;
    }
    
   VerifyAccount _respConnectMsg(static_cast<int>(QFunctionType::VERIFY_ACCOUNT), 0, "TESTID", "1234", "127.0.0.1:8080");
   std::vector<uint8_t> _msgBuffer = _respConnectMsg.serialize();
   SendMessageBinary(_msgBuffer);  
 
   PrintOnScreenMessage(TEXT("SetUp TCP Socket Completed."), 5.0f, FColor::Green);

    // 수신용 스레드 시작
    Thread = FRunnableThread::Create(this, TEXT("TCPClientThread"), 0, TPri_BelowNormal);

    return true;
}

void FTCPSocketListener::Disconnect()
{
    bRunThread = false;

    if (Thread)
    {
        Thread->Kill(true);
        delete Thread;
        Thread = nullptr;
    }

    if (ClientSocket)
    {
        ClientSocket->Close();
        ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->DestroySocket(ClientSocket);
        ClientSocket = nullptr;
    }

    // UE_LOG(LogTemp, Log, TEXT("클라이언트 연결 종료"));
}

bool FTCPSocketListener::SendMessage(const FString& Message)
{
    if (!ClientSocket) return false;

    FTCHARToUTF8 Converter(*Message);
    int32 BytesSent = 0;
    bool bSuccess = ClientSocket->Send((uint8*)Converter.Get(), Converter.Length(), BytesSent);

    if (!bSuccess)
    {
        // UE_LOG(LogTemp, Error, TEXT("메시지 전송 실패"));
    }

    return bSuccess;
}

bool FTCPSocketListener::SendMessageBinary(const std::vector<uint8_t>& Data)
{
    if (!ClientSocket) return false;

    int32 BytesSent = 0;
    const uint8* RawData = Data.data();
    int32 DataSize = static_cast<int32>(Data.size());

    bool bSuccess = ClientSocket->Send(RawData, DataSize, BytesSent);

    return bSuccess && BytesSent == DataSize;
}


uint32 FTCPSocketListener::Run()
{
    ReceiveData();
    return 0;
}

void FTCPSocketListener::ReceiveData()
{
    std::vector<uint8_t> Buffer;
    int32 BytesRead = 0;

    while (bRunThread && ClientSocket && ClientSocket->Recv(Buffer.data(), BufferSize, BytesRead))
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

    // UE_LOG(LogTemp, Warning, TEXT("서버와의 연결이 끊어졌습니다."));
}

void FTCPSocketListener::PrintOnScreenMessage(const FString& Message, float Duration, FColor TextColor)
{
    if (GEngine)
    {
        GEngine->AddOnScreenDebugMessage(-1, Duration, TextColor, Message);
    }
}