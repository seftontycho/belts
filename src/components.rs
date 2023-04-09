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

    pub fn up(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }

    pub fn down(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    pub fn left(&self) -> Self {
        Self::new(self.x - 1, self.y)
    }

    pub fn right(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }

    pub fn adjacent(&self) -> [Self; 4] {
        [self.up(), self.down(), self.left(), self.right()]
    }

    pub fn direction_to(&self, other: &Self) -> Option<Direction> {
        if self.up() == *other {
            Some(Direction::Up)
        } else if self.down() == *other {
            Some(Direction::Down)
        } else if self.left() == *other {
            Some(Direction::Left)
        } else if self.right() == *other {
            Some(Direction::Right)
        } else {
            None
        }
    }

    pub fn in_direction(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
    }
}

#[derive(Component, Deref, DerefMut, Debug)]
pub struct AnimationTimer(pub Timer);

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn rotate_clockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct Belt {
    pub start: Direction,
    pub end: Direction,
}

impl Belt {
    pub fn new(start: Direction, end: Direction) -> Self {
        Self { start, end }
    }

    pub fn rotate(&mut self, end: Direction) {
        self.start = end.opposite();
        self.end = end;
    }

    fn is_straight(&self) -> bool {
        self.start.opposite() == self.end
    }

    fn is_clockwise(&self) -> bool {
        match self.start {
            Direction::Up => self.end == Direction::Left,
            Direction::Down => self.end == Direction::Right,
            Direction::Left => self.end == Direction::Down,
            Direction::Right => self.end == Direction::Up,
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct SpriteSelector {
    indecies: [Option<(usize, usize)>; 12],
}

impl SpriteSelector {
    pub fn from_indecies(indecies: [Option<(usize, usize)>; 12]) -> Self {
        Self { indecies }
    }

    pub fn get_indecies(&self, belt: &Belt) -> (usize, usize) {
        let mut offset = 8;

        if belt.is_clockwise() {
            offset -= 4;
        }

        if belt.is_straight() {
            offset -= 8;
        }

        self.indecies[offset + belt.end.clone() as usize].unwrap()
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
