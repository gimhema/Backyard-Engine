#pragma once

#include "CoreMinimal.h"
#include "Sockets.h"
#include "SocketSubsystem.h"
#include "IPAddress.h"
#include "HAL/Runnable.h"
#include "HAL/RunnableThread.h"

class UNREALCLIENT_API FUDPSocketWrapper : public FRunnable
{
private:
    FSocket* UdpSocket;
    TSharedPtr<FInternetAddr> RemoteAddress;
    FRunnableThread* Thread;
    bool bRunThread;
    const int32 BufferSize = 2048; // 2KB 버퍼

public:
    FUDPSocketWrapper();
    virtual ~FUDPSocketWrapper();

    void SendMessage(const FString& Message, const FString& TargetIP, int32 TargetPort);
    void StopReceiving();

private:
    virtual uint32 Run() override;
};
