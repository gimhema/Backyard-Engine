

pub enum ActorStatusMode 
{
    IDLE = 0,
    ALIVE = 1,
    DEATH = 2
}

pub struct ActorStatus
{
    alive : ActorStatusMode,
    health_point : i64,
    ability_point : i64,
    stamina : i64
}

impl ActorStatus {
    pub fn new_zero() -> Self {
        return ActorStatus { 
            alive : ActorStatusMode::IDLE,
            health_point : 0,
            ability_point : 0,
            stamina : 0
         }
    }
}


