// https://github.com/lpxxn/rust-design-pattern/blob/master/behavioral/state.rs
#![allow(dead_code)]

use std::fmt::Debug;

enum Input {
    Up,
    Down,
}

#[derive(Debug)]
struct Heroine {
    state: Box<dyn HeroineState>,
}

impl Heroine {
    fn new() -> Self {
        Heroine {
            state: Box::new(StandingState),
        }
    }

    fn handle_input(&mut self, input: Input) {
        self.state = self.state.handle_input(input);

        // Call the enter action on the new state.
        self.state.enter(self);
    }
}

trait HeroineState: Debug {
    fn enter(&self, heroine: &Heroine);
    fn handle_input(&self, input: Input) -> Box<dyn HeroineState>;
}

#[derive(Debug)]
struct DuckingState;

impl HeroineState for DuckingState {
    fn enter(&self, heroine: &Heroine) {
        println!("{:?} Ducking!", heroine);
    }

    fn handle_input(&self, input: Input) -> Box<dyn HeroineState> {
        match input {
            Input::Up => Box::new(StandingState),
            _ => panic!("Invalid input"),
        }
    }
}

#[derive(Debug)]
struct StandingState;

impl HeroineState for StandingState {
    fn enter(&self, heroine: &Heroine) {
        println!("{:?} Standing!", heroine);
    }

    fn handle_input(&self, input: Input) -> Box<dyn HeroineState> {
        match input {
            Input::Down => Box::new(DuckingState),
            _ => panic!("Invalid input"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state() {
        let mut heroine = Heroine::new();
        // heroine is standing
        println!("heroine: {:?}", heroine.state);

        heroine.handle_input(Input::Down);
        // heroine is ducking
        println!("heroine: {:?}", heroine.state);
    }
}
