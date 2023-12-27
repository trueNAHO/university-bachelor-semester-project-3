use tap::Pipe;
use thiserror::Error;

use std::str::FromStr;

#[derive(Error, Debug)]
pub enum ActionError {
    #[error("Invalid action")]
    InvalidActionError,
}

pub enum Action {
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveUp,
}

impl FromStr for Action {
    type Err = ActionError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "a" | "h" | "left" => Action::MoveLeft.pipe(Ok),
            "j" | "s" | "down" => Action::MoveDown.pipe(Ok),
            "k" | "w" | "up" => Action::MoveUp.pipe(Ok),
            "d" | "l" | "right" => Action::MoveRight.pipe(Ok),

            _ => ActionError::InvalidActionError.pipe(Err),
        }
    }
}
