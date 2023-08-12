#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
use std::ops::{Add, Sub};

impl Add for Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Position;
    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x.abs_diff(rhs.x),
            y: self.y.abs_diff(rhs.y),
        }
    }
}

impl Position {
    pub fn mid(&self, rhs: Self) -> Option<Self> {
        if rhs.x.abs_diff(self.x) % 2 == 0 && rhs.x.abs_diff(self.x) % 2 == 0 {
            return Some(Position {
                x: (rhs.x + self.x) / 2,
                y: (rhs.y + self.y) / 2,
            });
        }

        return None;
    }

    pub fn is_adjacent(self, rhs: &Self, steps: usize) -> bool {
        let diff = self.sub(*rhs);
        (diff.x == steps && diff.y == 0)
            || (diff.x == 0 && diff.y == steps)
            || (diff.x == steps && diff.y == steps)
    }

    pub fn is_diag(self, rhs: &Self) -> bool {
        let diff = self.sub(*rhs);
        diff.x == diff.y
    }
}
