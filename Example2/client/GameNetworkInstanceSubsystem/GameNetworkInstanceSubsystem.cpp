// Fill out your copyright notice in the Description page of Project Settings.


#include "GameNetworkInstanceSubsystem.h"

void UGameNetworkInstanceSubsystem::RegisterHandler(FName MessageType, FMessageHandler Handler)
{
    HandlerMap.Add(MessageType, Handler);
}

void UGameNetworkInstanceSubsystem::DispatchMessage(FName MessageType, const TArray<uint8>& Payload)
{
    if (FMessageHandler* Handler = HandlerMap.Find(MessageType))
    {
        Handler->ExecuteIfBound(Payload);
    }
    else
    {
        UE_LOG(LogTemp, Warning, TEXT("No handler for message type: %s"), *MessageType.ToString());
    }
}
