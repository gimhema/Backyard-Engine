// Copyright Epic Games, Inc. All Rights Reserved.

#include "VoidEscapeGameMode.h"
#include "VoidEscapeCharacter.h"
#include "VoidEscapeGameInstance.h"
#include "QSM/QSM_BaseMessage.h"
#include "QSM/QSM_AllowConnectGame.hpp"
#include "UObject/ConstructorHelpers.h"

AVoidEscapeGameMode::AVoidEscapeGameMode()
	: Super()
{
	PrimaryActorTick.bCanEverTick = true;
	PrimaryActorTick.bStartWithTickEnabled = true; // 안전빵

	// set default pawn class to our Blueprinted character
	static ConstructorHelpers::FClassFinder<APawn> PlayerPawnClassFinder(TEXT("/Game/FirstPerson/Blueprints/BP_FirstPersonCharacter"));
	DefaultPawnClass = PlayerPawnClassFinder.Class;

}

void AVoidEscapeGameMode::Tick(float DeltaSeconds)
{
	Super::Tick(DeltaSeconds);
	// You can add any game mode specific logic here that needs to be executed every frame

	// PrintOnScreenMessage("Tick 1", 1.0f, FColor::Red);

	auto* GI = GetGameInstance<UVoidEscapeGameInstance>();
	if (!GI) return;

	//PrintOnScreenMessage("Tick 2", 1.0f, FColor::Blue);

	std::vector<uint8_t> Msg;

    while (GI->TryDequeue(Msg))
    {
        PrintOnScreenMessage("STEP 1", 1.0f, FColor::Blue);
        BaseMessage BaseMsg = BaseMessage::deserialize(Msg);
        EServerMessageType MessageType = static_cast<EServerMessageType>(BaseMsg.id);

        switch (MessageType)
        {
        case EServerMessageType::ALLOW_CONNECT_GAME:
        {
            PrintOnScreenMessage("STEP 2", 1.0f, FColor::Blue);
            AllowConnectGame recvMessage = AllowConnectGame::deserialize(Msg);

            // (선택 권장) 실패 프레임 또는 부분 패킷일 때 응답을 피하기 위한 1줄 가드
            if (recvMessage.mid != static_cast<uint32_t>(EServerMessageType::ALLOW_CONNECT_GAME)) {
                PrintOnScreenMessage("ALLOW_CONNECT_GAME EXCPETION", 1.0f, FColor::Blue);
                break;
            }

            AllowConnectGame resp;
            resp.mid = static_cast<uint32_t>(EServerMessageType::ALLOW_CONNECT_GAME);
            resp.sessionId = recvMessage.sessionId;
            resp.pid = recvMessage.pid;
            resp.accountId = "TEST_CONNECTED_PLAYER";
            resp.name = "TEST_CONNECTED_PLAYER_NAME";
            resp.connect_info = "127.0.0.1";
            GI->SendMessageBinary(resp.serialize());
        }
        break;

        default:
            {
                PrintOnScreenMessage("UNSUPPORTED MESSAGE", 1.0f, FColor::Blue);
				PrintOnScreenMessage(FString::Printf(TEXT("Message Type: %d"), static_cast<int32_t>(MessageType)), 1.0f, FColor::Red);
            }
            break;
        }
    }
}

void AVoidEscapeGameMode::PrintOnScreenMessage(const FString& Message, float Duration, FColor TextColor)
{
	if (GEngine)
	{
		GEngine->AddOnScreenDebugMessage(-1, Duration, TextColor, Message);
	}
}