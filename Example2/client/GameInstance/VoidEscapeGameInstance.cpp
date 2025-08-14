// Fill out your copyright notice in the Description page of Project Settings.


#include "VoidEscapeGameInstance.h"
#include "QSM/QSM_VerifyAccount.hpp"


UVoidEscapeGameInstance::UVoidEscapeGameInstance()
{
	// Constructor logic if needed
}

void UVoidEscapeGameInstance::CreateSocket()
{
	if (!SocketListener)
	{
		SocketListener = new TCPSocketListener();
	}
	else
	{
		delete SocketListener;
		SocketListener = new TCPSocketListener();
	}

	if (!udpSocketWrapper)
	{
		udpSocketWrapper = new UDPSocketWrapper();
	}
	else
	{
		delete udpSocketWrapper;
		udpSocketWrapper = new UDPSocketWrapper();
	}
}

void UVoidEscapeGameInstance::ConnectToServer()
{
	if (SocketListener->ConnectToServer(tcpServerIP, tcpServerPort))
	{
		SocketListener->SetGameInstance();
	}
	else
	{
		delete SocketListener;
	}
}

void UVoidEscapeGameInstance::DisconnectFromServer()
{
	SocketListener->Disconnect();
	delete SocketListener;
}

void UVoidEscapeGameInstance::SendMessage(const FString& Message)
{
	if (SocketListener->SendMessage(Message))
	{
		// Message sent successfully
	}
	else
	{
		// Handle send failure
	}
	delete SocketListener;
}

void UVoidEscapeGameInstance::PrintOnScreenMessage(const FString& Message, float Duration, FColor TextColor)
{
	if (GEngine)
	{
		GEngine->AddOnScreenDebugMessage(-1, Duration, TextColor, Message);
	}
}

void UVoidEscapeGameInstance::MessageActionAllocate(std::vector<uint8_t> Message)
{
	// This function can be used to allocate resources for message processing
	// For now, it does nothing but can be extended later

	// Message parse, and find messge unique

	int MessageType = 0; // Example: Determine message type from the first byte or some other logic

	switch (MessageType)
		{
		case 0: // Example case for a specific message type
			// Handle the message accordingly
			PushMessageToQueue(Message);
			break;
		default:
			// Handle unknown message type or default case
			break;
		}
}

void UVoidEscapeGameInstance::CheckGameInstance()
{
	PrintOnScreenMessage("Game Instance Valid", 10.0, FColor::Blue);
}

void UVoidEscapeGameInstance::PushMessageToQueue(const std::vector<uint8_t>& Message)
{
	GameInstanceMessageQueue.Enqueue(Message);
}

void UVoidEscapeGameInstance::ProcessMessageQueue()
{
	std::vector<uint8_t> Message;
	while (GameInstanceMessageQueue.Dequeue(Message))
	{
		// Process the message
		// For example, you can convert it to FString and print it
		// FString ReceivedMessage = FString(UTF8_TO_TCHAR(Message.data()));
		// PrintOnScreenMessage(ReceivedMessage, 5.0f, FColor::Green);
		DoMessageAction(Message);
	}
}

void UVoidEscapeGameInstance::DoMessageAction(const std::vector<uint8_t>& Message)
{

}