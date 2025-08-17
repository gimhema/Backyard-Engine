// Copyright Epic Games, Inc. All Rights Reserved.

#include "VoidEscapeGameMode.h"
#include "VoidEscapeCharacter.h"
#include "VoidEscapeGameInstance.h"
#include "QSM/QSM_BaseMessage.h"
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

	while (GI->TryDequeue(Msg)) // 소비자는 오직 여기 한 곳
	{
		// PrintOnScreenMessage("Message Parse . . .", 3.0f, FColor::Red);
		BaseMessage BaseMsg = BaseMessage::deserialize(Msg);

		EServerMessageType MessageType = static_cast<EServerMessageType>(BaseMsg.id);

		switch (MessageType)
		{
			// Game Instance Actions
		case EServerMessageType::ALLOW_CONNECT_GAME: // Example case for a specific message type
			// Handle the message accordingly
			PrintOnScreenMessage("Received ALLOW_CONNECT_GAME message", 3.0f, FColor::Red);


			// PushMessageToQueue(Message);

			break;
		default:
			// Handle unknown message type or default case
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