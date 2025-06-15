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
pub struct ArmorSocket
{
    armor_unique : i64,
    armor_position : ArmorEquipPosition
}

impl ArmorSocket {
    pub fn new() -> Self {
        return ArmorSocket { armor_unique : 0, armor_position : ArmorEquipPosition::DEFAULT }
    }

    pub fn create_empty_armomr_at_position(position : ArmorEquipPosition) -> Self {
        return ArmorSocket { armor_unique: 0, armor_position: position }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct WeaponSocket
{
    weapon_unique : i64,
    weapon_position : WeaponEquipPosition
}

impl WeaponSocket {
    pub fn new() -> Self {
        return WeaponSocket { weapon_unique : 0, weapon_position : WeaponEquipPosition::DEFAULT }
    }

    pub fn create_empty_weapon_at_position(position : WeaponEquipPosition) -> Self {
        return WeaponSocket { weapon_unique: 0, weapon_position: WeaponEquipPosition::DEFAULT }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerEquipment
{
    armor_sockets : HashMap<ArmorEquipPosition, ArmorSocket>,
    weapon_sockets : HashMap<WeaponEquipPosition, WeaponSocket>
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
             ArmorSocket::create_empty_armomr_at_position(ArmorEquipPosition::HEAD));
        
        self.armor_sockets.insert(ArmorEquipPosition::BODY,
             ArmorSocket::create_empty_armomr_at_position(ArmorEquipPosition::BODY));
        
        self.armor_sockets.insert(ArmorEquipPosition::HAND,
             ArmorSocket::create_empty_armomr_at_position(ArmorEquipPosition::HAND));
        
        self.armor_sockets.insert(ArmorEquipPosition::FOOT,
             ArmorSocket::create_empty_armomr_at_position(ArmorEquipPosition::FOOT));
        
        self.weapon_sockets.insert(WeaponEquipPosition::MAIN,
             WeaponSocket::create_empty_weapon_at_position(WeaponEquipPosition::MAIN));
        
        self.weapon_sockets.insert(WeaponEquipPosition::SUB1,
             WeaponSocket::create_empty_weapon_at_position(WeaponEquipPosition::SUB1));
        
        self.weapon_sockets.insert(WeaponEquipPosition::SUB2,
             WeaponSocket::create_empty_weapon_at_position(WeaponEquipPosition::SUB2));
    }

}

impl VECharcater {

}

impl VECharacterManager {

}