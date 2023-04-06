use crate::components::Position;

#[derive(Debug)]
pub struct ModifiedEvent(pub Position);

#[derive(Debug)]
pub struct RotateEvent(pub Position);

impl RotateEvent {
    pub fn new(position: Position) -> Self {
        Self(position)
    }
}
