
use super::game_setting::*;

// let cfg = GameConfig::get();
// println!("Server: {}, Max Players: {}", cfg.server_name, cfg.max_players);

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

    pub fn init(&mut self) {
        // 캐릭터의 기본 상태정보를 초기화함
        // 장비로 인한 보너스는 init이후 장비 초기화 메소드로 추가해준다.
        let cfg = GameConfig::get();
        self.health_point = cfg.init_health_point;
        self.ability_point = cfg.init_ability_point;
        self.stamina = cfg.init_stamina_point;
        self.alive = ActorStatusMode::ALIVE;
    }
}


