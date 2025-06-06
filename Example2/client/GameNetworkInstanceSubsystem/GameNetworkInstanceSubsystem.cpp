// Fill out your copyright notice in the Description page of Project Settings.
#include "GameNetworkInstanceSubsystem.h"
#include "../Unreal_Messages/QSM_BaseMessage.hpp"

void UGameNetworkInstanceSubsystem::RegisterHandler(EServerMessageType MessageType, FMessageHandler Handler)
{
    HandlerMap.Add(MessageType, Handler);
}

void UGameNetworkInstanceSubsystem::DispatchMessage(EServerMessageType MessageType, const TArray<uint8>& Payload)
{
    if (FMessageHandler* Handler = HandlerMap.Find(MessageType))
    {
        Handler->ExecuteIfBound(Payload);
    }
    else
    {
        // UE_LOG(LogTemp, Warning, TEXT("No handler for message type: %s"), *MessageType.ToString());
    }
}

void UGameNetworkInstanceSubsystem::InitFunctionHandler()
{
    this->RegisterHandler(
        EServerMessageType::PLAYER_MOVEMENT,
        FMessageHandler::CreateRaw(Handler, &MyHandler::HandleMove)
    );
}

void FMyHandler::ChatMessage(const TArray<uint8>& Payload)
{

}

void FMyHandler::CreateNewPlayer(const TArray<uint8>& Payload)
{

}

void FMyHandler::HandleMove(const TArray<uint8>& Payload)
{

}
