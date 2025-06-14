

pub enum ArmorEquipPosition
{
    HEAD = 0,
    BODY = 1,
    FOOT = 2,
    HAND = 3
}

pub enum WeaponEquipPosition
{
    MAIN = 0,
    SUB1 = 1,
    SUB2 = 2
}

pub struct Armor
{

}

pub struct Weapon
{

}

#[derive(Debug, Clone)]
pub struct PlayerEquipment
{

}

impl PlayerEquipment
{
    pub fn new() -> Self {
        return PlayerEquipment{}
    }
}