// Tick 함수에서 보간 처리
void AMyCharacter::Tick(float DeltaTime)
{
    Super::Tick(DeltaTime);

    if (!HasAuthority())
    {
        // 보간 처리
        FVector NewLocation = FMath::VInterpTo(GetActorLocation(), ServerPosition, DeltaTime, InterpSpeed);
        SetActorLocation(NewLocation);
    }
}

// 위치 복제 변수 처리
void AMyCharacter::GetLifetimeReplicatedProps(TArray< FLifetimeProperty >& OutLifetimeProps) const
{
    Super::GetLifetimeReplicatedProps(OutLifetimeProps);
    DOREPLIFETIME(AMyCharacter, ServerPosition);
}

void AMyCharacter::OnRep_ServerPosition()
{
    // 서버에서 위치 수신시 → 다음 Tick에서 보간할 위치로 저장
}
