void AMyAICharacter::Tick(float DeltaTime)
{
    Super::Tick(DeltaTime);

    if (bUseInterpolation)
    {
        FVector NewLocation = FMath::VInterpTo(GetActorLocation(), TargetLocation, DeltaTime, InterpSpeed);

        // 점프 상태면 중력 적용
        if (!bOnGround)
        {
            FVector GravityEffect = FVector(0, 0, -980.0f * DeltaTime); // 중력 가속도 적용
            ServerVelocity += GravityEffect;
        }

        // 최종 위치 = 보간 위치 + 서버 속도 기반 예측
        FVector PredictedLocation = NewLocation + (ServerVelocity * DeltaTime);
        SetActorLocation(PredictedLocation);
    }
}

void AMyAICharacter::UpdatePositionFromServer(FVector NewPosition, FVector NewVelocity, bool bNewOnGround)
{
    TargetLocation = NewPosition;
    ServerVelocity = NewVelocity;
    bOnGround = bNewOnGround;

    if (bOnGround)
    {
        // 착지하면 속도 초기화
        ServerVelocity = FVector::ZeroVector;
    }
}

if (bOnGround && !PreviousOnGround) // 착지 순간 감지
{
    FVector CorrectedLocation = FMath::VInterpTo(GetActorLocation(), TargetLocation, DeltaTime, 10.0f);
    SetActorLocation(CorrectedLocation);
}
PreviousOnGround = bOnGround;
