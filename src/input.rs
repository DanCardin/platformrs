use coffee::input::{keyboard, KeyboardAndMouse};
use nalgebra::Vector2;

pub enum MoveDirection {
    Left,
    Right,
}

pub struct PlayerInput {
    move_direction: Option<MoveDirection>,
    crouched: bool,

    jump: bool,
    jumping: bool,
    jump_released: bool,
}

impl PlayerInput {
    pub fn new() -> Self {
        Self {
            move_direction: None,
            crouched: false,

            jump: false,
            jumping: false,
            jump_released: true,
        }
    }

    fn update(&mut self, input: &mut KeyboardAndMouse) {
        let mut move_direction = None;
        if input.is_key_pressed(keyboard::KeyCode::A) {
            move_direction = Some(MoveDirection::Left);
        }
        if input.is_key_pressed(keyboard::KeyCode::D) {
            move_direction = Some(MoveDirection::Right);
        }

        let crouched = input.is_key_pressed(keyboard::KeyCode::S);
        let jump_pressed = input.is_key_pressed(keyboard::KeyCode::W);

        let mut jump_released = false;
        if !jump_pressed {
            jump_released = true;
        }

        let mut jump = false;
        if !self.jumping {
            self.jumping = true;
            jump = input.is_key_pressed(keyboard::KeyCode::W);
        }

        self.jump_released = jump_released;
        self.jump = jump;

        self.crouched = crouched;
        self.move_direction = move_direction;
    }

    pub fn reset_jump(&mut self) {
        self.jumping = false;
    }

    fn get_force(&self) -> Vector2<f32> {
        let mut result = Vector2::new(0.0, 0.0);

        match self.move_direction {
            Some(MoveDirection::Left) => {
                result.x = -5.0;
            }
            Some(MoveDirection::Right) => {
                result.x = 5.0;
            }
            _ => {}
        }

        if self.jump {
            result.y = -30.0;
        }
        result
    }
}

pub enum Input {
    Player(PlayerInput),
}

impl Input {
    pub fn update(&mut self, input: &mut KeyboardAndMouse) {
        match self {
            Input::Player(player_input) => player_input.update(input),
        }
    }

    pub fn get_force(&self) -> Vector2<f32> {
        match self {
            Input::Player(player_input) => player_input.get_force(),
        }
    }
}
