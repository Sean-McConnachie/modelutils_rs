use modelutils_rs::parse;

const PATH: &str = "puppet.obj";

fn main1() {
    let (models, _materials) = parse::load_default(&PATH).unwrap();

    let mut models = models
        .into_iter()
        .map(|m| parse::Vertex::from_model(m.mesh))
        .collect::<Vec<parse::Model>>();
    let mut model = &mut models[0];

    parse::rotate_model(
        &mut model,
        parse::Order::XYZ,
        parse::Rotation::new_deg(0.0, 90.0, 0.0),
    );

    let dims = parse::ModelDims::from_model(&model);
    let global_scale = dims.global_scale(
        None,
        Some(parse::Range::new(0.0, 2.5)),
        Some(parse::Range::new(0.0, 5.0)),
    );
    dbg!(&dims);
    dbg!(&global_scale);

    parse::scale_model(
        &mut model,
        parse::Vertex::new(global_scale, global_scale, global_scale),
    );

    let dims = parse::ModelDims::from_model(&model);
    let global_scale = dims.global_scale(
        None,
        Some(parse::Range::new(0.0, 2.5)),
        Some(parse::Range::new(0.0, 5.0)),
    );
    dbg!(&dims);
    dbg!(&global_scale);

    
}

fn main() {
    let obj_file = std::env::args()
        .skip(1)
        .next()
        .expect("A .obj file to print is required");

    let (models, materials) =
        tobj::load_obj(&obj_file, &tobj::LoadOptions::default()).expect("Failed to OBJ load file");

    // Note: If you don't mind missing the materials, you can generate a default.
    //let materials = materials.expect("Failed to load MTL file");

    println!("Number of models          = {}", models.len());
    //println!("Number of materials       = {}", materials.len());

    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!("");
        println!("model[{}].name             = \'{}\'", i, m.name);
        println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

        println!(
            "model[{}].face_count       = {}",
            i,
            mesh.face_arities.len()
        );

        let mut next_face = 0;
        for face in 0..mesh.face_arities.len() {
            let end = next_face + mesh.face_arities[face] as usize;

            let face_indices = &mesh.indices[next_face..end];
            println!(" face[{}].indices          = {:?}", face, face_indices);

            if !mesh.texcoord_indices.is_empty() {
                let texcoord_face_indices = &mesh.texcoord_indices[next_face..end];
                println!(
                    " face[{}].texcoord_indices = {:?}",
                    face, texcoord_face_indices
                );
            }
            if !mesh.normal_indices.is_empty() {
                let normal_face_indices = &mesh.normal_indices[next_face..end];
                println!(
                    " face[{}].normal_indices   = {:?}",
                    face, normal_face_indices
                );
            }

            next_face = end;
        }

        // Normals and texture coordinates are also loaded, but not printed in
        // this example.
        println!(
            "model[{}].positions        = {}",
            i,
            mesh.positions.len() / 3
        );
        assert!(mesh.positions.len() % 3 == 0);

        for vtx in 0..mesh.positions.len() / 3 {
            //println!(
            //"              position[{}] = ({}, {}, {})",
            //vtx,
            //mesh.positions[3 * vtx],
            //mesh.positions[3 * vtx + 1],
            //mesh.positions[3 * vtx + 2]
            //);
        }
    }

    //for (i, m) in materials.iter().enumerate() {
    //let ambient = m.ambient.unwrap();
    //let diffuse = m.diffuse.unwrap();
    //let specular = m.specular.unwrap();
    //println!("material[{}].name = \'{}\'", i, m.name);
    //println!(
    //"    material.Ka = ({}, {}, {})",
    //ambient[0], ambient[1], ambient[2]
    //);
    //println!(
    //"    material.Kd = ({}, {}, {})",
    //diffuse[0], diffuse[1], diffuse[2]
    //);
    //println!(
    //"    material.Ks = ({}, {}, {})",
    //specular[0], specular[1], specular[2]
    //);
    //println!("    material.Ns = {:?}", m.shininess);
    //println!("    material.d = {:?}", m.dissolve);
    //println!("    material.map_Ka = {:?}", m.ambient_texture);
    //println!("    material.map_Kd = {:?}", m.diffuse_texture);
    //println!("    material.map_Ks = {:?}", m.specular_texture);
    //println!("    material.map_Ns = {:?}", m.shininess_texture);
    //println!("    material.map_Bump = {:?}", m.normal_texture);
    //println!("    material.map_d = {:?}", m.dissolve_texture);

    //for (k, v) in &m.unknown_param {
    //println!("    material.{} = {}", k, v);
    //}
    //}
}
