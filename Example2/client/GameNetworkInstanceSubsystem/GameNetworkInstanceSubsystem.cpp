// Fill out your copyright notice in the Description page of Project Settings.


#include "GameNetworkInstanceSubsystem.h"

void UGameNetworkInstanceSubsystem::RegisterHandler(EServerMessageType MessageType, FMessageHandler Handler)
{
    HandlerMap.Add(MessageType, Handler);
}

void UGameNetworkInstanceSubsystem::DispatchMessage(EServerMessageType MessageType, const TArray<uint8>& Payload)
{
    switch (MessageType)
    {
        case EServerMessageType::DEFAULT:
        {
    
        }
        break;
        case EServerMessageType::CHAT:
        {

        }
        break;
        case EServerMessageType::NEW_PLAYER:
        {

        }
        break;
        case EServerMessageType::PLAYER_MOVEMENT:
        {

        }
        break;
    break;
    default:
        break;
    }
}

void UGameNetworkInstanceSubsystem::InitFunctionHandler()
{

}

