use std::sync::OnceLock;


#[derive(Debug)]
pub struct GameConfig {
    pub max_players: u32,
    pub server_name: String,

}

// 전역 싱글턴
static CONFIG: OnceLock<GameConfig> = OnceLock::new();

impl GameConfig {

    pub fn parse_from_file(_path: &str) -> GameConfig {
        // 여기에 JSON, TOML, YAML 등 파서 구현 가능
        // 지금은 더미 데이터 리턴
        GameConfig {
            max_players: 64,
            server_name: "MyRustServer".to_string(),
        }
    }

    // 조건 1, 3, 4: 초기화 함수 (딱 1번만 호출되어야 함)
    pub fn init(path: &str) {
        let config = GameConfig::parse_from_file(path);
        CONFIG.set(config).expect("GameConfig already initialized");
    }

    // 조건 3, 4, 5: 락 없는 불변 참조 get
    pub fn get() -> &'static GameConfig {
        CONFIG.get().expect("GameConfig not initialized")
    }
}
