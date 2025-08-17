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

        while (AccumulatorBuffer.size() >= sizeof(uint32_t))
        {
            uint32_t totalLen = 0;
            FMemory::Memcpy(&totalLen, AccumulatorBuffer.data(), sizeof(uint32_t));

            // 최소 길이 보정: 길이 헤더 자체 포함이라고 가정
            if (totalLen < sizeof(uint32_t))
            {
                // 프로토콜 에러: 안전하게 드롭하거나 Accumulator 비우기
                AccumulatorBuffer.clear();
                break;
            }

            if (AccumulatorBuffer.size() < totalLen)
            {
                // 아직 패킷이 덜 옴
                break;
            }

            // 완성된 패킷 슬라이스 추출
            std::vector<uint8_t> FullMessageBytes(
                AccumulatorBuffer.begin(),
                AccumulatorBuffer.begin() + totalLen
            );

            // 게임 스레드로 넘겨 호출 (Option 2 패턴)
            TWeakObjectPtr<UVoidEscapeGameInstance> GIWeak = GameInstanceWeak;
            if (GIWeak.IsValid())
            {
                auto DataCopy = MoveTemp(FullMessageBytes);

                AsyncTask(ENamedThreads::GameThread, [GIWeak, Data = MoveTemp(DataCopy)]() mutable
                    {
                        if (UVoidEscapeGameInstance* GI = GIWeak.Get())
                        {
                            // 게임 스레드에서만 멤버 호출
                            GI->EnqueueMessage(MoveTemp(Data));
                        }
                    });
            }

            // 소비한 영역만 제거 (clear 대신 erase)
			AccumulatorBuffer.clear();
            // AccumulatorBuffer.erase(AccumulatorBuffer.begin(),
            //     AccumulatorBuffer.begin() + totalLen);
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