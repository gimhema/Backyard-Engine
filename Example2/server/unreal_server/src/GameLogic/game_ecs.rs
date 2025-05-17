
use super::game_geometry::*;
use super::game_player::*;




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
    world_type : WorldType,
    world_id : WorldId,
    entities: HashSet<EntityId>,
    transforms: HashMap<EntityId, Transform>,
}

impl World {
    pub fn new() -> Self {
        Self {
            world_type : WorldType::Default,
            world_id : 0,
            entities: HashSet::new(),
            transforms: HashMap::new(),
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
        _new_id
    }

    /// Entity 생성과 동시에 Position, Velocity 등록
    pub fn create_entity_with_components(
        &mut self,
        position: Option<Transform>,
        _new_id : EntityId
    ) -> EntityId {
        let id = self.create_entity(_new_id);
        if let Some(pos) = position {
            self.transforms.insert(id, pos);
        }
        id
    }

    pub fn delete_entity(&mut self, entity: EntityId) {
        self.entities.remove(&entity);
        self.transforms.remove(&entity);
        // 향후 다른 컴포넌트들도 여기에 추가
    }

    pub fn add_position(&mut self, entity: EntityId, pos: Transform) {
        self.transforms.insert(entity, pos);
    }


    pub fn get_position(&self, entity: EntityId) -> Option<&Transform> {
        self.transforms.get(&entity)
    }

}
