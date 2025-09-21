use std::sync::{Arc, Mutex};
use once_cell::sync::OnceCell;

use super::game_logic_main::GameLogicMain;

// 전역 GameLogic 핸들 (서버 초기화 시 1회 set)
static GAME_LOGIC_CELL: OnceCell<Arc<Mutex<GameLogicMain>>> = OnceCell::new();

/// 서버 초기화 시 한 번만 호출
pub fn set_global_game_logic(gl: Arc<Mutex<GameLogicMain>>) {
    // 이미 세팅되었다면 무시(또는 panic! 선택)
    let _ = GAME_LOGIC_CELL.set(gl);
}

/// 콜백 등 어디서든 접근
pub fn get_game_logic() -> Option<&'static Arc<Mutex<GameLogicMain>>> {
    GAME_LOGIC_CELL.get()
}
