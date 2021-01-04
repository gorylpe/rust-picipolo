use std::io;
use crate::data::*;

pub trait View {
    fn get_input(&self) -> Option<Input>;
    fn show_state(&mut self, state: &State);
}

pub struct ConsoleViewImpl {
    stdin: io::Stdin,
}

impl ConsoleViewImpl {
    pub fn new() -> ConsoleViewImpl {
        ConsoleViewImpl {
            stdin: io::stdin(),
        }
    }
}

impl View for ConsoleViewImpl {
    fn get_input(&self) -> Option<Input> {
        let mut input_str = String::new();
        self.stdin.read_line(&mut input_str).unwrap();
        let input_str = input_str.trim();
        match input_str {
            number if input_str.parse::<i32>().is_ok() => {
                let input = Input::Choose(number.parse::<i32>().unwrap());
                println!("Generated input {:?}", input);
                Some(input)
            }
            _ => {
                None
            }
        }
    }

    fn show_state(&mut self, state: &State) {
        println!("Current {:#?} \n", state);
        if let Some(winner) = state.winner {
            match winner {
                Winner::Player => println!("YOU WIN!!!!"),
                Winner::Computer => println!("COMPUTER WIN"),
            }
        }
    }
}
