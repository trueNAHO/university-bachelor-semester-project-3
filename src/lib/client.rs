use educe::Educe;

use core::fmt;

use crate::action::Action;

#[derive(Educe)]
#[educe(Default(new))]
pub struct Client {
    #[educe(Default = 1)]
    speed: i32,

    x: i32,
    y: i32,
}

impl Client {
    pub fn update(&mut self, action: Action) {
        match action {
            Action::MoveDown => self.y += self.speed,
            Action::MoveLeft => self.x -= self.speed,
            Action::MoveRight => self.x += self.speed,
            Action::MoveUp => self.y -= self.speed,
        }
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Client(x={}, y={}, speed={})",
            self.x, self.y, self.speed
        )
    }
}
