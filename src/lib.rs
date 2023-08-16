pub mod vec3;
pub mod coords;
pub mod utils;
pub mod model;
pub mod barycentric;

pub type F = f32;

pub const PI: F = std::f64::consts::PI as F;
pub const DEG2RAD: F = PI / 180.0;
pub const RAD2DEG: F = 180.0 / PI;

pub fn load_default<P>(p: P) -> tobj::LoadResult
    where
        P: AsRef<std::path::Path> + std::fmt::Debug,
{
    tobj::load_obj(&p, &tobj::LoadOptions::default())
}

