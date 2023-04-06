use bevy::prelude::*;

const TILE_SIZE: f32 = 64.0;

#[derive(Component, PartialEq, Debug, Clone)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_world_position(world_position: Vec2) -> Self {
        Self {
            x: (world_position.x / TILE_SIZE).round() as i32,
            y: (world_position.y / TILE_SIZE).round() as i32,
        }
    }
}

#[derive(Component, Deref, DerefMut, Debug)]
pub struct AnimationTimer(pub Timer);

#[derive(Clone, Debug)]
pub enum SpriteCardinal {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Clone, Debug)]
pub enum SpriteDirection {
    Straight(SpriteCardinal),
    // Cardinal here is the final direction
    Clockwise(SpriteCardinal),
    // Cardinal here is the final direction
    Anticlockwise(SpriteCardinal),
}

#[derive(Component, Clone, Debug)]
pub struct SpriteSelector {
    indecies: [Option<(usize, usize)>; 12],
}

impl SpriteSelector {
    pub fn from_indecies(indecies: [Option<(usize, usize)>; 12]) -> Self {
        Self { indecies }
    }

    pub fn get_indecies(&self, direction: SpriteDirection) -> Option<(usize, usize)> {
        match direction {
            SpriteDirection::Straight(cardinal) => self.indecies[cardinal as usize],
            SpriteDirection::Clockwise(cardinal) => self.indecies[cardinal as usize + 4],
            SpriteDirection::Anticlockwise(cardinal) => self.indecies[cardinal as usize + 8],
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
    current: usize,
}

impl AnimationIndices {
    pub fn new(first: usize, last: usize) -> Self {
        Self {
            first,
            last,
            current: first,
        }
    }

    pub fn first(&self) -> usize {
        self.first
    }

    pub fn update(&mut self, (first, last): (usize, usize)) {
        let offset = self.current - self.first;
        self.first = first;
        self.last = last;
        self.current = first + offset;
    }

    pub fn next(&mut self) -> usize {
        if self.current == self.last {
            self.current = self.first
        } else {
            self.current += 1
        }

        self.current
    }
}
