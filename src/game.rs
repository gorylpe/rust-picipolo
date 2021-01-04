use crate::data::*;
use crate::game_logic::GameLogic;
use crate::view::View;
use crate::game_judge::GameJudge;

pub trait Game {
    fn tick(&mut self) -> Result<bool, ()>;
}

pub struct GameImpl<TGameLogic, TView, TGameJudge>
    where TGameLogic: GameLogic,
          TView: View,
          TGameJudge: GameJudge {
    game_logic: TGameLogic,
    view: TView,
    game_judge: TGameJudge,

    tick: i32,
    current_state: State,
}

impl<TGameLogic, TView, TGameJudge> GameImpl<TGameLogic, TView, TGameJudge>
    where TGameLogic: GameLogic,
          TView: View,
          TGameJudge: GameJudge {
    pub fn new(game_logic: TGameLogic, view: TView, game_judge: TGameJudge) -> GameImpl<TGameLogic, TView, TGameJudge> {
        GameImpl {
            current_state: game_logic.initial_state(),
            tick: 0,

            game_logic,
            view,
            game_judge,

        }
    }
}

impl<TGameLogic, TView, TGameJudge> Game for GameImpl<TGameLogic, TView, TGameJudge>
    where TGameLogic: GameLogic,
          TView: View,
          TGameJudge: GameJudge {
    fn tick(&mut self) -> Result<bool, ()> {
        let input = self.view.get_input();
        if let Some(input) = input {
            self.current_state = self.game_logic.apply_input(&input, &self.current_state);
            self.view.show_state(&self.current_state)
        };
        loop {
            let judge_input = self.game_judge.on_state_received(&self.current_state);
            match judge_input {
                Some(judge_input) => {
                    self.current_state = self.game_logic.apply_input(&judge_input, &self.current_state);
                    self.view.show_state(&self.current_state);
                }
                None => {
                    break;
                }
            }
        }

        if let Some(_) = self.current_state.winner {
            return Ok(false);
        }

        self.tick += 1;
        Ok(true)
    }
}