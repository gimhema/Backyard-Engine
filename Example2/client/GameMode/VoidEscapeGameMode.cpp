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

void AVoidEscapeGameMode::InitGameInstance()
{
	if (UDPSocketWrapper == nullptr)
	{
		// Create a new instance of the UDP socket wrapper
		UDPSocketWrapper = new FUDPSocketWrapper();
		UDPSocketWrapper->SetGameInstance();
	}

	if (TCPSocketListener == nullptr)
	{
		// Create a new instance of the TCP socket listener
		TCPSocketListener = new FTCPSocketListener();
		TCPSocketListener->SetGameInstance();
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
		// 8080
		TCPSocketListener->ConnectToServer(ServerIP, TCPPort);
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
		// 8081
		UDPSocketWrapper->SetUpUDPSocket(ServerIP, UDPPort);
	}
	else
	{
		// UE_LOG(LogTemp, Error, TEXT("UDP Socket Wrapper is not initialized!"));
	}
}
void AVoidEscapeGameMode::SendTCPSpin()
{
	if (TCPSocketListener != nullptr)
	{
		if (!TCPMessageQueue.empty())
		{
			std::vector<uint8_t> Message = TCPMessageQueue.front();

			TCPSocketListener->SendMessageBinary(Message);

			TCPMessageQueue.pop(); // Remove the message after sending
		}
		else
		{
			// UE_LOG(LogTemp, Warning, TEXT("TCP Message Queue is empty!"));
		}
	}
	else
	{
		// UE_LOG(LogTemp, Error, TEXT("TCP Socket Listener is not initialized!"));
	}
}
void AVoidEscapeGameMode::SendUDPSpin()
{
	if (UDPSocketWrapper != nullptr)
	{
		if (!UDPMessageQueue.empty())
		{
			std::vector<uint8_t> Message = UDPMessageQueue.front();

			UDPSocketWrapper->SendMessageBinary(Message);

			UDPMessageQueue.pop(); // Remove the message after sending

		}
		else
		{
			// UE_LOG(LogTemp, Warning, TEXT("UDP Message Queue is empty!"));
		}
	}
	else
	{
		// UE_LOG(LogTemp, Error, TEXT("UDP Socket Wrapper is not initialized!"));
	}
}
void AVoidEscapeGameMode::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);
	// Example of sending messages
	SendTCPSpin();
	SendUDPSpin();
}