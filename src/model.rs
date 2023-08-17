use crate::coords::{Axis, Order, RotMtx};
use crate::utils;
use crate::vec3::Vec3;
use super::float;

pub struct Points(pub Vec<Vec3>);

impl Points {
    pub fn from_flat_vec(v: Vec<float>) -> Points {
        let mut r_vec = Points(vec![]);
        for vtx in 0..v.len() / 3 {
            r_vec.0.push(Vec3::new(v[3 * vtx], v[3 * vtx + 1], v[3 * vtx + 2]));
        }
        r_vec
    }
}

pub struct Faces(pub Vec<Vec<usize>>);

impl Faces {
    pub fn from_face_arteries(indices: Vec<u32>, arteries: Vec<u32>) -> Self {
        let mut faces = Vec::with_capacity(arteries.len());
        let mut i = 0;
        for artery in arteries {
            let mut face = Vec::with_capacity(artery as usize);
            for _ in 0..artery {
                face.push(indices[i] as usize);
                i += 1;
            }
            faces.push(face);
        }
        Self(faces)
    }

    pub fn from_triangles(indices: Vec<u32>) -> Self {
        let mut faces = Vec::with_capacity(indices.len() / 3);
        for f in 0..indices.len() / 3 {
            faces.push(vec![
                indices[3 * f] as usize,
                indices[3 * f + 1] as usize,
                indices[3 * f + 2] as usize,
            ]);
        }
        Self(faces)
    }
}

pub struct Model {
    pub vertices: Points,
    pub faces: Faces,
}

impl Model {
    pub fn new(vertices: Points, faces: Faces) -> Self {
        Self { vertices, faces }
    }

    pub fn model_dims(&self) -> (Vec3, Vec3) {
        let p = &self.vertices.0[0]; // Hopefully doesn't crash :)
        let mut x = utils::MinMax::new(p.x, p.x);
        let mut y = utils::MinMax::new(p.y, p.y);
        let mut z = utils::MinMax::new(p.z, p.z);

        for vtx in &self.vertices.0 {
            x.update(vtx.x);
            y.update(vtx.y);
            z.update(vtx.z);
        }

        (
            Vec3::new(x.min, y.min, z.min),
            Vec3::new(x.max, y.max, z.max),
        )
    }

    pub fn mv(&mut self, move_vec: Vec3) {
        for vtx in self.vertices.0.iter_mut() {
            *vtx += &move_vec;
        }
    }

    pub fn rotate(&mut self, rot: Vec3, order: Order) {
        let order = order.order_arr();
        let x_mtx = RotMtx::x_axis(rot.x);
        let y_mtx = RotMtx::y_axis(rot.y);
        let z_mtx = RotMtx::z_axis(rot.z);

        for vtx in self.vertices.0.iter_mut() {
            for ax in order {
                match ax {
                    Axis::X => *vtx *= &x_mtx,
                    Axis::Y => *vtx *= &y_mtx,
                    Axis::Z => *vtx *= &z_mtx,
                };
            }
        }
    }

    pub fn scale_for_box(&mut self, box_size: Vec3) -> Vec3 {
        let dims = self.model_dims();
        let scale = dims.1 - dims.0;
        Vec3::new(
            box_size.x / scale.x,
            box_size.y / scale.y,
            box_size.z / scale.z,
        )
    }

    pub fn scale(&mut self, scale: Vec3) {
        for vtx in self.vertices.0.iter_mut() {
            *vtx *= &scale;
        }
    }
}