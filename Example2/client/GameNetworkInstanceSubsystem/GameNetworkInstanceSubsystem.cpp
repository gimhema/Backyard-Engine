// Fill out your copyright notice in the Description page of Project Settings.


#include "GameNetworkInstanceSubsystem.h"
#include "../QSM/QSM_VerifyAccount.hpp"
#include "../QSM/QSM_AllowConnectGame.hpp"

void UGameNetworkInstanceSubsystem::RegisterHandler(EServerMessageType MessageType, FMessageHandler Handler)
{
    HandlerMap.Add(MessageType, Handler);
}

void UGameNetworkInstanceSubsystem::DispatchMessage(EServerMessageType MessageType, std::vector<uint8_t>& Payload)
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
		case EServerMessageType::MAKE_ACCOUNT:
		{
		}
        break;
		case EServerMessageType::VERIFY_ACCOUNT:
		{
			VerifyAccount _recvMessage = VerifyAccount::deserialize(Payload);
			FString UserId = FString::Printf(TEXT("UserId: %s"), *FString(_recvMessage.userId.c_str()));
			FString UserName = FString::Printf(TEXT("UserName: %s"), *FString(_recvMessage.userName.c_str()));
			FString Password = FString::Printf(TEXT("Password: %s"), *FString(_recvMessage.password.c_str()));
			FString ConnectInfo = FString::Printf(TEXT("Connect Info: %s"), *FString(_recvMessage.connect_info.c_str()));
			
			FString ScrrenMessage = UserId + TEXT("\n") +
				UserName + TEXT("\n") +
				Password + TEXT("\n") +
				ConnectInfo;

			PrintOnScreenMessage(ScrrenMessage, 5.0f, FColor::Green);

			// Handle the verify account message here
		}
        break;
        case EServerMessageType::ALLOW_CONNECT_GAME:
		{
			AllowConnectGame _recvMessage = AllowConnectGame::deserialize(Payload);

			FString UserId = FString::Printf(TEXT("UserId: %s"), *FString(_recvMessage.accountId.c_str()));
			FString UserName = FString::Printf(TEXT("UserName: %s"), *FString(_recvMessage.name.c_str()));
			FString ConnectInfo = FString::Printf(TEXT("Connect Info: %s"), *FString(_recvMessage.connect_info.c_str()));
			FString ScrrenMessage = UserId + TEXT("\n") +
				UserName + TEXT("\n") +
				ConnectInfo;
			PrintOnScreenMessage(ScrrenMessage, 5.0f, FColor::Green);
			// Handle the allow connect game message here
            // 접속허가를 받았으니 정식으로 접속요청을함
		}
        break;
        default:
        {
            PrintOnScreenMessage("Recved Unsupported Message", 30.0f, FColor::Red);
        }
        break;
    }
}

void UGameNetworkInstanceSubsystem::InitFunctionHandler()
{

}

void UGameNetworkInstanceSubsystem::PrintOnScreenMessage(const FString& Message, float Duration, FColor TextColor)
{
    if (GEngine)
    {
        GEngine->AddOnScreenDebugMessage(-1, Duration, TextColor, Message);
    }
}

