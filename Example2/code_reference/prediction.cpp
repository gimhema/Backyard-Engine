// MyCharacter.cpp
void AMyCharacter::MoveForward(float Value)
{
    if (Controller && Value != 0.0f)
    {
        const FRotator Rotation = Controller->GetControlRotation();
        const FVector Direction = FRotationMatrix(Rotation).GetUnitAxis(EAxis::X);
        
        AddMovementInput(Direction, Value);

        if (HasAuthority() == false)
        {
            // 클라이언트 입력을 서버에 전송
            Server_Move(Direction * Value);
        }
    }
}

void AMyCharacter::Server_Move_Implementation(FVector MoveDelta)
{
    AddMovementInput(MoveDelta.GetSafeNormal(), MoveDelta.Size());
}

bool AMyCharacter::Server_Move_Validate(FVector MoveDelta)
{
    return true; // TODO: 추후 유효성 검사 구현
}
