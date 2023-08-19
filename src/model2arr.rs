#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::io::Write;
use std::ops::Range;
use crate::model::Model;
use super::float;
use crate::vec3::Vec3;

#[derive(Serialize, Deserialize)]
struct ZAxis {
    b: Vec<u16>,
}

#[derive(Serialize, Deserialize)]
struct XAxis {
    b: Vec<ZAxis>,
}

#[derive(Serialize, Deserialize)]
struct JsonModel {
    b: Vec<XAxis>,
}

impl Into<JsonModel> for ArrayModel {
    fn into(self) -> JsonModel {
        let mut y_arr = Vec::with_capacity(self.blocks.len());

        for y in self.blocks.into_iter() {
            let mut x_arr = Vec::with_capacity(y.len());
            for x in y.into_iter() {
                x_arr.push(ZAxis { b: x });
            }
            y_arr.push(XAxis { b: x_arr });
        }

        JsonModel { b: y_arr }
    }
}

pub fn arr_2_json<P>(path: P, arr: ArrayModel) -> std::io::Result<()>
    where
        P: AsRef<std::path::Path> + std::fmt::Debug,
{
    let mut file = std::fs::File::create(path)?;

    let json: JsonModel = arr.into();
    let json = serde_json::to_string(&json)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

pub struct Blocks(pub Vec<String>);

pub type Block = u16;

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
        let k = n.x * A.x + n.y * A.y + n.z * A.z;
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

fn make_range(min: float, max: float, resolution: float) -> Range<usize> {
    let min = (min * resolution).round() as usize;
    let max = (max * resolution).round() as usize;
    min..max
}

fn bounds_to_range(bounds: (Vec3, Vec3), resolution: float) -> (Range<usize>, Range<usize>, Range<usize>) {
    (
        make_range(bounds.0.x, bounds.1.x, resolution),
        make_range(bounds.0.y, bounds.1.y, resolution),
        make_range(bounds.0.z, bounds.1.z, resolution),
    )
}

pub type CoordXZ = (usize, usize);
pub type CoordXYZ = (usize, usize, usize);

/// Blocks are stored as blocks[y][x][z]
#[derive(Debug)]
pub struct ArrayModel {
    pub blocks: Vec<Vec<Vec<Block>>>,
    pub dims: (usize, usize, usize),
    pub resolution: float,
}

impl ArrayModel {
    pub fn new(dims: CoordXYZ, resolution: float) -> Self {
        let mut blocks = Vec::with_capacity(dims.1);
        for _y in 0..dims.1 {
            let mut x_arr = Vec::with_capacity(dims.0);
            for _x in 0..dims.0 {
                let mut z_arr = Vec::with_capacity(dims.2);
                for _z in 0..dims.2 {
                    z_arr.push(0);
                }
                x_arr.push(z_arr);
            }
            blocks.push(x_arr);
        }
        Self { blocks, dims, resolution }
    }

    pub fn get(&self, b: CoordXYZ) -> u16 {
        self.blocks[b.1][b.0][b.2]
    }

    pub fn set(&mut self, b: CoordXYZ, val: u16) {
        self.blocks[b.1][b.0][b.2] = val;
    }
}

/// Resolution up samples the model. Increase this if there are many "holes" in the resulting array.
pub fn model_2_arr(model: Model, dims: CoordXYZ, resolution: float) -> ArrayModel {
    let vertices = model.vertices.0;

    let mut array_model = ArrayModel::new(dims, resolution);

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
        let (x_range, y_range, z_range) = bounds_to_range(bounds, resolution);
        let fills_z = plane.fills_z();
        for x in x_range.clone() {
            for y in y_range.clone() {
                let p = Vec3::new(x as float / resolution, y as float / resolution, 0.0);
                if plane.is_within(p.clone()) {
                    let x = x / resolution as usize;
                    let y = y / resolution as usize;
                    if fills_z {
                        for z in z_range.clone() {
                            let z = (z as float / resolution).round() as usize;
                            array_model.set((x, y, z), 1);
                        }
                    } else if !fills_z {
                        let z = plane.calculate_z(p);
                        let z = z.round() as usize;
                        array_model.set((x, y, z), 1);
                    }
                }
            }
        }
    }
    array_model
}