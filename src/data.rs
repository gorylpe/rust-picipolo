use std::ops::Range;
use rand::{
    distributions::{Distribution, Standard},
    Rng
};

#[derive(Debug)]
pub enum Input {
    ChangeGameMode(GameMode),
    SetWinner(Winner),
    Choose(i32),
    Guess(i32),
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum GameMode {
    Small,
    Medium,
    Large,
}

impl Distribution<GameMode> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GameMode {
        match rng.gen_range(0..3) {
            0 => GameMode::Small,
            1 => GameMode::Medium,
            _ => GameMode::Large
        }
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Winner {
    Player,
    Computer,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Config {
    pub(crate) range: Range<i32>
}

#[derive(Debug)]
#[derive(Clone)]
pub struct State {
    pub config: Config,
    pub mode: Option<GameMode>,
    pub choose: Option<i32>,
    pub guess: Option<i32>,
    pub winner: Option<Winner>,
}
