#![allow(dead_code)]

trait InputComponent {
    fn update(&self, velocity: &mut i32, direction: Directions);
}

const WALK_ACCELERATION: i32 = 1;

struct PlayerInputComponent {}

impl PlayerInputComponent {
    fn new() -> Self {
        Self {}
    }
}

enum Directions {
    Left,
    Right,
}

impl InputComponent for PlayerInputComponent {
    fn update(&self, velocity: &mut i32, direction: Directions) {
        match direction {
            Directions::Left => {
                *velocity -= WALK_ACCELERATION;
            }
            Directions::Right => {
                *velocity += WALK_ACCELERATION;
            }
        }
    }
}

struct DemoInputComponent {}

impl DemoInputComponent {
    fn new() -> Self {
        Self {}
    }
}

impl InputComponent for DemoInputComponent {
    fn update(&self, velocity: &mut i32, direction: Directions) {
        match direction {
            Directions::Left => {
                *velocity -= WALK_ACCELERATION * 5;
            }
            Directions::Right => {
                *velocity += WALK_ACCELERATION * 5;
            }
        }
    }
}

trait PhysicsComponent {
    fn update(&self, velocity: i32, x: &mut i32);
}

struct BjornPhysicsComponent {}

impl BjornPhysicsComponent {
    fn new() -> Self {
        Self {}
    }
}

impl PhysicsComponent for BjornPhysicsComponent {
    fn update(&self, velocity: i32, x: &mut i32) {
        *x += velocity;
    }
}

struct GameObject {
    velocity: i32,
    x: i32,
    input_: Box<dyn InputComponent>,
    physics_: Box<dyn PhysicsComponent>,
}

impl GameObject {
    fn new(input: Box<dyn InputComponent>, physics: Box<dyn PhysicsComponent>) -> Self {
        Self {
            velocity: 5,
            x: 2,
            input_: input,
            physics_: physics,
        }
    }

    fn update(&mut self, directions: Directions) {
        self.input_.update(&mut self.velocity, directions);
        self.physics_.update(self.velocity, &mut self.x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component() {
        let mut player_1 = GameObject::new(
            Box::new(PlayerInputComponent::new()),
            Box::new(BjornPhysicsComponent::new()),
        );
        player_1.update(Directions::Left);
        assert_eq!(player_1.x, 6);
        assert_eq!(player_1.velocity, 4);

        let mut player_2 = GameObject::new(
            Box::new(DemoInputComponent::new()),
            Box::new(BjornPhysicsComponent::new()),
        );
        player_2.update(Directions::Right);
        assert_eq!(player_2.x, 12);
        assert_eq!(player_2.velocity, 10);
    }
}
