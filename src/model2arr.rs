#![allow(non_snake_case)]

use std::ops::Range;
use crate::model::Model;
use super::float;
use crate::vec3::Vec3;

pub struct Blocks(pub Vec<String>);

pub type Block = usize;

type Vec3Ref<'a> = &'a Vec3;

#[allow(non_snake_case)]
pub struct TriangularPlane<'a> {
    pub A: Vec3Ref<'a>,
    pub B: Vec3Ref<'a>,
    pub C: Vec3Ref<'a>,

    pub N: Vec3,
    pub k: float,
}

impl<'a> TriangularPlane<'a> {
    pub fn new(A: Vec3Ref<'a>, B: Vec3Ref<'a>, C: Vec3Ref<'a>, N: Vec3, k: float) -> Self {
        Self { A, B, C, N, k }
    }

    pub fn from_plane(A: Vec3Ref<'a>, B: Vec3Ref<'a>, C: Vec3Ref<'a>) -> Self {
        let v1 = A.clone() - B.clone();
        let v2 = A.clone() - C.clone();
        let n = Vec3::cross(v1, v2);
        let k = (n.x * A.x + n.y * A.y + n.z * A.z);
        Self::new(A, B, C, n, k)
    }

    /// Only considers x and y component of vector!!
    pub fn is_within(&self, P: Vec3) -> bool {
        let s0 = self.C.y - self.A.y;
        let s1 = P.y - self.A.y;
        let s2 = self.C.x - self.A.x;
        let s3 = self.B.y - self.A.y;

        let mut w1 = (self.A.x * s0 + s1 * s2 - P.x * s0) / (s3 * s2 - (self.B.x - self.A.x) * s0);
        let mut w2 = (s1 - w1 * s3) / s0;

        if w1.is_nan() {
            w1 = 0.0;
        }
        if w2.is_nan() {
            w2 = 0.0;
        }

        w1 >= 0.0 && w2 >= 0.0 && (w1 + w2) <= 1.0
    }

    pub fn fills_z(&self) -> bool {
        self.N.z == 0.0
    }

    pub fn calculate_z(&self, P: Vec3) -> float {
        (1.0 / self.N.z) * (self.k - self.N.x * P.x - self.N.y * P.y)
    }

    pub fn bounds(&self) -> (Vec3, Vec3) {
        (
            Vec3::new(
                self.A.x.min(self.B.x.min(self.C.x)),
                self.A.y.min(self.B.y.min(self.C.y)),
                self.A.z.min(self.B.z.min(self.C.z)),
            ),
            Vec3::new(
                self.A.x.max(self.B.x.max(self.C.x)),
                self.A.y.max(self.B.y.max(self.C.y)),
                self.A.z.max(self.B.z.max(self.C.z)),
            )
        )
    }
}

fn bounds_to_range(bounds: (Vec3, Vec3)) -> (Range<usize>, Range<usize>, Range<usize>) {
    (
        bounds.0.x as usize..bounds.1.x as usize + 1,
        bounds.0.y as usize..bounds.1.y as usize + 1,
        bounds.0.z as usize..bounds.1.z as usize + 1,
    )
}

pub fn model_2_arr(mut model: Model, dims: (usize, usize, usize)) -> Vec<Vec<Vec<bool>>> {
    let vertices = model.vertices.0;
    let mut blocks = Vec::with_capacity(dims.0);

    // Populate blocks
    for _x in 0..dims.0 {
        let mut y_arr = Vec::with_capacity(dims.1);
        for _y in 0..dims.1 {
            let mut z_arr = Vec::with_capacity(dims.2);
            for _z in 0..dims.2 {
                z_arr.push(false);
            }
            y_arr.push(z_arr);
        }
        blocks.push(y_arr);
    }

    for face in &model.faces.0 {
        if face.len() < 3 {
            panic!("Invalid face!");
        }
        if face.len() > 3 {
            unimplemented!("Cannot handle polygons yet!");
        }
        let plane = TriangularPlane::from_plane(
            &vertices[face[0]],
            &vertices[face[1]],
            &vertices[face[2]],
        );
        let bounds = plane.bounds();
        let (x_range, y_range, _z_range) = bounds_to_range(bounds);
        let fills_z = plane.fills_z();
        for x in x_range.clone() {
            for y in y_range.clone() {
                let p = Vec3::new(x as float, y as float, 0.0);
                if plane.is_within(p.clone()) {
                    if fills_z {
                        for z in 0..dims.2 {
                            blocks[x][y][z] = true;
                        }
                    } else if !fills_z {
                        let z = plane.calculate_z(p);
                        let z = z.round() as usize;
                        blocks[x][y][z] = true;
                    }
                }
            }
        }
    };

    blocks
}