use crate::camera::Camera;
use crate::geometry::hitable::Hitable;
use crate::geometry::sphere::Sphere;
use crate::linalg::Vec3;
use crate::materials::material::Material;
use crate::materials::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::textures::texture::Texture;
use crate::textures::{CheckerTexture, ConstantTexture};

use std::collections::HashMap;
use std::sync::Arc;
use std::vec::Vec;

pub struct Scene {
    pub camera: Camera,
    pub shapes: Vec<Arc<Hitable>>,
    pub materials: HashMap<String, Arc<Material>>,
    pub textures: HashMap<String, Arc<Texture>>,
}

impl Scene {
    pub fn new() -> Self {
        // let mut world = HitableList::new();

        // world.push(Arc::new(XYRect::new(
        //     -1.5,
        //     1.0,
        //     0.0,
        //     3.0,
        //     2.0,
        //     Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(
        //         1.0 * Vec3::new(1.0, 1.0, 1.0),
        //     )))),
        // )));
        let parsed = json::parse(
            r#"

        {
            "camera": {
                "look_from": [8.0, 0.5, 2.0],
                "look_at": [0.0, 0.0, 0.0],
                "vup": [0.0, 1.0, 0.0],
                "vfov": 40.0,
                "aspect": 1.0,
                "aperture": 0.1,
                "focus_dist": 8.0,
                "t_open": 0.0,
                "t_close": 1.0
            },
            "textures": {
                "white": {
                    "type": "constant",
                    "color": [1.0, 1.0, 1.0]
                },
                "black": {
                    "type": "constant",
                    "color": [0.0, 0.0, 0.0]
                },
                "blueish": {
                    "type": "constant",
                    "color": [0.05, 0.1, 0.8]
                },
                "redish": {
                    "type": "constant",
                    "color": [0.9, 0.05, 0.1]
                },
                "checker": {
                    "type": "checker",
                    "odd": "black",
                    "even": "white"
                }
            },
            "materials": {
                "glass": {
                    "type": "dielectric",
                    "ref_idx": 1.5
                },
                "white_lambertian": {
                    "type": "lambertian",
                    "albedo": "white"
                },
                "checkerboard": {
                    "type": "lambertian",
                    "albedo": "checker"
                },
                "dim_light": {
                    "type": "diffuse_light",
                    "emit_tex": "white"
                },
                "blue_metalic": {
                    "type": "metal",
                    "albedo": "blueish",
                    "fuzz": 0.2
                },
                "red_metalic": {
                    "type": "metal",
                    "albedo": "redish",
                    "fuzz": 0.01
                }
            },
            "shapes": [
                {
                    "type": "sphere",
                    "center": [0.0, -1000.0, 0.0],
                    "radius": 1000.0,
                    "material": "checkerboard"
                },
                {
                    "type": "sphere",
                    "center": [0.0, 1.4, 0.0],
                    "radius": 1.1,
                    "material": "blue_metalic"
                },
                {
                    "type": "sphere",
                    "center": [1.5, 0.5, 1.5],
                    "radius": 0.5,
                    "material": "red_metalic"
                },
                {
                    "type": "sphere",
                    "center": [3.5, 0.65, 0.1],
                    "radius": 0.65,
                    "material": "glass"
                },
                {
                    "type": "sphere",
                    "center": [0.0, 6.2, 2.0],
                    "radius": 2.0,
                    "material": "dim_light"
                }
            ]
        }

        "#,
        )
        .unwrap();

        // world.push(Arc::new(Sphere::new(
        //     Vec3::new(0.0, 6.2, 2.0),
        //     2.0,
        //     Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(
        //         1.0 * Vec3::new(1.0, 1.0, 1.0),
        //     )))),
        // )));

        let camera = build_camera_from_json(&parsed["camera"]);
        let textures = build_textures_from_json(&parsed["textures"]);
        let materials = build_materials_from_json(&parsed["materials"], &textures);
        let shapes = build_shapes_from_json(&parsed["shapes"], &materials);

        Self {
            camera: camera,
            shapes: shapes,
            materials: materials,
            textures: textures,
        }
    }
}

