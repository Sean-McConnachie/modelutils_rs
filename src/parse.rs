use std::ops::Mul;

pub fn load_default<P>(p: P) -> tobj::LoadResult
where
    P: AsRef<std::path::Path> + std::fmt::Debug,
{
    tobj::load_obj(&p, &tobj::LoadOptions::default())
}

#[derive(Debug)]
pub struct Range {
    pub min: f32,
    pub max: f32,
}

impl Range {
    pub fn new(v1: f32, v2: f32) -> Self {
        if v1 < v2 {
            Self { min: v1, max: v2 }
        } else {
            Self { min: v2, max: v1 }
        }
    }

    pub fn update(&mut self, v: f32) {
        if v > self.max {
            self.max = v;
        } else if v < self.min {
            self.min = v;
        }
    }

    pub fn range(&self) -> f32 {
        self.max - self.min
    }
}

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

#[derive(Debug, Clone)]
pub struct Rotation {
    x: f32,
    y: f32,
    z: f32,
}

use crate::PI;

impl Rotation {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn new_deg(x: f32, y: f32, z: f32) -> Self {
        Self::new((x * PI) / 180.0, (y * PI) / 180.0, (z * PI) / 180.0)
    }
}

pub type Model = Vec<Vertex>;

#[derive(Clone, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn from_model(m: tobj::Mesh) -> Model {
        let mut r_vec = vec![];
        for vtx in 0..m.positions.len() / 3 {
            r_vec.push(Self {
                x: m.positions[3 * vtx],
                y: m.positions[3 * vtx + 1],
                z: m.positions[3 * vtx + 2],
            });
        }
        r_vec
    }

    pub fn mul_rot_mtx(&mut self, m: Mtx3b3) {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        self.x = m[0][0] * x + m[0][1] * y + m[0][2] * z;
        self.y = m[1][0] * x + m[1][1] * y + m[1][2] * z;
        self.z = m[2][0] * x + m[2][1] * y + m[2][2] * z;
    }
}

pub type Mtx3b3 = [[f32; 3]; 3];

pub fn x_rot_mtx(theta: f32) -> Mtx3b3 {
    let t = theta;
    [
        [1.0, 0.0, 0.0],
        [0.0, t.cos(), -t.sin()],
        [0.0, t.sin(), t.cos()],
    ]
}

pub fn y_rot_mtx(theta: f32) -> Mtx3b3 {
    let t = theta;
    [
        [t.cos(), 0.0, t.sin()],
        [0.0, 1.0, 0.0],
        [-t.sin(), 0.0, t.cos()],
    ]
}

pub fn z_rot_mtx(theta: f32) -> Mtx3b3 {
    let t = theta;
    [
        [t.cos(), -t.sin(), 0.0],
        [t.sin(), t.cos(), 0.0],
        [0.0, 0.0, 1.0],
    ]
}

pub fn rotate_model(m: &mut Model, order: Order, rot: Rotation) {
    let order = order.order_arr();
    let x_mtx = x_rot_mtx(rot.x);
    let y_mtx = y_rot_mtx(rot.y);
    let z_mtx = z_rot_mtx(rot.z);
    for vtx in m.iter_mut() {
        for axis in order {
            match axis {
                Axis::X => vtx.mul_rot_mtx(x_mtx),
                Axis::Y => vtx.mul_rot_mtx(y_mtx),
                Axis::Z => vtx.mul_rot_mtx(z_mtx),
            }
        }
    }
}

#[derive(Debug)]
pub struct ModelDims {
    pub x: Range,
    pub y: Range,
    pub z: Range,
}

impl ModelDims {
    pub fn from_model(m: &Model) -> Self {
        let mut dims = ModelDims {
            x: Range::new(0.0, 0.0),
            y: Range::new(0.0, 0.0),
            z: Range::new(0.0, 0.0),
        };
        for vtx in m {
            dims.x.update(vtx.x);
            dims.y.update(vtx.y);
            dims.z.update(vtx.z);
        }
        dims
    }

    pub fn global_scale(
        &self,
        x_max: Option<Range>,
        y_max: Option<Range>,
        z_max: Option<Range>,
    ) -> f32 {
        let mut scale = 1.0;

        match x_max {
            Some(r) => {
                let s = r.range() / self.x.range();
                if s < scale {
                    scale = s;
                }
            }
            None => (),
        }

        match y_max {
            Some(r) => {
                let s = r.range() / self.y.range();
                if s < scale {
                    scale = s;
                }
            }
            None => (),
        }

        match z_max {
            Some(r) => {
                let s = r.range() / self.z.range();
                if s < scale {
                    scale = s;
                }
            }
            None => (),
        }
        scale
    }
}

pub fn scale_model(m: &mut Model, scale: Vertex) {
    for vtx in m.iter_mut() {
        vtx.x *= scale.x;
        vtx.y *= scale.y;
        vtx.z *= scale.z;
    }
}

pub type Blocks = Vec<String>;
pub type Block = usize;

pub fn model_to_vec(m: Model, arr_size: (usize, usize)) -> Vec<Vec<Block>> {
    let mut arr: Vec<Vec<Block>> = Vec::with_capacity(arr_size.0);
    
    let dims = ModelDims::from_model(&m);

    arr
    }
