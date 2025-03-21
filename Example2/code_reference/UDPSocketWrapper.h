
#pragma once

#include "CoreMinimal.h"
#include "Sockets.h"
#include "SocketSubsystem.h"
#include "IPAddress.h"


/**
 * 
 */
class UNREALCLIENT_API FUDPSocketWrapper
{
private:
	FSocket* UdpSocket;
	TSharedPtr<FInternetAddr> RemoteAddress;
	const int32 BufferSize = 2 * 1024; // 2KB 버퍼

public:
	FUDPSocketWrapper();
	~FUDPSocketWrapper();

public:
	void SendMessage(const FString& Message, const FString& TargetIP, int32 TargetPort);
	void ReceiveMessage();

};
