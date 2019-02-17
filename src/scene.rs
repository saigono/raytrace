use crate::camera::Camera;
use crate::geometry::bvh_node::BVHNode;
use crate::geometry::hitable::Hitable;
use crate::geometry::hitable_list::HitableList;
use crate::geometry::sphere::Sphere;
use crate::geometry::triangle::TriangleMesh;
use crate::linalg::Vec3;
use crate::materials::material::Material;
use crate::materials::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::textures::texture::Texture;
use crate::textures::{CheckerTexture, ConstantTexture};

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::sync::Arc;
use std::vec::Vec;

pub struct Scene {
    pub camera: Camera,
    pub shapes: Vec<Arc<Hitable>>,
    pub materials: HashMap<String, Arc<Material>>,
    pub textures: HashMap<String, Arc<Texture>>,
}

impl Scene {
    pub fn new(path_to_file: &str) -> Self {
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
        let path = Path::new(path_to_file);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };
        let mut source = String::new();
        file.read_to_string(&mut source).unwrap();
        let parsed = json::parse(source.as_mut_str()).unwrap();

        // world.push(Arc::new(Sphere::new(
        //     Vec3::new(0.0, 6.2, 2.0),
        //     2.0,
        //     Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(
        //         1.0 * Vec3::new(1.0, 1.0, 1.0),
        //     )))),
        // )));

        // dbg!("file parsed");
        let camera = build_camera_from_json(&parsed["camera"]);
        // dbg!("camera read");
        let textures = build_textures_from_json(&parsed["textures"]);
        // dbg!("textures read");
        let materials = build_materials_from_json(&parsed["materials"], &textures);
        // dbg!("materials read");
        let shapes = build_shapes_from_json(&parsed["shapes"], &materials);
        // dbg!("shapes read");
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
            // "triangle" => {
            //     let mat_name = String::from(shape_data["material"].as_str().unwrap());
            //     let mat: &Arc<Material> = materials.get(&mat_name).unwrap();
            //     Arc::new(Triangle::new(
            //         &build_vector_from_json(&shape_data["vertices"][0]),
            //         &build_vector_from_json(&shape_data["vertices"][1]),
            //         &build_vector_from_json(&shape_data["vertices"][2]),
            //         mat.clone(),
            //     ))
            // }
            "triangle_mesh" => {
                let mat_name = String::from(shape_data["material"].as_str().unwrap());
                let mat: &Arc<Material> = materials.get(&mat_name).unwrap();
                let triangle_mesh = build_triangle_mesh_from_obj(
                    &shape_data["filename"].as_str().unwrap(),
                    mat.clone(),
                );
                let mut hl = HitableList::new();
                for t in triangle_mesh.iter() {
                    hl.push(Arc::new(t));
                }
                Arc::new(BVHNode::new(hl.list.as_mut_slice(), 0.0, 1.0))
            }
            _ => panic!("Unknown shape type"),
        };
        shapes.push(shape);
    }
    shapes
}

fn build_triangle_mesh_from_obj(path_to_file: &str, material: Arc<Material>) -> TriangleMesh {
    let path = Path::new(path_to_file);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut v_index = Vec::new();
    let mut n_index = Vec::new();
    let mut n_triangles = 0;

    for line in BufReader::new(file).lines() {
        // println!("{}", line.unwrap());
        let l = line.unwrap();
        if l.starts_with("v ") {
            let coord: Vec<f32> = l
                .split(" ")
                .skip(1)
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<f32>().unwrap())
                .collect();
            vertices.push(Vec3::new(coord[0], coord[1], coord[2]));
        } else if l.starts_with("vn") {
            let coord: Vec<f32> = l
                .split(" ")
                .skip(1)
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<f32>().unwrap())
                .collect();
            normals.push(Vec3::new(coord[0], coord[1], coord[2]));
        } else if l.starts_with("f ") {
            for (v, _, n) in l
                .split(" ")
                .skip(1)
                .filter(|v| !v.is_empty())
                .map(parse_face_fragment)
            {
                v_index.push(v);
                n_index.push(n);
            }
            n_triangles += 1;
        }
    }
    dbg!(normals.len());
    dbg!(n_triangles);
    TriangleMesh::new(
        n_triangles,
        Arc::new(vertices),
        Arc::new(normals),
        v_index,
        n_index,
        material.clone(),
    )
}

fn parse_face_fragment(s: &str) -> (usize, usize, usize) {
    let parts: Vec<&str> = s.split("/").collect();
    match parts.len() {
        1 => {
            let v = parts[0].parse::<usize>().unwrap();
            (v, 0, v)
        }
        3 => {
            let tc = match parts[1].parse::<usize>() {
                Ok(x) => x,
                Err(_) => 0,
            };
            (
                parts[0].parse::<usize>().unwrap(),
                tc,
                parts[2].parse::<usize>().unwrap(),
            )
        }
        _ => panic!("unknown format"),
    }
}
