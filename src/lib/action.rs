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
            "h" | "left" => Action::MoveLeft.pipe(Ok),
            "j" | "down" => Action::MoveDown.pipe(Ok),
            "k" | "up" => Action::MoveUp.pipe(Ok),
            "l" | "right" => Action::MoveRight.pipe(Ok),

            _ => ActionError::InvalidActionError.pipe(Err),
        }
    }
}
