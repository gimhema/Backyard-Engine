use std::sync::{Arc, Mutex};
use once_cell::sync::OnceCell;

use super::game_logic_main::GameLogicMain;

static GAME_LOGIC_CELL: OnceCell<Arc<Mutex<GameLogicMain>>> = OnceCell::new();

pub fn set_global_game_logic(gl: Arc<Mutex<GameLogicMain>>) {
    // 이미 세팅되었다면 무시(또는 panic! 선택)
    let _ = GAME_LOGIC_CELL.set(gl);
}

pub fn get_game_logic() -> Option<&'static Arc<Mutex<GameLogicMain>>> {
    GAME_LOGIC_CELL.get()
}
