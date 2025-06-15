use std::collections::HashMap;

use crate::GameLogic::game_player::{VECharacterManager, VECharcater};


#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum ArmorEquipPosition
{
    DEFAULT = 0,
    HEAD = 1,
    BODY = 2,
    FOOT = 3,
    HAND = 4
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum WeaponEquipPosition
{
    DEFAULT = 0,
    MAIN = 1,
    SUB1 = 2,
    SUB2 = 3
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Armor
{
    armor_unique : i64,
    armor_position : ArmorEquipPosition
}

impl Armor {
    pub fn new() -> Self {
        return Armor { armor_unique : 0, armor_position : ArmorEquipPosition::DEFAULT }
    }

    pub fn create_empty_armomr_at_position(position : ArmorEquipPosition) -> Self {
        return Armor { armor_unique: 0, armor_position: position }
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
        return Weapon { weapon_unique : 0, weapon_position : WeaponEquipPosition::DEFAULT }
    }

    pub fn create_empty_weapon_at_position(position : WeaponEquipPosition) -> Self {
        return Weapon { weapon_unique: 0, weapon_position: WeaponEquipPosition::DEFAULT }
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
        self.armor_sockets.insert(ArmorEquipPosition::HEAD,
             Armor::create_empty_armomr_at_position(ArmorEquipPosition::HEAD));
        
        self.armor_sockets.insert(ArmorEquipPosition::BODY,
             Armor::create_empty_armomr_at_position(ArmorEquipPosition::BODY));
        
        self.armor_sockets.insert(ArmorEquipPosition::HAND,
             Armor::create_empty_armomr_at_position(ArmorEquipPosition::HAND));
        
        self.armor_sockets.insert(ArmorEquipPosition::FOOT,
             Armor::create_empty_armomr_at_position(ArmorEquipPosition::FOOT));
        
        self.weapon_sockets.insert(WeaponEquipPosition::MAIN,
             Weapon::create_empty_weapon_at_position(WeaponEquipPosition::MAIN));
        
        self.weapon_sockets.insert(WeaponEquipPosition::SUB1,
             Weapon::create_empty_weapon_at_position(WeaponEquipPosition::SUB1));
        
        self.weapon_sockets.insert(WeaponEquipPosition::SUB2,
             Weapon::create_empty_weapon_at_position(WeaponEquipPosition::SUB2));
    }

}

impl VECharcater {

}

impl VECharacterManager {

}