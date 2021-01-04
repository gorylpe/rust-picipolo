use std::ops::Range;

mod data;
mod game;
mod game_logic;
mod view;
mod game_judge;

use crate::data::*;
use crate::game_logic::GameLogicImpl;
use crate::game::{Game, GameImpl};
use crate::view::ConsoleViewImpl;
use crate::game_judge::GameJudgeImpl;

fn main() {
    let config = Config {
        range: Range {
            start: 0,
            end: 100,
        }
    };

    let game_logic = GameLogicImpl::new(config);
    let view = ConsoleViewImpl::new();
    let game_judge = GameJudgeImpl::new();

    let mut game = GameImpl::new(game_logic, view, game_judge);

    loop {
        let result = game.tick();
        match result {
            Ok(true) => { continue; }
            Ok(false) => { break; }
            Err(()) => { panic!("Tick err") }
        }
    }

    println!("Looping finished")
}
