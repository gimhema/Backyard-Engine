use std::collections::HashMap;

use crate::GameLogic::game_player::{VECharacterManager, VECharcater};


#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum ArmorEquipPosition
{
    HEAD = 0,
    BODY = 1,
    FOOT = 2,
    HAND = 3
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum WeaponEquipPosition
{
    MAIN = 0,
    SUB1 = 1,
    SUB2 = 2
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Armor
{
    armor_unique : i64,
    armor_position : ArmorEquipPosition
}

impl Armor {
    pub fn new() -> Self {
        return Armor {  }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Weapon
{
    weapon_unique : i64,
    weapon_position : WeaponEquipPosition
}

impl Weapon {
    pub fn new() -> Self {
        return Weapon {  }
    }

    pub fn create_empty_weapon_at_position(position : WeaponEquipPosition) -> Self {

    }
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

    pub fn init(&mut self) {
        self.armor_sockets.insert(ArmorEquipPosition::HEAD, Armor::new());
        self.armor_sockets.insert(ArmorEquipPosition::BODY, Armor::new());
        self.armor_sockets.insert(ArmorEquipPosition::HAND, Armor::new());
        self.armor_sockets.insert(ArmorEquipPosition::FOOT, Armor::new());
        self.weapon_sockets.insert(WeaponEquipPosition::MAIN, Weapon::new());
        self.weapon_sockets.insert(WeaponEquipPosition::SUB1, Weapon::new());
        self.weapon_sockets.insert(WeaponEquipPosition::SUB2, Weapon::new());
    }

}

impl VECharcater {

}

impl VECharacterManager {

}