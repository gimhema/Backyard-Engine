// Copyright Epic Games, Inc. All Rights Reserved.

#include "VoidEscapeGameMode.h"
#include "VoidEscapeCharacter.h"
#include "UObject/ConstructorHelpers.h"

AVoidEscapeGameMode::AVoidEscapeGameMode()
	: Super()
{
	// set default pawn class to our Blueprinted character
	static ConstructorHelpers::FClassFinder<APawn> PlayerPawnClassFinder(TEXT("/Game/FirstPerson/Blueprints/BP_FirstPersonCharacter"));
	DefaultPawnClass = PlayerPawnClassFinder.Class;

}


void AVoidEscapeGameMode::InitNetwork()
{
	udpSocketWrapper = new FUDPSocketWrapper();
	tcpSocketWrapper = new FTCPSocketListener();
}
