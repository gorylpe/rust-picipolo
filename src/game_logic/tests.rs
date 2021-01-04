use crate::game_logic::{GameLogicImpl, GameLogic};
use crate::data::{Config, Input, GameMode};
use std::ops::Range;

#[test]
fn game_logic_after_change_game_mode_should_change_it() {
    let game_mode = GameMode::Small;
    let mut game_logic = GameLogicImpl::new(Config {
        range: Range {start: 0, end: 100}
    });
    let initial_state = game_logic.initial_state();
    let change_mode_input = Input::ChangeGameMode(game_mode.clone());
    let after_state = game_logic.apply_input(&change_mode_input, &initial_state);
    assert!(after_state.mode.is_some());
    match after_state.mode {
        Some(GameMode::Small) => assert!(true),
        _ => assert!(false)
    }
}