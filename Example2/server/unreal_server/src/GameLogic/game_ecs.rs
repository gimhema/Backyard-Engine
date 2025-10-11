
use super::game_geometry::*;
use super::game_player::*;
use super::game_system_status::*;




use std::collections::{HashMap, HashSet};

type EntityId = u32;
type WorldId = i64;

pub enum WorldType
{
    Default,
    Lobby,
    MainWorld,
}


// ==== ECS World ====

pub struct World {
    pub world_type : WorldType,
    pub world_id : WorldId,
    pub entities: HashSet<EntityId>,
    pub transforms: HashMap<EntityId, Transform>,
    pub statuses: HashMap<EntityId, ActorStatus>,
}

impl World {
    pub fn new() -> Self {
        Self {
            world_type : WorldType::Default,
            world_id : 0,
            entities: HashSet::new(),
            transforms: HashMap::new(),
            statuses: HashMap::new(),
        }
    }

    pub fn init_world_info(&mut self, _world_id : WorldId, 
        _world_type : WorldType) {
            self.world_id = _world_id;
            self.world_type = _world_type;
    }

    /// 기본 Entity 생성 (빈 컴포넌트)
    pub fn create_entity(&mut self, _new_id : EntityId) -> EntityId {
        self.entities.insert(_new_id);

        // Init Transform Component
        let mut _init_position = Transform::new(
            Position::new_zero(),
            Rotation::new_zero()
        );
        self.transforms.insert(_new_id, _init_position);

        // Init Status Component
        let mut _init_status = ActorStatus::new_zero();
        _init_status.init();
        self.statuses.insert(_new_id, _init_status);

        _new_id
    }



    pub fn update_movement(&mut self, entity: EntityId,  update_mov : Transform) {
        if let Some(transform) = self.transforms.get_mut(&entity) {
            transform.set_position(update_mov.position);
            transform.set_rotation(update_mov.rotation);
        }
        // self.transforms.get_mut(&entity).unwrap().set_position(target);
        // self.transforms.get_mut(&entity).unwrap().set_rotation(target);
    }

    pub fn delete_entity(&mut self, entity: EntityId) {
        self.entities.remove(&entity);
        self.transforms.remove(&entity);
        // 향후 다른 컴포넌트들도 여기에 추가
    }

    pub fn get_position(&self, entity: EntityId) -> Option<&Transform> {
        self.transforms.get(&entity)
    }

}
