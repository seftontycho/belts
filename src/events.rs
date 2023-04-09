use bevy::prelude::Entity;

use crate::components::{Direction, Position};

pub struct UpdateEdgesEvent {
    pub entity: Entity,
    pub new_start: Direction,
}

impl UpdateEdgesEvent {
    pub fn new(entity: Entity, new_start: Direction) -> Self {
        Self { entity, new_start }
    }
}

pub struct FindEdgesEvent {
    pub entity: Entity,
}

impl FindEdgesEvent {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

#[derive(Debug)]
pub struct ModifiedEvent {
    pub position: Position,
    pub entity: Option<Entity>,
}

impl ModifiedEvent {
    pub fn new(position: Position, entity: Option<Entity>) -> Self {
        Self { position, entity }
    }
}

pub enum InputEvent {
    Rotate(Position),
    Create(Position),
    Delete(Position),
}
