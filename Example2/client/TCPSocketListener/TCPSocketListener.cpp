// Fill out your copyright notice in the Description page of Project Settings.


#include "TCPSocketListener.h"
#include "VoidEscape/VoidEscapeGameInstance.h"
#include "../QSM/QSM_VerifyAccount.hpp"
#include "../QSM/QSM_AllowConnectGame.hpp"

void TCPSocketListener::Exit()
{
    PrintOnScreenMessage(TEXT("TCP Client Thread Exited."), 5.0f, FColor::Yellow);
}

TCPSocketListener::TCPSocketListener(UVoidEscapeGameInstance* InGI)
    : ClientSocket(nullptr)
    , Thread(nullptr)
    , bRunThread(true)
    , GameInstanceWeak(InGI) // 게임 스레드에서 주입받은 GI를 Weak로 저장
{
}

void TCPSocketListener::SetGameInstance()
{
    PrintOnScreenMessage(TEXT("Set Game Instance."), 5.0f, FColor::Red);
    // if (!GameInstance)
    // {
    //     GameInstance = Cast<UVoidEscapeGameInstance>(GEngine->GetWorld()->GetGameInstance());
    //     if (!GameInstance)
    //     {

    //     }
    // }
}

TCPSocketListener::~TCPSocketListener()
{
    Disconnect();
}

bool TCPSocketListener::ConnectToServer(const FString& IP, int32 Port)
{
    ISocketSubsystem* SocketSubsystem = ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM);
    if (!SocketSubsystem)
    {
        PrintOnScreenMessage(TEXT("Not Found Socket SubSystem."), 5.0f, FColor::Red);
        return false;
    }

    ClientSocket = SocketSubsystem->CreateSocket(NAME_Stream, TEXT("TCPClient"), false);
    if (!ClientSocket)
    {
        PrintOnScreenMessage(TEXT("Failed Create Client Socket"), 5.0f, FColor::Red);
        return false;
    }

    TSharedRef<FInternetAddr> ServerAddr = SocketSubsystem->CreateInternetAddr();
    bool bIsValid;
    ServerAddr->SetIp(*IP, bIsValid);
    ServerAddr->SetPort(Port);

    if (!bIsValid)
    {
        PrintOnScreenMessage(FString::Printf(TEXT("Invalid IP : %s"), *IP), 5.0f, FColor::Red);
        return false;
    }

    int32 BytesRead = 0;

    ClientSocket->SetNoDelay(true);
    ClientSocket->SetReceiveBufferSize(BufferSize, BytesRead);
    ClientSocket->SetSendBufferSize(BufferSize, BytesRead);

    if (!ClientSocket->Connect(*ServerAddr))
    {
        PrintOnScreenMessage(FString::Printf(TEXT("Failed Connected Server IP: %s, Port: %d"), *IP, Port), 5.0f, FColor::Red);

        ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->DestroySocket(ClientSocket);
        ClientSocket = nullptr;
        return false;
    }


    if (!Thread)
    {
        bRunThread = true;
        Thread = FRunnableThread::Create(this, TEXT("TCPClientThread"), 0, TPri_BelowNormal);
    }

    return true;
}

void TCPSocketListener::Disconnect()
{
    PrintOnScreenMessage(TEXT("Disconnecting TCP Socket..."), 5.0f, FColor::Red);
    bRunThread = false;

    if (Thread)
    {
        Thread->WaitForCompletion();
        delete Thread;
        Thread = nullptr;
    }

    if (ClientSocket)
    {
        ClientSocket->Close();
        ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->DestroySocket(ClientSocket);
        ClientSocket = nullptr;
    }

    AccumulatorBuffer.clear();

    PrintOnScreenMessage(TEXT("Exit Client Connect"), 5.0f, FColor::Red);
}

bool TCPSocketListener::SendMessage(const FString& Message)
{
    if (!ClientSocket)
    {

        return false;
    }

    FTCHARToUTF8 Converter(*Message);
    int32 BytesSent = 0;
    bool bSuccess = ClientSocket->Send((uint8*)Converter.Get(), Converter.Length(), BytesSent);

    if (!bSuccess)
    {
        PrintOnScreenMessage(TEXT("Failed Send Message"), 5.0f, FColor::Red);
    }

    return bSuccess && BytesSent == Converter.Length();
}

bool TCPSocketListener::SendMessageBinary(const std::vector<uint8_t>& Data)
{
    if (!ClientSocket)
    {

        return false;
    }

    int32 BytesSent = 0;
    const uint8* RawData = Data.data();
    int32 DataSize = static_cast<int32>(Data.size());

    bool bSuccess = ClientSocket->Send(RawData, DataSize, BytesSent);

    if (!bSuccess)
    {

        PrintOnScreenMessage(TEXT("Failed Send binary"), 5.0f, FColor::Red);
    }

    return bSuccess && BytesSent == DataSize;
}


uint32 TCPSocketListener::Run()
{
    PrintOnScreenMessage(TEXT("TCP Client Thread Started."), 5.0f, FColor::Green);




    while (bRunThread)
    {
        ReceiveData();


		// if (GameInstance)
		// {
		// 	GameInstance->ProcessMessageQueue();
		// }
    }
    return 0;
}

