#include "MyAICharacter.h"
#include "GameFramework/CharacterMovementComponent.h"
#include "GameFramework/Controller.h"

void AMyAICharacter::Tick(float DeltaTime)
{
    Super::Tick(DeltaTime);

    if (bUseInterpolation && GetCharacterMovement())
    {
        // 현재 위치 → 보간된 위치 계산
        FVector InterpolatedLocation = FMath::VInterpTo(GetActorLocation(), TargetLocation, DeltaTime, InterpSpeed);

        // 속도 기반 예측 이동 추가
        FVector PredictedLocation = InterpolatedLocation + (ServerVelocity * DeltaTime);

        // 이동 벡터 계산
        FVector MoveDelta = PredictedLocation - GetActorLocation();

        if (!MoveDelta.IsNearlyZero())
        {
            FHitResult Hit;
            // 충돌을 고려한 자연스러운 이동
            GetCharacterMovement()->SafeMoveUpdatedComponent(
                MoveDelta,
                GetActorRotation(), // 회전은 유지
                true,               // 슬라이딩 등 충돌 처리 허용
                Hit
            );

            // 필요시 충돌 반응 처리
            if (Hit.IsValidBlockingHit())
            {
                // 예: 충돌 후 반응 처리 (optional)
            }
        }
    }
}

void AMyAICharacter::UpdatePositionFromServer(FVector NewPosition, FVector NewVelocity)
{
    TargetLocation = NewPosition;
    ServerVelocity = NewVelocity;
}
