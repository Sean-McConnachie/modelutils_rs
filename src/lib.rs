use serde::{Deserialize, Serialize};
use crate::model2arr::Block;

/// ```
/// use modelutils_rs::{model, model2arr};
/// use modelutils_rs::vec3::Vec3;
/// use modelutils_rs::float;
///
/// const PATH: &str = "shapes/rotated_puppet.obj";
///
/// const RESOLUTION: float = 100.0;
/// const S: usize = 100;
/// const DIMS: (usize, usize, usize) = (S, S, S);
/// const SCALE_DIMS: (float, float, float) = ((DIMS.0 - 1) as float, (DIMS.1 - 1) as float, (DIMS.2 - 1) as float);
///
/// fn main() {
///     let scale_vec = Vec3::new(SCALE_DIMS.0, SCALE_DIMS.1, SCALE_DIMS.2);
///     let (models, _materials) = modelutils_rs::load_default(&PATH).unwrap();
///
///     let mut models = models
///         .into_iter()
///         .map(|m| model::Model::new(
///             model::Points::from_flat_vec(m.mesh.positions),
///             model::Faces::from_triangles(m.mesh.indices),
///         ))
///         .collect::<Vec<model::Model>>();
///
///     for mut model in models.into_iter() {
///         // Align model to origin
///         let bounds = model.model_dims();
///         model.mv(bounds.0 * Vec3::from_scalar(-1.0));
///
///         // Scale model to fit in 10x10x10 cube
///         let scale = model.scale_for_box(scale_vec);
///         model.scale(Vec3::from_scalar(scale.min_val()));
///
///         // Convert to array
///         let arr3d = model2arr::model_2_arr(model, DIMS, RESOLUTION);
///
///         // Save to json for unity
///         model2arr::arr_2_json("arr.json", arr3d).unwrap();
///
///         break;
///     }
/// }
/// ```

pub mod vec3;
pub mod coords;
pub mod utils;
pub mod model;
pub mod model2arr;
pub mod vec2;

#[allow(non_camel_case_types)]
pub type float = f32;

pub const PI: float = std::f64::consts::PI as float;
pub const DEG2RAD: float = PI / 180.0;
pub const RAD2DEG: float = 180.0 / PI;

pub fn load_default<P>(p: P) -> tobj::LoadResult
    where
        P: AsRef<std::path::Path> + std::fmt::Debug,
{
    let opts = tobj::LoadOptions {
        // single_index: true,
        triangulate: true,
        ..Default::default()
    };
    tobj::load_obj(&p, &opts)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TextureNames {
    textures: Vec<String>,
}

pub fn load_textures<P>(dir: P) -> (TextureNames, Vec<(Block, image::DynamicImage)>)
where
        P: AsRef<std::path::Path> + std::fmt::Debug, {
    let mut textures = Vec::new();
    let mut texture_names = Vec::new();
    for (i, entry) in std::fs::read_dir(dir).unwrap().enumerate() {
        let entry = entry.unwrap();
        let path = entry.path();
        let img = image::open(path).unwrap();
        textures.push((i as Block, img));
        let fname = entry.file_name().into_string().unwrap();
        texture_names.push(fname);
    }
    (TextureNames { textures: texture_names }, textures)
}