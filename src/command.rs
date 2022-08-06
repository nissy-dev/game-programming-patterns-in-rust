mod game {
    use rand::prelude::SliceRandom;

    #[allow(dead_code)]
    enum GameCommand {
        JumpCommand,
        FireCommand,
        LurchCommand,
        SwapCommand,
    }

    struct Actor {}

    #[allow(dead_code)]
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
    pub struct GameInputHandler {
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
}

mod move_unit {
    use rand::prelude::SliceRandom;

    #[allow(non_camel_case_types)]
    enum Button {
        BUTTON_UP,
        BUTTON_DOWN,
    }

    #[derive(Clone, Copy)]
    struct Unit {
        x: i32,
        y: i32,
    }

    impl Unit {
        fn new(x: i32, y: i32) -> Unit {
            Unit { x, y }
        }

        fn move_to(&self, x: i32, y: i32) {
            println!("move to ({}, {})", x, y);
        }
    }

    trait Command {
        fn new(unit: Unit, x: i32, y: i32) -> Self;
        fn execute(&mut self);
        fn undo(&self);
    }

    struct MoveUnitCommand {
        unit: Unit,
        x_: i32,
        y_: i32,
        x: i32,
        y: i32,
    }

    impl Command for MoveUnitCommand {
        fn new(unit: Unit, x: i32, y: i32) -> Self {
            MoveUnitCommand {
                unit: unit,
                x_: 0,
                y_: 0,
                x: x,
                y: y,
            }
        }

        fn execute(&mut self) {
            self.x_ = self.unit.x.clone();
            self.y_ = self.unit.y.clone();
            self.unit.move_to(self.x, self.y);
        }

        fn undo(&self) {
            self.unit.move_to(self.x_, self.y_);
        }
    }

    trait InputHandler {
        fn is_press(&self, button: Button) -> bool;
        fn handle_input(&self) -> Option<MoveUnitCommand>;
    }

    #[allow(non_snake_case)]
    struct MoveUnitInputHandler {}

    impl InputHandler for MoveUnitInputHandler {
        fn is_press(&self, button: Button) -> bool {
            // Note: this is dummy code
            let mut arr = vec![0, 1];
            let mut rng = rand::thread_rng();
            arr.shuffle(&mut rng);
            match button {
                Button::BUTTON_UP => arr[0] == 0,
                Button::BUTTON_DOWN => arr[0] == 1,
            }
        }

        fn handle_input(&self) -> Option<MoveUnitCommand> {
            let unit = Unit::new(0, 0);
            if self.is_press(Button::BUTTON_DOWN) {
                let dest_y = unit.y - 1;
                return Some(MoveUnitCommand::new(unit, unit.x.clone(), dest_y));
            } else if self.is_press(Button::BUTTON_UP) {
                let dest_y = unit.y + 1;
                return Some(MoveUnitCommand::new(unit, unit.x.clone(), dest_y));
            } else {
                return None;
            }
        }
    }

    #[test]
    fn move_unit_test() {
        let handler = MoveUnitInputHandler {};

        if let Some(mut command) = handler.handle_input() {
            command.execute();
            command.undo();
        }
    }
}
