use std::ops::Mul;
use crate::vec3::Vec3;
use super::F;

pub enum Axis {
    X,
    Y,
    Z,
}

pub enum Order {
    XYZ,
    YXZ,
    XZY,
    YZX,
    ZXY,
    ZYX,
}

impl Order {
    pub fn order_arr(&self) -> &[Axis] {
        match self {
            Self::XYZ => &[Axis::X, Axis::Y, Axis::Z],
            Self::YXZ => &[Axis::Y, Axis::X, Axis::Z],
            Self::XZY => &[Axis::X, Axis::Z, Axis::Y],
            Self::YZX => &[Axis::Y, Axis::Z, Axis::X],
            Self::ZXY => &[Axis::Z, Axis::X, Axis::Y],
            Self::ZYX => &[Axis::Z, Axis::Y, Axis::X],
        }
    }
}


pub struct RotMtx([[f32; 3]; 3]);

impl RotMtx {
    pub fn new() -> Self {
        Self([[0.0; 3]; 3])
    }

    pub fn x_axis(theta: F) -> Self {
        let t = theta;
        Self([
            [1.0, 0.0, 0.0],
            [0.0, t.cos(), -t.sin()],
            [0.0, t.sin(), t.cos()],
        ])
    }

    pub fn y_axis(theta: F) -> Self {
        let t = theta;
        Self([
            [t.cos(), 0.0, t.sin()],
            [0.0, 1.0, 0.0],
            [-t.sin(), 0.0, t.cos()],
        ])
    }

    pub fn z_axis(theta: F) -> Self {
        let t = theta;
        Self([
            [t.cos(), -t.sin(), 0.0],
            [t.sin(), t.cos(), 0.0],
            [0.0, 0.0, 1.0],
        ])
    }
}

impl Mul<Vec3> for RotMtx {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let m = self;
        let x = rhs.x;
        let y = rhs.y;
        let z = rhs.z;
        Vec3::new(m[0][0] * x + m[0][1] * y + m[0][2] * z,
                  m[1][0] * x + m[1][1] * y + m[1][2] * z,
                  m[2][0] * x + m[2][1] * y + m[2][2] * z)
    }
}