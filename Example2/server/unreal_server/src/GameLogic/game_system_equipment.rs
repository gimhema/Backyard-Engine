use std::collections::HashMap;


#[derive(Debug, Clone)]
pub enum ArmorEquipPosition
{
    HEAD = 0,
    BODY = 1,
    FOOT = 2,
    HAND = 3
}

#[derive(Debug, Clone)]
pub enum WeaponEquipPosition
{
    MAIN = 0,
    SUB1 = 1,
    SUB2 = 2
}

#[derive(Debug, Clone)]
pub struct Armor
{

}

#[derive(Debug, Clone)]
pub struct Weapon
{

}

#[derive(Debug, Clone)]
pub struct PlayerEquipment
{
    armor_sockets : HashMap<ArmorEquipPosition, Armor>,
    weapon_sockets : HashMap<WeaponEquipPosition, Weapon>
}

impl PlayerEquipment
{
    pub fn new() -> Self {
        return PlayerEquipment{
            armor_sockets : HashMap::new(),
            weapon_sockets : HashMap::new()
        }
    }
}