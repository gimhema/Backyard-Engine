#include "TCPSocketListener.h"

FTCPSocketListener::FTCPSocketListener()
    : ServerSocket(nullptr), ClientSocket(nullptr), Thread(nullptr), bRunThread(true)
{
}

FTCPSocketListener::~FTCPSocketListener()
{
    StopTCPListener();
}

bool FTCPSocketListener::StartTCPListener(int32 Port)
{
    ISocketSubsystem* SocketSubsystem = ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM);
    if (!SocketSubsystem)
    {
        UE_LOG(LogTemp, Error, TEXT("소켓 서브시스템을 찾을 수 없음!"));
        return false;
    }

    ServerSocket = SocketSubsystem->CreateSocket(NAME_Stream, TEXT("TCPListener"), false);
    if (!ServerSocket)
    {
        UE_LOG(LogTemp, Error, TEXT("TCP 소켓 생성 실패!"));
        return false;
    }

    TSharedPtr<FInternetAddr> LocalAddress = SocketSubsystem->CreateInternetAddr();
    bool bIsValid;
    LocalAddress->SetIp(TEXT("0.0.0.0"), bIsValid);
    LocalAddress->SetPort(Port);

    if (!ServerSocket->Bind(*LocalAddress))
    {
        UE_LOG(LogTemp, Error, TEXT("TCP 소켓 바인딩 실패!"));
        return false;
    }

    if (!ServerSocket->Listen(8)) // 최대 8개의 연결 대기 가능
    {
        UE_LOG(LogTemp, Error, TEXT("TCP 소켓 리스닝 실패!"));
        return false;
    }

//    UE_LOG(LogTemp, Log, TEXT("TCP 서버가 포트 %d에서 실행 중"), Port);

    // 새로운 스레드에서 실행
    Thread = FRunnableThread::Create(this, TEXT("TCPListenerThread"), 0, TPri_BelowNormal);

    return true;
}

void FTCPSocketListener::StopTCPListener()
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

    if (ServerSocket)
    {
        ServerSocket->Close();
        ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->DestroySocket(ServerSocket);
        ServerSocket = nullptr;
    }
}

uint32 FTCPSocketListener::Run()
{
    while (bRunThread)
    {
        if (!ServerSocket) break;

        // 클라이언트 연결 대기
        ClientSocket = ServerSocket->Accept(TEXT("TCPClient"));
        if (ClientSocket)
        {
            UE_LOG(LogTemp, Log, TEXT("클라이언트가 연결되었습니다!"));
            ReceiveData();
        }
    }
    return 0;
}

void FTCPSocketListener::ReceiveData()
{
    if (!ClientSocket) return;

    uint8 Buffer[4096];
    int32 BytesRead = 0;

    while (bRunThread && ClientSocket->Recv(Buffer, BufferSize, BytesRead))
    {
        if (BytesRead > 0)
        {
            FString ReceivedMessage = FString(UTF8_TO_TCHAR(reinterpret_cast<const char*>(Buffer)));
            UE_LOG(LogTemp, Log, TEXT("TCP 메시지 수신: %s (%d bytes)"), *ReceivedMessage, BytesRead);
        }
    }

//    UE_LOG(LogTemp, Log, TEXT("클라이언트 연결 종료"));
    ClientSocket->Close();
    ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->DestroySocket(ClientSocket);
    ClientSocket = nullptr;
}
