use modelutils_rs::{model, model2arr};
use modelutils_rs::vec3::Vec3;

const PATH: &str = "shapes/cube.obj";

fn main() {
    let (models, _materials) = modelutils_rs::load_default(&PATH).unwrap();

    let mut models = models
        .into_iter()
        .map(|m| model::Model::new(
            model::Points::from_flat_vec(m.mesh.positions),
            model::Faces::from_triangles(m.mesh.indices),
        ))
        .collect::<Vec<model::Model>>();

    for mut model in models.into_iter() {
        // Align model to origin
        let bounds = model.model_dims();
        model.mv(bounds.0 * Vec3::from_scalar(-1.0));

        // Scale model to fit in 10x10x10 cube
        let scale = model.scale_for_box(Vec3::new(9.0, 9.0, 9.0));
        model.scale(Vec3::from_scalar(scale.min_val()));

        // Convert to array
        let arr3d = model2arr::model_2_arr(model, (10, 10, 10));
    }
}

