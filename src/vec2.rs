use std::ops::{Div, Sub};
use super::float;

#[derive(Debug, Clone)]
pub struct Vec2 {
    pub x: float,
    pub y: float,
}

impl Vec2 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
    };

    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
    };

    pub fn new(x: float, y: float) -> Self {
        Self { x, y }
    }

    pub fn from_scalar(v: float) -> Self {
        Self {
            x: v,
            y: v,
        }
    }

    pub fn length_squared(&self) -> float {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> float {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        let l = self.length();
        self / l
    }

    pub fn min_val(&self) -> float {
        self.x.min(self.y)
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Self;

    fn div(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl Div<float> for Vec2 {
    type Output = Self;

    fn div(self, rhs: float) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<float> for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: float) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}