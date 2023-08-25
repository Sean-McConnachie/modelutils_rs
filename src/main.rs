use std::path;
use modelutils_rs::{model, model2arr};
use modelutils_rs::vec3::Vec3;
use modelutils_rs::float;
use modelutils_rs::model2arr::uint;

// const PATH: &str = "shapes/cube/untitled.obj";
// const TEXTURE_PATH: &str = "shapes/cube/tex.jpg";

const PATH: &str = "shapes/cottage/cottage_obj.obj";
const TEXTURE_PATH: &str = "shapes/cottage/cottage_textures/cottage_diffuse.png";

const TEXTURE_DIR: &str = "textures/";

const RESOLUTION: float = 100.0;
const S: uint = 50;
const DIMS: (uint, uint, uint) = (S, S, S);
const SCALE_DIMS: (float, float, float) = ((DIMS.0 - 1) as float, (DIMS.1 - 1) as float, (DIMS.2 - 1) as float);


fn main() {
    let texture_path = path::PathBuf::from(TEXTURE_PATH);

    let (texture_names, available_textures) = modelutils_rs::load_textures(TEXTURE_DIR);
    let texture_names_json = serde_json::to_string(&texture_names).unwrap();
    std::fs::write("texture_names.json", texture_names_json).unwrap();

    let scale_vec = Vec3::new(SCALE_DIMS.0, SCALE_DIMS.1, SCALE_DIMS.2);
    let (models, _materials) = modelutils_rs::load_default(&PATH).unwrap();

    // println!("=== Positions ===\n{:?}\n{:?}", &models[4].mesh.positions, &models[4].mesh.positions.len());
    // println!("=== Indicies ===\n{:?}\n{:?}", &models[4].mesh.indices, &models[4].mesh.indices.len());
    // println!("=== Texcoords ===\n{:?}\n{:?}", models[4].mesh.texcoords, models[4].mesh.texcoords.len());
    // println!("=== Texcoordinds ===\n{:?}\n{:?}", models[4].mesh.texcoord_indices, models[4].mesh.texcoord_indices.len());

    let models = models
        .into_iter()
        .map(|m| model::Model::new(
            model::Points::from_flat_vec(m.mesh.positions),
            model::Faces::from_triangles(m.mesh.indices),
            Some(path::PathBuf::from(texture_path.clone())),
            model::TextureCoords::from_flat_vec(m.mesh.texcoords),
            model::TextureFaces::from_triangles(m.mesh.texcoord_indices),
        ))
        .collect::<Vec<model::Model>>();

    for mut model in models.into_iter().skip(0) {

        // Align model to origin
        let bounds = model.model_dims();
        model.mv(bounds.0 * Vec3::from_scalar(-1.0));

        // Scale model to fit in 10x10x10 cube
        let scale = model.scale_for_box(scale_vec.clone());
        model.scale(Vec3::from_scalar(scale.min_val()));

        // Convert to array
        let arr3d = model2arr::model_2_arr(
            model,
            DIMS,
            RESOLUTION,
            available_textures.clone(),
        );

        // Save to json for unity
        model2arr::arr_2_json("arr.json", arr3d).unwrap();

        // break;
    }
}