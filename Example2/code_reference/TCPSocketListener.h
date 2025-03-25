#pragma once

#include "CoreMinimal.h"
#include "Sockets.h"
#include "SocketSubsystem.h"
#include "IPAddress.h"
#include "HAL/Runnable.h"
#include "HAL/RunnableThread.h"

class UNREALCLIENT_API FTCPSocketListener : public FRunnable
{
private:
    FSocket* ServerSocket;
    FSocket* ClientSocket;
    FRunnableThread* Thread;
    bool bRunThread;
    const int32 BufferSize = 4096; 

public:
    FTCPSocketListener();
    virtual ~FTCPSocketListener();

    bool StartTCPListener(int32 Port);
    void StopTCPListener();

private:
    virtual uint32 Run() override;
    void ReceiveData();
};
