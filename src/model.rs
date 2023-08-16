use std::ops::Mul;
use crate::coords::{Axis, Order, RotMtx};
use crate::utils;
use crate::vec3::Vec3;
use super::F;

pub struct Points(Vec<Vec3>);

impl Points {
    pub fn from_flat_vec(v: Vec<F>) -> Points {
        let mut r_vec = Points(vec![]);
        for vtx in 0..v.len() / 3 {
            r_vec.push(Vec3::new(v[3 * vtx], v[3 * vtx + 1], v[3 * vtx + 2]));
        }
        r_vec
    }
}

pub struct Faces(Vec<Vec<usize>>);

pub struct Model {
    pub vertices: Points,
    pub faces: Faces,
}

impl Model {
    pub fn new(vertices: Points, faces: Faces) -> Self {
        Self { vertices, faces }
    }

    pub fn model_dims(&self) -> (Vec3, Vec3) {
        let mut x = utils::MinMax::new(0.0, 0.0);
        let mut y = x.clone();
        let mut z = x.clone();

        for vtx in self.vertices.0 {
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
            vtx.x += move_vec.x;
            vtx.y += move_vec.y;
            vtx.z += move_vec.z;
        }
    }

    pub fn rotate(&mut self, rot: Vec3, order: Order) {
        let order = order.order_arr();
        let x_mtx = RotMtx.x_axis(rot.x);
        let y_mtx = RotMtx.x_axis(rot.x);
        let z_mtx = RotMtx.x_axis(rot.x);

        for vtx in self.vertices.0.iter_mut() {
            for ax in order {
                match ax {
                    Axis::X => vtx.mul(x_mtx),
                    Axis::Y => vtx.mul(y_mtx),
                    Axis::Z => vtx.mul(z_mtx),
                }
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
            vtx.x *= scale.x;
            vtx.y *= scale.y;
            vtx.z *= scale.z;
        }
    }
}