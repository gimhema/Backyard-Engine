#include "MyAICharacter.h"
#include "GameFramework/CharacterMovementComponent.h"

void AMyAICharacter::Tick(float DeltaTime)
{
    Super::Tick(DeltaTime);

    if (bUseInterpolation)
    {
        FVector NewLocation = FMath::VInterpTo(GetActorLocation(), TargetLocation, DeltaTime, InterpSpeed);
        SetActorLocation(NewLocation);
    }
}

void AMyAICharacter::UpdatePositionFromServer(FVector NewPosition)
{
    TargetLocation = NewPosition;
}



void AMyAICharacter::Tick(float DeltaTime)
{
    Super::Tick(DeltaTime);

    if (bUseInterpolation)
    {
        // 보간된 위치 계산
        FVector NewLocation = FMath::VInterpTo(GetActorLocation(), TargetLocation, DeltaTime, InterpSpeed);

        // 서버에서 받은 속도를 사용하여 보정
        FVector PredictedLocation = NewLocation + (ServerVelocity * DeltaTime);
        
        SetActorLocation(PredictedLocation);
    }
}

void AMyAICharacter::UpdatePositionFromServer(FVector NewPosition, FVector NewVelocity)
{
    TargetLocation = NewPosition;
    ServerVelocity = NewVelocity;
}