void TCPSocketListener::ReceiveData()
{
    std::vector<uint8_t> TempRecvBuffer;
    TempRecvBuffer.resize(BufferSize);

    int32 BytesRead = 0;


    if (!ClientSocket)
    {

        PrintOnScreenMessage(TEXT("ReceiveData failed."), 5.0f, FColor::Red);
        bRunThread = false;
        return;
    }

    bool bReceived = ClientSocket->Recv(TempRecvBuffer.data(), BufferSize, BytesRead, ESocketReceiveFlags::None);


    if (bReceived && BytesRead > 0)
    {

        AccumulatorBuffer.insert(AccumulatorBuffer.end(), TempRecvBuffer.begin(), TempRecvBuffer.begin() + BytesRead);

        while (AccumulatorBuffer.size() >= 4)
        {
            bool processed = false;

            // ------- 케이스 1: [length | payload(mid..)] 프리픽스가 있는 경우 -------
            {
                uint32_t payloadLen = 0;
                FMemory::Memcpy(&payloadLen, AccumulatorBuffer.data(), 4);

                // payload의 최소 길이: u32*3(12) + len*3(12) = 24 이상이어야 정상
                const size_t need = 4 + static_cast<size_t>(payloadLen);
                if (payloadLen >= 24 && AccumulatorBuffer.size() >= need)
                {
                    // ★ payload만 추출 (길이 4바이트는 버림). payload의 첫 4바이트가 mid(id)!
                    std::vector<uint8_t> Payload(
                        AccumulatorBuffer.begin() + 4,
                        AccumulatorBuffer.begin() + need
                    );

                    if (auto GI = GameInstanceWeak.Get())
                    {
                        AsyncTask(ENamedThreads::GameThread, [GI, Data = MoveTemp(Payload)]() mutable {
                            GI->EnqueueMessage(MoveTemp(Data)); // mid부터
                            });
                    }

                    AccumulatorBuffer.erase(AccumulatorBuffer.begin(),
                        AccumulatorBuffer.begin() + need);
                    processed = true;
                }
            }
            if (processed) continue;

            // ------- 케이스 2: 프리픽스 없이 바로 mid부터 시작하는 경우 -------
            if (AccumulatorBuffer.size() >= 24)
            {
                // [mid(0) | sessionId(4) | pid(8) | accLen(12) | nameLen(16) | connLen(20) | .. ]
                uint32_t accLen = 0, nameLen = 0, connLen = 0;
                FMemory::Memcpy(&accLen, AccumulatorBuffer.data() + 12, 4);
                FMemory::Memcpy(&nameLen, AccumulatorBuffer.data() + 16, 4);
                FMemory::Memcpy(&connLen, AccumulatorBuffer.data() + 20, 4);

                // 안전 가드 (필요시 상한 조정)
                const uint32_t MAX_FIELD = 1u << 20; // 1MB
                if (accLen <= MAX_FIELD && nameLen <= MAX_FIELD && connLen <= MAX_FIELD)
                {
                    const size_t need = 24ull + accLen + nameLen + connLen;
                    if (AccumulatorBuffer.size() >= need)
                    {
                        // ★ 프리픽스가 없으므로, 0부터 need까지가 완전한 payload (첫 4바이트가 mid)
                        std::vector<uint8_t> Payload(
                            AccumulatorBuffer.begin(),
                            AccumulatorBuffer.begin() + need
                        );

                        if (auto GI = GameInstanceWeak.Get())
                        {
                            AsyncTask(ENamedThreads::GameThread, [GI, Data = MoveTemp(Payload)]() mutable {
                                GI->EnqueueMessage(MoveTemp(Data)); // mid부터
                                });
                        }

                        AccumulatorBuffer.erase(AccumulatorBuffer.begin(),
                            AccumulatorBuffer.begin() + need);
                        processed = true;
                    }
                }
            }

            // 둘 다 아니면 데이터가 더 필요함
            if (!processed) break;
        }
    }
    else if (!bReceived)
    {
        ESocketConnectionState ConnectionState = ClientSocket->GetConnectionState();
        if (ConnectionState == SCS_NotConnected || ConnectionState == SCS_ConnectionError)
        {
            PrintOnScreenMessage(TEXT("DisConnect Server!"), 5.0f, FColor::Red);
            bRunThread = false;
        }
        else
        {
            FPlatformProcess::Sleep(0.01f);
        }
    }
    else if (bReceived && BytesRead == 0)
    {
        PrintOnScreenMessage(TEXT("Exit Server."), 5.0f, FColor::Red);
        bRunThread = false;
    }
}

void TCPSocketListener::PrintOnScreenMessage(const FString& Message, float Duration, FColor TextColor)
{
    AsyncTask(ENamedThreads::GameThread, [M = Message, Duration, TextColor]()
        {
            if (GEngine)
            {
                GEngine->AddOnScreenDebugMessage(-1, Duration, TextColor, M);
            }
        });

    /*if (GEngine)
    {
        GEngine->AddOnScreenDebugMessage(-1, Duration, TextColor, Message);
    }*/
}