fn build_vector_from_json(data: &json::JsonValue) -> Vec3 {
    Vec3::new(
        data[0].as_f32().unwrap(),
        data[1].as_f32().unwrap(),
        data[2].as_f32().unwrap(),
    )
}

fn build_camera_from_json(data: &json::JsonValue) -> Camera {
    Camera::new(
        build_vector_from_json(&data["look_from"]),
        build_vector_from_json(&data["look_at"]),
        build_vector_from_json(&data["vup"]),
        data["vfov"].as_f32().unwrap(),
        data["aspect"].as_f32().unwrap(),
        data["aperture"].as_f32().unwrap(),
        data["focus_dist"].as_f32().unwrap(),
        data["t_open"].as_f32().unwrap(),
        data["t_close"].as_f32().unwrap(),
    )
}

fn build_textures_from_json(data: &json::JsonValue) -> HashMap<String, Arc<Texture>> {
    let mut textures = HashMap::new();

    for (name, texture_data) in data.entries() {
        let t_type = texture_data["type"].as_str().unwrap();
        let texture: Arc<Texture> = match t_type {
            "constant" => Arc::new(ConstantTexture::new(build_vector_from_json(
                &texture_data["color"],
            ))),
            "checker" => {
                let odd_name = String::from(texture_data["odd"].as_str().unwrap());
                let even_name = String::from(texture_data["even"].as_str().unwrap());
                let odd: &Arc<Texture> = textures.get(&odd_name).unwrap();
                let even: &Arc<Texture> = textures.get(&even_name).unwrap();
                Arc::new(CheckerTexture::new(odd.clone(), even.clone()))
            }
            _ => panic!("Unknown texture type"),
        };
        textures.insert(String::from(name), texture);
    }

    textures
}

fn build_materials_from_json(
    data: &json::JsonValue,
    textures: &HashMap<String, Arc<Texture>>,
) -> HashMap<String, Arc<Material>> {
    let mut materials = HashMap::new();

    for (name, material_data) in data.entries() {
        let m_type = material_data["type"].as_str().unwrap();
        let material: Arc<Material> = match m_type {
            "dielectric" => Arc::new(Dielectric::new(material_data["ref_idx"].as_f32().unwrap())),
            "diffuse_light" => {
                let tex_name = String::from(material_data["emit_tex"].as_str().unwrap());
                let emit_tex: &Arc<Texture> = textures.get(&tex_name).unwrap();
                Arc::new(DiffuseLight::new(emit_tex.clone()))
            }
            "lambertian" => {
                let tex_name = String::from(material_data["albedo"].as_str().unwrap());
                let tex: &Arc<Texture> = textures.get(&tex_name).unwrap();
                Arc::new(Lambertian::new(tex.clone()))
            }
            "metal" => {
                let tex_name = String::from(material_data["albedo"].as_str().unwrap());
                let fuzz = material_data["fuzz"].as_f32().unwrap();
                let tex: &Arc<Texture> = textures.get(&tex_name).unwrap();
                Arc::new(Metal::new(tex.clone(), fuzz))
            }
            _ => panic!("Unknown material type"),
        };
        materials.insert(String::from(name), material);
    }

    materials
}

fn build_shapes_from_json(
    data: &json::JsonValue,
    materials: &HashMap<String, Arc<Material>>,
) -> Vec<Arc<Hitable>> {
    let mut shapes = Vec::new();
    for shape_data in data.members() {
        let s_type = shape_data["type"].as_str().unwrap();
        let shape: Arc<Hitable> = match s_type {
            "sphere" => {
                let mat_name = String::from(shape_data["material"].as_str().unwrap());
                let mat: &Arc<Material> = materials.get(&mat_name).unwrap();
                Arc::new(Sphere::new(
                    build_vector_from_json(&shape_data["center"]),
                    shape_data["radius"].as_f32().unwrap(),
                    mat.clone(),
                ))
            }
            _ => panic!("Unknown shape type"),
        };
        shapes.push(shape);
    }
    shapes
}
