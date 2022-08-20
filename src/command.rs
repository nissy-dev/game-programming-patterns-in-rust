#![allow(dead_code)]

use rand::prelude::SliceRandom;

enum GameCommand {
    JumpCommand,
    FireCommand,
    LurchCommand,
    SwapCommand,
}

struct Actor {}

impl Actor {
    fn new() -> Actor {
        Actor {}
    }

    fn jump(&self) {
        println!("Actor::jump()");
    }
}

trait Command {
    fn execute(&self, actor: &Actor);
}

impl Command for GameCommand {
    fn execute(&self, actor: &Actor) {
        match self {
            GameCommand::JumpCommand => {
                println!("Jumping");
                actor.jump();
            }
            GameCommand::FireCommand => {
                println!("Firing");
            }
            GameCommand::LurchCommand => {
                println!("Lurching");
            }
            GameCommand::SwapCommand => {
                println!("Swapping");
            }
        }
    }
}

#[allow(non_camel_case_types)]
enum Button {
    BUTTON_X,
    BUTTON_Y,
    BUTTON_A,
    BUTTON_B,
}

#[allow(non_snake_case)]
struct GameInputHandler {
    buttonX_: GameCommand,
    buttonY_: GameCommand,
    buttonA_: GameCommand,
    buttonB_: GameCommand,
}

trait InputHandler {
    fn is_press(&self, button: Button) -> bool;
    fn handle_input(&self) -> Option<&GameCommand>;
}

impl InputHandler for GameInputHandler {
    fn is_press(&self, button: Button) -> bool {
        // Note: this is dummy code
        let mut arr = vec![0, 1, 2, 3];
        let mut rng = rand::thread_rng();
        arr.shuffle(&mut rng);
        match button {
            Button::BUTTON_X => arr[0] == 0,
            Button::BUTTON_Y => arr[0] == 1,
            Button::BUTTON_A => arr[0] == 2,
            Button::BUTTON_B => arr[0] == 3,
        }
    }

    fn handle_input(&self) -> Option<&GameCommand> {
        if self.is_press(Button::BUTTON_X) {
            return Some(&self.buttonX_);
        } else if self.is_press(Button::BUTTON_Y) {
            return Some(&self.buttonY_);
        } else if self.is_press(Button::BUTTON_A) {
            return Some(&self.buttonA_);
        } else if self.is_press(Button::BUTTON_B) {
            return Some(&self.buttonB_);
        } else {
            return None;
        }
    }
}

struct Position {
    x: i32,
    y: i32,
}

trait BaseCommand: Clone {
    fn new(x: i32, y: i32) -> Self;
    fn execute(&mut self, pos: &mut Position);
    fn undo(&self, pos: &mut Position);
}

#[derive(Clone)]
struct MoveCommand {
    x_: i32,
    y_: i32,
    x: i32,
    y: i32,
}

impl BaseCommand for MoveCommand {
    fn new(x: i32, y: i32) -> Self {
        MoveCommand {
            x_: 0,
            y_: 0,
            x: x,
            y: y,
        }
    }

    fn execute(&mut self, pos: &mut Position) {
        self.x_ = pos.x.clone();
        self.y_ = pos.y.clone();
        pos.x = pos.x + self.x;
        pos.y = pos.y + self.y;
    }

    fn undo(&self, pos: &mut Position) {
        pos.x = self.x_;
        pos.y = self.y_;
    }
}

#[allow(non_snake_case)]
struct CommandManager<T: BaseCommand> {
    pos: Position,
    commands_history: Vec<T>,
}

impl<T: BaseCommand> CommandManager<T> {
    fn new() -> Self {
        CommandManager {
            pos: Position { x: 0, y: 0 },
            commands_history: Vec::new(),
        }
    }

    fn execute(&mut self, command: &mut T) {
        command.execute(&mut self.pos);
        self.commands_history.push(command.clone());
    }

    fn undo(&mut self) {
        let command = self.commands_history.pop();
        if let Some(command) = command {
            command.undo(&mut self.pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_test() {
        let handler = GameInputHandler {
            buttonX_: GameCommand::JumpCommand,
            buttonY_: GameCommand::FireCommand,
            buttonA_: GameCommand::LurchCommand,
            buttonB_: GameCommand::SwapCommand,
        };

        if let Some(command) = handler.handle_input() {
            command.execute(&Actor::new());
        }
    }

    #[test]
    fn move_test() {
        let mut manager = CommandManager::new();

        let mut command_a = MoveCommand::new(1, 1);
        let mut command_b = MoveCommand::new(0, 1);
        manager.execute(&mut command_a);
        assert_eq!(manager.pos.x, 1);
        assert_eq!(manager.pos.y, 1);
        manager.execute(&mut command_b);
        assert_eq!(manager.pos.x, 1);
        assert_eq!(manager.pos.y, 2);

        // undo command_b
        manager.undo();
        assert_eq!(manager.pos.x, 1);
        assert_eq!(manager.pos.y, 1);
    }
}
