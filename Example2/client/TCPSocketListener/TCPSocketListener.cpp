#include "TCPSocketListener.h"
#include "../QSM/QSM_VerifyAccount.hpp"
#include "../QSM/QSM_MessageEnum.h"
#include "../QSM/QSM_BaseMessage.h"
#include "GameNetworkInstanceSubsystem.h"

// FRunnable::Exit() 구현 (스레드가 종료될 때 호출될 로직)
void FTCPSocketListener::Exit()
{
    // 스레드 종료 시 정리 작업 (필요하다면 추가)
    PrintOnScreenMessage(TEXT("TCP Client Thread Exited."), 5.0f, FColor::Yellow);
}

FTCPSocketListener::FTCPSocketListener()
    : ClientSocket(nullptr), Thread(nullptr), bRunThread(true)
{
    // 생성자에서 ClientSocket을 nullptr로 초기화하는 것은 이미 되어 있으므로 중복 제거
    // ClientSocket = nullptr; // 이 줄은 제거해도 됩니다.
}

void FTCPSocketListener::SetGameInstance()
{
    if (!GameInstance)
    {
        GameInstance = GEngine->GetWorld()->GetGameInstance();
        if (!GameInstance)
        {
            // 실제 게임 개발 시에는 UE_LOG를 사용하는 것이 좋습니다.
//            UE_LOG(LogTemp, Error, TEXT("게임 인스턴스를 찾을 수 없습니다."));
            PrintOnScreenMessage(TEXT("Not Found Game Instance."), 5.0f, FColor::Red);
        }
    }
}

FTCPSocketListener::~FTCPSocketListener()
{
    Disconnect(); // 소멸자에서 연결 해제 호출
}

bool FTCPSocketListener::ConnectToServer(const FString& IP, int32 Port)
{
    ISocketSubsystem* SocketSubsystem = ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM);
    if (!SocketSubsystem)
    {
//        UE_LOG(LogTemp, Error, TEXT("소켓 서브시스템을 찾을 수 없습니다."));
        PrintOnScreenMessage(TEXT("Not Found Socket SubSystem."), 5.0f, FColor::Red);
        return false;
    }

    ClientSocket = SocketSubsystem->CreateSocket(NAME_Stream, TEXT("TCPClient"), false); // 블로킹 소켓으로 생성
    if (!ClientSocket)
    {
  //      UE_LOG(LogTemp, Error, TEXT("클라이언트 소켓 생성 실패"));
        PrintOnScreenMessage(TEXT("Failed Create Client Socket"), 5.0f, FColor::Red);
        return false;
    }

    TSharedRef<FInternetAddr> ServerAddr = SocketSubsystem->CreateInternetAddr();
    bool bIsValid;
    ServerAddr->SetIp(*IP, bIsValid);
    ServerAddr->SetPort(Port);

    if (!bIsValid)
    {
    //    UE_LOG(LogTemp, Error, TEXT("유효하지 않은 IP 주소: %s"), *IP);
        PrintOnScreenMessage(FString::Printf(TEXT("Invalid IP : %s"), *IP), 5.0f, FColor::Red);
        return false;
    }

    int32 BytesRead = 0;

    // Connect 타임아웃 설정 (선택 사항이지만 권장)
    ClientSocket->SetNoDelay(true); // Nagle 알고리즘 비활성화 (지연 최소화)
    ClientSocket->SetReceiveBufferSize(BufferSize, BytesRead); // 수신 버퍼 크기 설정
    ClientSocket->SetSendBufferSize(BufferSize, BytesRead); // 송신 버퍼 크기 설정

    if (!ClientSocket->Connect(*ServerAddr))
    {
//        UE_LOG(LogTemp, Error, TEXT("서버에 연결할 수 없습니다. IP: %s, Port: %d"), *IP, Port);
        PrintOnScreenMessage(FString::Printf(TEXT("Failed Connected Server IP: %s, Port: %d"), *IP, Port), 5.0f, FColor::Red);
        // 연결 실패 시 소켓 정리
        ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->DestroySocket(ClientSocket);
        ClientSocket = nullptr;
        return false;
    }

    PrintOnScreenMessage(TEXT("SetUp TCP Socket Completed. Sending VERIFY_ACCOUNT..."), 5.0f, FColor::Green);

    // VERIFY_ACCOUNT 메시지 전송
    VerifyAccount _respConnectMsg;
    _respConnectMsg.mid = static_cast<uint32_t>(QFunctionType::VERIFY_ACCOUNT);
    _respConnectMsg.userId = "TESTID";
    _respConnectMsg.userName = "TESTNAME";
    _respConnectMsg.password = "1234";
    _respConnectMsg.connect_info = "127.0.0.1"; // TODO: 실제 클라이언트 IP/포트 또는 세션 식별자로 변경 필요

    std::vector<uint8_t> _msgBuffer = _respConnectMsg.serialize();
    if (!SendMessageBinary(_msgBuffer))
    {
//        UE_LOG(LogTemp, Error, TEXT("VERIFY_ACCOUNT 메시지 전송 실패"));
        PrintOnScreenMessage(TEXT("VERIFY_ACCOUNT Message Send Failed"), 5.0f, FColor::Red);
        Disconnect(); // 메시지 전송 실패 시 연결 해제
        return false;
    }

    PrintOnScreenMessage(TEXT("VERIFY_ACCOUNT message sent. Starting Receive Thread..."), 5.0f, FColor::Green);

    // 수신용 스레드 시작
    // 스레드가 이미 실행 중이 아닐 때만 생성
    if (!Thread)
    {
        bRunThread = true; // 스레드 시작 전에 실행 플래그 true로 설정
        Thread = FRunnableThread::Create(this, TEXT("TCPClientThread"), 0, TPri_BelowNormal);
    }

    return true;
}

