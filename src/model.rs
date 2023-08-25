use std::path;
use crate::coords::{Axis, Order, RotMtx};
use crate::utils;
use crate::vec2::Vec2;
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

pub struct Faces(pub Vec<[usize; 3]>);

impl Faces {
    pub fn from_triangles(indices: Vec<u32>) -> Self {
        let mut faces = Vec::with_capacity(indices.len() / 3);
        for f in 0..indices.len() / 3 {
            faces.push([
                indices[3 * f] as usize,
                indices[3 * f + 1] as usize,
                indices[3 * f + 2] as usize,
            ]);
        }
        Self(faces)
    }
}

pub struct TextureCoords(pub Vec<Vec2>);

impl TextureCoords {
    pub fn from_flat_vec(v: Vec<float>) -> Option<Self> {
        if v.len() % 2 != 0 || v.is_empty() {
            return None;
        }

        let mut r_vec = TextureCoords(vec![]);
        for vtx in 0..v.len() / 2 {
            r_vec.0.push(Vec2::new(v[2 * vtx], v[2 * vtx + 1]));
        }
        Some(r_vec)
    }
}

pub struct TextureFaces(pub Vec<[usize; 3]>);

impl TextureFaces {
    pub fn from_triangles(indices: Vec<u32>) -> Option<Self> {
        if indices.len() % 3 != 0 || indices.is_empty() {
            return None;
        }
        let mut faces = Vec::with_capacity(indices.len() / 3);
        for f in 0..indices.len() / 3 {
            faces.push([
                indices[3 * f] as usize,
                indices[3 * f + 1] as usize,
                indices[3 * f + 2] as usize,
            ]);
        }
        Some(Self(faces))
    }
}

pub struct Model {
    pub vertices: Points,
    pub faces: Faces,
    pub texture_img: Option<image::DynamicImage>,
    pub texture_coords: Option<TextureCoords>,
    pub texture_faces: Option<TextureFaces>,
}

impl Model {
    pub fn new(
        vertices: Points, faces: Faces, texture_fp: Option<path::PathBuf>,
        texture_coords: Option<TextureCoords>, texture_faces: Option<TextureFaces>,
    ) -> Self {
        let img = if let Some(fp) = texture_fp {
            Some(image::open(fp).unwrap())
        } else {
            None
        };
        Self { vertices, faces, texture_img: img, texture_coords, texture_faces }
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