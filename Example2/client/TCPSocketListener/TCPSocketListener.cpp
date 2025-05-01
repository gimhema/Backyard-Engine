#include "TCPSocketListener.h"

FTCPSocketListener::FTCPSocketListener()
    : ClientSocket(nullptr), Thread(nullptr), bRunThread(true)
{
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

 //   // UE_LOG(LogTemp, Log, TEXT("서버에 연결됨: %s:%d"), *IP, Port);

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

uint32 FTCPSocketListener::Run()
{
    ReceiveData();
    return 0;
}

void FTCPSocketListener::ReceiveData()
{
    uint8 Buffer[4096];
    int32 BytesRead = 0;

    while (bRunThread && ClientSocket && ClientSocket->Recv(Buffer, BufferSize, BytesRead))
    {
        if (BytesRead > 0)
        {
            FString Received = FString(UTF8_TO_TCHAR(reinterpret_cast<const char*>(Buffer)));
            // UMyServerMessageSubsystem* MsgSubsystem = GetGameInstance()->GetSubsystem<UMyServerMessageSubsystem>();
            // if (MsgSubsystem)
            // {
            //     MsgSubsystem->DispatchMessage("Move", ReceivedData);
            // }
            // UE_LOG(LogTemp, Log, TEXT("서버로부터 수신된 메시지: %s"), *Received);
        }
    }

    // UE_LOG(LogTemp, Warning, TEXT("서버와의 연결이 끊어졌습니다."));
}

