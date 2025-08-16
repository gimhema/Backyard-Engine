// Fill out your copyright notice in the Description page of Project Settings.


#include "TCPSocketListener.h"
#include "VoidEscape/VoidEscapeGameInstance.h"
#include "../QSM/QSM_VerifyAccount.hpp"
#include "../QSM/QSM_AllowConnectGame.hpp"

void TCPSocketListener::Exit()
{
    PrintOnScreenMessage(TEXT("TCP Client Thread Exited."), 5.0f, FColor::Yellow);
}

TCPSocketListener::TCPSocketListener()
    : ClientSocket(nullptr), Thread(nullptr), bRunThread(true)
{

    // ClientSocket = nullptr;
}

void TCPSocketListener::SetGameInstance()
{
    if (!GameInstance)
    {
        GameInstance = Cast<UVoidEscapeGameInstance>(GEngine->GetWorld()->GetGameInstance());
        if (!GameInstance)
        {
            PrintOnScreenMessage(TEXT("Not Found Game Instance."), 5.0f, FColor::Red);
        }
    }
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


		if (GameInstance)
		{
			GameInstance->ProcessMessageQueue();
		}
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
            uint32_t MessageTotalLength = 0;
            FMemory::Memcpy(&MessageTotalLength, AccumulatorBuffer.data(), sizeof(uint32_t));

            if (AccumulatorBuffer.size() < sizeof(uint32_t)) {
                break;
            }

            uint32_t message_length_prefix;
            FMemory::Memcpy(&message_length_prefix, AccumulatorBuffer.data(), sizeof(uint32_t));
            if (AccumulatorBuffer.size() < message_length_prefix) {

                break;
            }

            std::vector<uint8_t> FullMessageBytes(AccumulatorBuffer.begin(), AccumulatorBuffer.begin() + message_length_prefix);

            if (GameInstance)
            {
				// GameInstance->PrintOnScreenMessage(
                //     FString::Printf(TEXT("Casting Game Instance Valid")),
                //     2.0f, 
                //     FColor::Blue);
				GameInstance->MessageActionAllocate(FullMessageBytes);
            }
			AccumulatorBuffer.clear();
            // AccumulatorBuffer.erase(AccumulatorBuffer.begin(), AccumulatorBuffer.begin() + message_length_prefix);
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
    if (GEngine)
    {
        GEngine->AddOnScreenDebugMessage(-1, Duration, TextColor, Message);
    }
}