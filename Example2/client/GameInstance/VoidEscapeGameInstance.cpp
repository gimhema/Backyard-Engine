// Fill out your copyright notice in the Description page of Project Settings.


#include "VoidEscapeGameInstance.h"
#include "QSM/QSM_VerifyAccount.hpp"
#include "QSM/QSM_AllowConnectGame.hpp"
#include "QSM/QSM_BaseMessage.h"

UVoidEscapeGameInstance::UVoidEscapeGameInstance()
{
	// Constructor logic if needed
	// GameInstanceMessageQueue = TQueue<std::vector<uint8_t>>();
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

	// if (!udpSocketWrapper)
	// {
	// 	udpSocketWrapper = new UDPSocketWrapper();
	// }
	// else
	// {
	// 	delete udpSocketWrapper;
	// 	udpSocketWrapper = new UDPSocketWrapper();
	// }
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

	BaseMessage BaseMsg = BaseMessage::deserialize(Message);

	EServerMessageType MessageType = static_cast<EServerMessageType>(BaseMsg.id);

	switch (MessageType)
		{
		// Game Instance Actions
		case EServerMessageType::ALLOW_CONNECT_GAME: // Example case for a specific message type
			// Handle the message accordingly
			PrintOnScreenMessage("Received ALLOW_CONNECT_GAME message", 3.0f, FColor::Red);
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
	PrintOnScreenMessage("Pushing Message to Queue ! ! ! ! ! ! ! ! ! ! ! ", 3.0f, FColor::Green);
	// GameInstanceMessageQueue.Enqueue(Message);
}

void UVoidEscapeGameInstance::ProcessMessageQueue()
{
	std::vector<uint8_t> Message;
	// while (GameInstanceMessageQueue.Dequeue(Message))
	// {
	// 	// Process the message
	// 	// For example, you can convert it to FString and print it
	// 	// FString ReceivedMessage = FString(UTF8_TO_TCHAR(Message.data()));
	// 	// PrintOnScreenMessage(ReceivedMessage, 5.0f, FColor::Green);
	// 	DoMessageAction(Message);
	// 
	// 	if (GameInstanceMessageQueue.IsEmpty()) {
	// 		return;
	// 	}
	// }
}

void UVoidEscapeGameInstance::DoMessageAction(const std::vector<uint8_t>& Message)
{
	PrintOnScreenMessage("Processing Message Action", 5.0f, FColor::Emerald);
	PrintOnScreenMessage("Processing Message Action", 5.0f, FColor::Emerald);
	PrintOnScreenMessage("Processing Message Action", 5.0f, FColor::Emerald);
	PrintOnScreenMessage("Processing Message Action", 5.0f, FColor::Emerald);
	PrintOnScreenMessage("Processing Message Action", 5.0f, FColor::Emerald);
	PrintOnScreenMessage("Processing Message Action", 5.0f, FColor::Emerald);
}

void UVoidEscapeGameInstance::SendVerifyAccount()
{
	bool allowConnect = false;
	if (allowConnect == false)
	{
		allowConnect = true;

		PrintOnScreenMessage(TEXT("SetUp TCP Socket Completed. Sending VERIFY_ACCOUNT..."), 5.0f, FColor::Green);

		VerifyAccount _respConnectMsg;
		_respConnectMsg.mid = static_cast<uint32_t>(EServerMessageType::VERIFY_ACCOUNT);
		_respConnectMsg.userId = "TESTID";
		_respConnectMsg.userName = "TESTNAME";
		_respConnectMsg.password = "1234";
		_respConnectMsg.connect_info = "127.0.0.1";

		std::vector<uint8_t> _msgBuffer = _respConnectMsg.serialize();
		if (!SocketListener->SendMessageBinary(_msgBuffer))
		{
			PrintOnScreenMessage(TEXT("VERIFY_ACCOUNT Message Send Failed"), 5.0f, FColor::Red);
			SocketListener->Disconnect();
			return;
		}

		PrintOnScreenMessage(TEXT("VERIFY_ACCOUNT message sent. Starting Receive Thread..."), 5.0f, FColor::Green);
	}
}