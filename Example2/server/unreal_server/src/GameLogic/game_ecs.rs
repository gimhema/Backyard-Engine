
use super::game_geometry::*;
use super::game_player::*;




use std::collections::{HashMap, HashSet};

type EntityId = u32;



// ==== ECS World ====

pub struct World {
    next_entity_id: EntityId,
    entities: HashSet<EntityId>,
    transforms: HashMap<EntityId, Transform>,
}

impl World {
    pub fn new() -> Self {
        Self {
            next_entity_id: 0,
            entities: HashSet::new(),
            transforms: HashMap::new(),
        }
    }

    /// 기본 Entity 생성 (빈 컴포넌트)
    pub fn create_entity(&mut self) -> EntityId {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.insert(id);
        id
    }

    /// Entity 생성과 동시에 Position, Velocity 등록
    pub fn create_entity_with_components(
        &mut self,
        position: Option<Transform>,
    ) -> EntityId {
        let id = self.create_entity();
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