void FTCPSocketListener::Disconnect()
{
    PrintOnScreenMessage(TEXT("Disconnecting TCP Socket..."), 5.0f, FColor::Red);
    bRunThread = false; // 스레드에 종료 신호 보냄

    if (Thread)
    {
        Thread->WaitForCompletion(); // 스레드가 완전히 종료될 때까지 대기
        delete Thread;
        Thread = nullptr;
    }

    if (ClientSocket)
    {
        ClientSocket->Close();
        ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->DestroySocket(ClientSocket);
        ClientSocket = nullptr;
    }

    AccumulatorBuffer.clear(); // 누적 버퍼 초기화
//    UE_LOG(LogTemp, Log, TEXT("클라이언트 연결 종료"));
    PrintOnScreenMessage(TEXT("Exit Client Connect"), 5.0f, FColor::Red);
}

bool FTCPSocketListener::SendMessage(const FString& Message)
{
    if (!ClientSocket) // 소켓 유효성 및 연결 상태 확인
    {
  //      UE_LOG(LogTemp, Warning, TEXT("메시지 전송 실패: 소켓이 연결되지 않았습니다."));
        return false;
    }

    FTCHARToUTF8 Converter(*Message);
    int32 BytesSent = 0;
    bool bSuccess = ClientSocket->Send((uint8*)Converter.Get(), Converter.Length(), BytesSent);

    if (!bSuccess)
    {
//        UE_LOG(LogTemp, Error, TEXT("메시지 전송 실패 (문자열). Error Code: %d"), (int32)ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->Get </*LastErrorMessage*/FString>(0).Len()); // 에러 코드 확인 함수는 실제 환경에 따라 다를 수 있음
        PrintOnScreenMessage(TEXT("Failed Send Message"), 5.0f, FColor::Red);
    }

    return bSuccess && BytesSent == Converter.Length(); // 모든 바이트가 전송되었는지 확인
}

bool FTCPSocketListener::SendMessageBinary(const std::vector<uint8_t>& Data)
{
    if (!ClientSocket) // 소켓 유효성 및 연결 상태 확인
    {
//        UE_LOG(LogTemp, Warning, TEXT("바이너리 메시지 전송 실패: 소켓이 연결되지 않았습니다."));
        return false;
    }

    int32 BytesSent = 0;
    const uint8* RawData = Data.data();
    int32 DataSize = static_cast<int32>(Data.size());

    bool bSuccess = ClientSocket->Send(RawData, DataSize, BytesSent);

    if (!bSuccess)
    {
//        UE_LOG(LogTemp, Error, TEXT("바이너리 메시지 전송 실패. Error Code: %d"), (int32)ISocketSubsystem::Get(PLATFORM_SOCKETSUBSYSTEM)->GetLastErrorCode());
        PrintOnScreenMessage(TEXT("Failed Send binary"), 5.0f, FColor::Red);
    }

    return bSuccess && BytesSent == DataSize; // 모든 바이트가 전송되었는지 확인
}


uint32 FTCPSocketListener::Run()
{
    PrintOnScreenMessage(TEXT("TCP Client Thread Started."), 5.0f, FColor::Green);
    // 이 루프가 스레드의 메인 실행 루프가 됩니다.
    while (bRunThread)
    {
        ReceiveData(); // 데이터를 수신하고 처리
    }
    return 0; // 스레드가 종료되면 0 반환
}

