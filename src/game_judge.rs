use crate::data::*;
use rand::Rng;
use std::option::Option::Some;
use crate::data::Winner::{Player, Computer};

pub trait GameJudge {
    fn on_state_received(&mut self, state: &State) -> Option<Input>;
}

pub struct GameJudgeImpl {}

impl GameJudgeImpl {
    pub fn new() -> GameJudgeImpl {
        GameJudgeImpl {}
    }

    fn choose_mode(&self) -> GameMode {
        rand::random()
    }

    fn guess(&self, state: &State) -> i32 {
        rand::thread_rng().gen_range(state.config.range.clone())
    }

    fn check_winner(&self, state: &State) -> Winner {
        let guess = &state.guess.unwrap();
        let choose = &state.choose.unwrap();
        let mode = &state.mode.unwrap();
        let range = &state.config.range;

        return match mode {
            GameMode::Small => {
                if choose < guess {
                    return Player;
                }
                Computer
            }
            GameMode::Medium => {
                let mid = (range.start + range.end) / 2;
                let choose_dist = (choose - mid).abs();
                let guess_dist = (guess - mid).abs();
                if choose_dist < guess_dist {
                    return Player;
                }
                Computer
            }
            GameMode::Large => {
                if guess < choose {
                    return Player;
                }
                Computer
            }
        }
    }
}

impl GameJudge for GameJudgeImpl {
    fn on_state_received(&mut self, state: &State) -> Option<Input> {
        if let None = &state.mode {
            return Some(Input::ChangeGameMode(self.choose_mode()))
        }
        if let (Some(_), Some(_), None) = (&state.mode, &state.choose, &state.guess) {
            return Some(Input::Guess(self.guess(state)));
        }
        if let (Some(_), Some(_), Some(_), None) = (&state.mode, &state.choose, &state.guess, &state.winner) {
            return Some(Input::SetWinner(self.check_winner(state)))
        }
        None
    }
}