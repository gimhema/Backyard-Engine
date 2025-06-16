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

	if (UDPSocketWrapper == nullptr)
	{
		// Create a new instance of the UDP socket wrapper
		UDPSocketWrapper = new FUDPSocketWrapper();
	}

	if (TCPSocketListener == nullptr)
	{
		// Create a new instance of the TCP socket listener
		TCPSocketListener = new FTCPSocketListener();
	}
}


void AVoidEscapeGameMode::InitNetwork()
{// Set up TCP and UDP connections
	SetUpTCPConnection();
	SetUpUDPConnection();
}

void AVoidEscapeGameMode::SetUpTCPConnection()
{
	if (TCPSocketListener != nullptr)
	{
		TCPSocketListener->ConnectToServer("127.0.0.1", 8080);
	}
	else
	{
		// UE_LOG(LogTemp, Error, TEXT("UDP Socket Wrapper is not initialized!"));
	}
}

void AVoidEscapeGameMode::SetUpUDPConnection()
{
	if (UDPSocketWrapper != nullptr)
	{
		UDPSocketWrapper->SetUpUDPSocket();
	}
	else
	{
		// UE_LOG(LogTemp, Error, TEXT("UDP Socket Wrapper is not initialized!"));
	}
}
