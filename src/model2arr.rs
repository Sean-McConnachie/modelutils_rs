#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::io::Write;
use std::ops::Range;
use image::{GenericImageView, Pixel};
use crate::model::{Model, TextureCoords, TextureFaces};
use crate::vec2::Vec2;
use super::float;
use crate::vec3::Vec3;

#[allow(non_camel_case_types)]
pub type int = i16;
#[allow(non_camel_case_types)]
pub type uint = u16;

#[derive(Serialize, Deserialize)]
struct ZAxis {
    b: Vec<Block>,
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

pub type Block = i16;

type A2B = float;
type A2C = float;

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

    /// Here we represent some point P on the plane of A, B, C by `P = w1*(A2B) + w2*(A2C)`
    pub fn calc_weights(&self, P: &Vec2) -> (A2B, A2C) {
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

        (w1, w2)
    }

    pub fn weights_within_plane((w1, w2): (A2B, A2C)) -> bool {
        w1 >= 0.0 && w2 >= 0.0 && (w1 + w2) <= 1.0
    }

    pub fn fills_z(&self) -> bool {
        self.N.z == 0.0
    }

    pub fn calculate_z(&self, P: &Vec2) -> float {
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

pub struct TexturePlane {
    a2b: Vec2,
    a2c: Vec2,
}

impl TexturePlane {
    pub fn new(A: &Vec2, B: &Vec2, C: &Vec2) -> Self {
        Self {
            a2b: B - A,
            a2c: C - A,
        }
    }

    pub fn calc_p(&self, (w1, w2): (A2B, A2C)) -> Vec2 {
        &self.a2b * w1 + &self.a2c * w2
    }
}

fn make_range(min: float, max: float, resolution: float) -> Range<usize> {
    let min = (min * resolution).round() as usize;
    let max = (max * resolution).round() as usize + 1;
    min..max
}

fn bounds_to_range(bounds: (Vec3, Vec3), resolution: float) -> (Range<usize>, Range<usize>, Range<usize>) {
    (
        make_range(bounds.0.x, bounds.1.x, resolution),
        make_range(bounds.0.y, bounds.1.y, resolution),
        make_range(bounds.0.z, bounds.1.z, resolution),
    )
}

pub type CoordXZ = (uint, uint);
pub type CoordXYZ = (uint, uint, uint);

/// Blocks are stored as blocks[y][x][z]
#[derive(Debug)]
pub struct ArrayModel {
    pub blocks: Vec<Vec<Vec<Block>>>,
    pub dims: CoordXYZ,
    pub resolution: float,
}

impl ArrayModel {
    pub fn new(dims: CoordXYZ, resolution: float) -> Self {
        let mut y_arr = Vec::with_capacity(dims.1 as usize);
        for _y in 0..dims.1 {
            let mut x_arr = Vec::with_capacity(dims.0 as usize);
            for _x in 0..dims.0 {
                let mut z_arr = Vec::with_capacity(dims.2 as usize);
                for _z in 0..dims.2 {
                    z_arr.push(0);
                }
                x_arr.push(z_arr);
            }
            y_arr.push(x_arr);
        }
        Self { blocks: y_arr, dims, resolution }
    }

    /// c_xyz: (x, y, z)
    pub fn get(&self, c_xyz: (usize, usize, usize)) -> Block {
        self.blocks[c_xyz.1][c_xyz.0][c_xyz.2]
    }

    /// c_xyz: (x, y, z)
    pub fn set(&mut self, c_xyz: (usize, usize, usize), val: Block) {
        self.blocks[c_xyz.1][c_xyz.0][c_xyz.2] = val;
    }
}

const DEFAULT_TEXTURE_ID: Block = -1;

fn optional_texture_plane(
    texture_faces: &Option<TextureFaces>,
    texture_coords: &Option<TextureCoords>,
) -> Option<TexturePlane> {
    if texture_faces.is_none() {
        return None;
    }

    let texture_faces = texture_faces.as_ref().unwrap();
    let texture_coords = texture_coords.as_ref().unwrap();

    Some(TexturePlane::new(
        &texture_coords.0[texture_faces.0[0][0]],
        &texture_coords.0[texture_faces.0[0][1]],
        &texture_coords.0[texture_faces.0[0][2]],
    ))
}

type SumPixel = usize;
type SumPixelSquared = usize;
type PixelCount = usize;

type RGB = ((SumPixel, SumPixelSquared), (SumPixel, SumPixelSquared), (SumPixel, SumPixelSquared));

fn precompute_imgs(
    available_textures: &Vec<(Block, image::DynamicImage)>,
) -> Vec<(Block, RGB, PixelCount)> {
    let mut precomputed_imgs = Vec::with_capacity(available_textures.len());
    for (block, img) in available_textures {
        let mut sum_pixels: RGB = ((0, 0), (0, 0), (0, 0));
        for (_, _, col) in img.pixels() {
            let r = col[0] as usize;
            let g = col[1] as usize;
            let b = col[2] as usize;
            sum_pixels.0.0 += r;
            sum_pixels.0.1 += r * r;
            sum_pixels.1.0 += g;
            sum_pixels.1.1 += g * g;
            sum_pixels.2.0 += b;
            sum_pixels.2.1 += b * b;
        }
        precomputed_imgs.push((*block, sum_pixels, img.pixels().count()));
    }
    precomputed_imgs
}

pub fn get_texture_id(
    (w1, w2): (A2B, A2C),
    texture_plane: &Option<TexturePlane>,
    texture_image: &Option<image::DynamicImage>,
    available_textures: &Vec<(Block, RGB, PixelCount)>,
) -> Block {
    if texture_plane.is_none() {
        return DEFAULT_TEXTURE_ID;
    }

    let texture_plane = texture_plane.as_ref().unwrap();
    let texture_image = texture_image.as_ref().unwrap();

    let point = texture_plane.calc_p((w1, w2));

    if point.x < 0.0 || point.y < 0.0 || point.x > texture_image.width() as float || point.y > texture_image.height() as float {
        // panic!("o o f");
        return DEFAULT_TEXTURE_ID;
    }

    let pixel = texture_image.get_pixel(point.x as u32, point.y as u32);
    let pixel = pixel[0] as usize + pixel[1] as usize + pixel[2] as usize;

    let mut min_dist = usize::MAX;
    let mut min_block = DEFAULT_TEXTURE_ID;

    for (block, (r, g, b), c) in available_textures {
        // let mut dist: usize = spixsq + pixel * (spix + cpix * pixel);
        let dist = r.1 + g.1 + b.1 + pixel * (r.0 + g.0 + b.0 + pixel * c);

        if dist < min_dist {
            min_dist = dist;
            min_block = *block;
        }
    }

    min_block
}

/// Resolution up samples the model. Increase this if there are many "holes" in the resulting array.
pub fn model_2_arr(
    model: Model,
    dims: CoordXYZ,
    resolution: float,
    available_textures: Vec<(Block, image::DynamicImage)>,
) -> ArrayModel {
    let available_textures = precompute_imgs(&available_textures);
    let texture_faces = model.texture_faces;
    let texture_coords = model.texture_coords;
    let texture_image = model.texture_img;

    let vertices = model.vertices.0;

    let mut array_model = ArrayModel::new(dims, resolution);

    for i in 0..model.faces.0.len() {
        let face = &model.faces.0[i];
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

        let texture_plane = optional_texture_plane(&texture_faces, &texture_coords);


        let bounds = plane.bounds();
        let (x_range, y_range, z_range) = bounds_to_range(bounds, resolution);
        let fills_z = plane.fills_z();
        for x in x_range.clone() {
            for y in y_range.clone() {
                let p = Vec2::new(x as float / resolution, y as float / resolution);
                let weights = plane.calc_weights(&p);

                if TriangularPlane::weights_within_plane(weights) {
                    let x = x / resolution as usize;
                    let y = y / resolution as usize;

                    // TODO: What if fills_z == true?
                    let block = get_texture_id(
                        weights,
                        &texture_plane,
                        &texture_image,
                        &available_textures,
                    );

                    if fills_z {
                        for z in z_range.clone() {
                            let z = (z as float / resolution).round() as usize;
                            array_model.set((x, y, z), block);
                        }
                    } else if !fills_z {
                        let z = plane.calculate_z(&p);
                        let z = z.round() as usize;
                        if z < array_model.dims.2.into() {
                            array_model.set((x, y, z), block);
                        }
                    }
                }
            }
        }
    }
    array_model
}