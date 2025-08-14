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

void UVoidEscapeGameInstance::CheckGameInstance()
{
	PrintOnScreenMessage("Game Instance Valid", 10.0, FColor::Blue);
}

