use coffee::input::{keyboard, KeyboardAndMouse};
use nalgebra::Vector2;

pub enum MoveDirection {
    Left,
    Right,
}

pub struct PlayerInput {
    move_direction: Option<MoveDirection>,
    jump: bool,
    jumping: bool,
    crouched: bool,
}

impl PlayerInput {
    pub fn new() -> Self {
        Self {
            move_direction: None,
            jump: false,
            jumping: false,
            crouched: false,
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

        let jump = input.is_key_pressed(keyboard::KeyCode::W);
        let crouched = input.is_key_pressed(keyboard::KeyCode::S);

        if !self.jumping {
            // self.jumping = true;
            self.jump = jump;
        }
        self.crouched = crouched;
        self.move_direction = move_direction;
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
            result.y = -15.0;
        }
        result
    }
}

pub enum Input {
    Player(PlayerInput),
    None,
}

impl Input {
    pub fn update(&mut self, input: &mut KeyboardAndMouse) {
        match self {
            Input::Player(player_input) => player_input.update(input),
            Input::None => return,
        }
    }
    pub fn get_force(&self) -> Vector2<f32> {
        match self {
            Input::Player(player_input) => player_input.get_force(),
            Input::None => Vector2::new(0.0, 0.0),
        }
    }
}
