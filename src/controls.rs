use bevy::prelude::*;
use utils::prelude::*;

const MOVE_UP: &[KeyCode] = &[KeyCode::KeyW, KeyCode::ArrowUp];
const MOVE_DOWN: &[KeyCode] = &[KeyCode::KeyS, KeyCode::ArrowDown];
const ROTATE_LEFT: &[KeyCode] = &[KeyCode::KeyA, KeyCode::ArrowLeft];
const ROTATE_RIGHT: &[KeyCode] = &[KeyCode::KeyD, KeyCode::ArrowRight];

pub struct MoveUp;

impl Pressed<KeyCode> for MoveUp {
    fn controls() -> &'static [KeyCode] {
        MOVE_UP
    }
}

pub struct MoveDown;

impl Pressed<KeyCode> for MoveDown {
    fn controls() -> &'static [KeyCode] {
        MOVE_DOWN
    }
}

pub struct RotateLeft;

impl Pressed<KeyCode> for RotateLeft {
    fn controls() -> &'static [KeyCode] {
        ROTATE_LEFT
    }
}

pub struct RotateRight;

impl Pressed<KeyCode> for RotateRight {
    fn controls() -> &'static [KeyCode] {
        ROTATE_RIGHT
    }
}