void FTCPSocketListener::ReceiveData()
{
    // 실제 수신할 데이터를 임시로 담을 버퍼
    std::vector<uint8_t> TempRecvBuffer;
    TempRecvBuffer.resize(BufferSize); // 버퍼 크기 올바르게 초기화

    int32 BytesRead = 0;

    // 소켓이 유효하고 연결되어 있을 때만 데이터를 받도록 함
    if (!ClientSocket)
    {
        PrintOnScreenMessage(TEXT("ReceiveData failed."), 5.0f, FColor::Red);
        bRunThread = false; // 연결이 끊겼으므로 스레드 종료 신호
        return;
    }

    bool bReceived = ClientSocket->Recv(TempRecvBuffer.data(), BufferSize, BytesRead); // 블로킹 모드

    if (bReceived && BytesRead > 0)
    {
        PrintOnScreenMessage("Sucess Recv Message ! ! !", 30.0f, FColor::Red);
        PrintOnScreenMessage("Sucess Recv Message ! ! !", 30.0f, FColor::Red);
        PrintOnScreenMessage("Sucess Recv Message ! ! !", 30.0f, FColor::Red);
        PrintOnScreenMessage("Sucess Recv Message ! ! !", 30.0f, FColor::Red);
        PrintOnScreenMessage("Sucess Recv Message ! ! !", 30.0f, FColor::Red);

        // 데이터가 성공적으로 읽혔다면 누적 버퍼에 추가
        AccumulatorBuffer.insert(AccumulatorBuffer.end(), TempRecvBuffer.begin(), TempRecvBuffer.begin() + BytesRead);

        while (AccumulatorBuffer.size() >= sizeof(uint32_t)) // 최소한 메시지 ID (4바이트)를 읽을 수 있는지 확인
        {
            uint32_t MessageTotalLength = 0;
            FMemory::Memcpy(&MessageTotalLength, AccumulatorBuffer.data(), sizeof(uint32_t));

            if (AccumulatorBuffer.size() < sizeof(uint32_t)) { // 최소한 메시지 총 길이 필드를 읽을 수 있는지 확인
                break; // 메시지 총 길이 필드조차 불충분
            }

            uint32_t message_length_prefix; // 메시지 전체 길이 (프리픽스 포함)
            FMemory::Memcpy(&message_length_prefix, AccumulatorBuffer.data(), sizeof(uint32_t));


            if (AccumulatorBuffer.size() < message_length_prefix) {
                // 아직 완전한 메시지가 도착하지 않았다면 더 많은 데이터를 기다림
                break;
            }

            // 완전한 메시지 데이터를 추출
            std::vector<uint8_t> FullMessageBytes(AccumulatorBuffer.begin(), AccumulatorBuffer.begin() + message_length_prefix);

            // 메시지 처리
            BaseMessage _recvMessage = BaseMessage::deserialize(FullMessageBytes); // 전체 메시지 바이트로 역직렬화
            EServerMessageType _msgType = static_cast<EServerMessageType>(_recvMessage.id);



            UGameNetworkInstanceSubsystem* MsgSubsystem = GameInstance->GetSubsystem<UGameNetworkInstanceSubsystem>();
            if (MsgSubsystem)
            {
                PrintOnScreenMessage(FString::Printf(TEXT("Received message type %d"), static_cast<int32>(_msgType)), 2.0f, FColor::Blue);
                MsgSubsystem->DispatchMessage(_msgType, FullMessageBytes); // 전체 메시지 바이트 전달
            }
            else
            {
                PrintOnScreenMessage(TEXT("GameNetworkInstanceSubsystem Failed!"), 2.0f, FColor::Orange);
            }

            // 처리된 메시지 바이트만큼 누적 버퍼에서 제거
            AccumulatorBuffer.erase(AccumulatorBuffer.begin(), AccumulatorBuffer.begin() + message_length_prefix);
        }
    }
    else if (!bReceived) // Recv가 false를 반환 (오류 또는 연결 종료 가능성)
    {
        ESocketConnectionState ConnectionState = ClientSocket->GetConnectionState();
        if (ConnectionState == SCS_NotConnected || ConnectionState == SCS_ConnectionError)
        {
            PrintOnScreenMessage(TEXT("DisConnect Server!"), 5.0f, FColor::Red);
            bRunThread = false; // 스레드 종료 신호
        }
        else
        {
            FPlatformProcess::Sleep(0.01f); // 10ms 슬립
        }
    }
    else if (bReceived && BytesRead == 0) // Recv가 true를 반환했지만 읽은 바이트가 0인 경우 (상대방이 연결을 우아하게 종료)
    {
        PrintOnScreenMessage(TEXT("Exit Server."), 5.0f, FColor::Red);
        bRunThread = false; // 스레드 종료 신호
    }
}

void FTCPSocketListener::PrintOnScreenMessage(const FString& Message, float Duration, FColor TextColor)
{
    if (GEngine)
    {
        GEngine->AddOnScreenDebugMessage(-1, Duration, TextColor, Message);
    }
}