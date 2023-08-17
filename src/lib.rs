pub mod vec3;
pub mod coords;
pub mod utils;
pub mod model;
pub mod model2arr;

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

