use super::float;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};
use crate::coords::RotMtx;

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: float,
    pub y: float,
    pub z: float,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };


    pub fn new(x: float, y: float, z: float) -> Self {
        Self { x, y, z }
    }

    pub fn from_scalar(v: float) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
        }
    }
    pub fn length_squared(&self) -> float {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> float {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        let l = self.length();
        self / l
    }

    pub fn min_val(&self) -> float {
        self.x.min(self.y.min(self.z))
    }

    pub fn cross(v1: Self, v2: Self) -> Self {
        Self::new(
            v1.y * v2.z - v1.z * v2.y,
            v1.z * v2.x - v1.x * v2.z,
            v1.x * v2.y - v1.y * v2.x,
        )
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<&RotMtx> for Vec3 {
    fn mul_assign(&mut self, rhs: &RotMtx) {
        let m = rhs.0;
        let x = self.x;
        let y = self.y;
        let z = self.z;
        self.x = m[0][0] * x + m[0][1] * y + m[0][2] * z;
        self.y = m[1][0] * x + m[1][1] * y + m[1][2] * z;
        self.z = m[2][0] * x + m[2][1] * y + m[2][2] * z;
    }
}

impl MulAssign<&Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: &Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        Self::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}
