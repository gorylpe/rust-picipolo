#[cfg(test)]
mod tests;

use crate::data::*;

pub trait GameLogic {
    fn initial_state(&self) -> State;
    fn apply_input(&mut self, input: &Input, before_state: &State) -> State;
}

pub struct GameLogicImpl {
    config: Config
}

impl GameLogicImpl {
    pub fn new(config: Config) -> GameLogicImpl {
        GameLogicImpl {
            config
        }
    }
}

impl GameLogic for GameLogicImpl {
    fn initial_state(&self) -> State {
        State {
            config: self.config.clone(),
            mode: None,
            choose: None,
            guess: None,
            winner: None,
        }
    }

    fn apply_input(&mut self, input: &Input, before_state: &State) -> State {
        let mut after_state = (*before_state).clone();
        match input {
            Input::ChangeGameMode(mode) => {
                after_state.mode = Some(*mode);
            }
            Input::Choose(choose) => {
                after_state.choose = Some(*choose);
            }
            Input::SetWinner(winner) => {
                after_state.winner = Some(*winner);
            }
            Input::Guess(guess) => {
                after_state.guess = Some(*guess);
            }
        }
        after_state
    }
}