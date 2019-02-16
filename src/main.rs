mod camera;
mod geometry;
mod image;
mod linalg;
mod materials;
mod random;
mod scene;
mod textures;

use geometry::bvh_node::BVHNode;
use geometry::hitable::Hitable;
use geometry::hitable_list::HitableList;
use linalg::{Ray, Vec3};

use std::env;
use std::sync::{mpsc, Arc};
use std::time::SystemTime;

use rand::Rng;
use threadpool::ThreadPool;

extern crate num_cpus;
#[macro_use]
extern crate json;

// fn random_scene() -> BVHNode {
//     let mut world = HitableList::new();
//     world.push(Arc::new(Sphere::new(
//         Vec3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         Arc::new(Lambertian::new(Arc::new(CheckerTexture::new(
//             Arc::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
//             Arc::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
//         )))),
//     )));
//     let mut rng = rand::thread_rng();
//     for a in -11..11 {
//         for b in -11..11 {
//             let choose_mat: f32 = rng.gen();
//             let off_x: f32 = rng.gen();
//             let off_z: f32 = rng.gen();
//             let center = Vec3::new((a as f32) + 0.9 * off_x, 0.2, (b as f32) + 0.9 * off_z);
//             if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
//                 if choose_mat < 0.8 {
//                     world.push(Arc::new(Sphere::new(
//                         center,
//                         0.2,
//                         Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
//                             rng.gen::<f32>() * rng.gen::<f32>(),
//                             rng.gen::<f32>() * rng.gen::<f32>(),
//                             rng.gen::<f32>() * rng.gen::<f32>(),
//                         ))))),
//                     )));
//                 } else if choose_mat < 0.95 {
//                     world.push(Arc::new(Sphere::new(
//                         center,
//                         0.2,
//                         Arc::new(Metal::new(
//                             Arc::new(ConstantTexture::new(Vec3::new(
//                                 0.5 * (1.0 + rng.gen::<f32>()),
//                                 0.5 * (1.0 + rng.gen::<f32>()),
//                                 0.5 * (1.0 + rng.gen::<f32>()),
//                             ))),
//                             0.5 * (1.0 + rng.gen::<f32>()),
//                         )),
//                     )));
//                 } else {
//                     world.push(Arc::new(Sphere::new(
//                         center,
//                         0.2,
//                         Arc::new(Dielectric::new(1.5)),
//                     )));
//                 }
//             }
//         }
//     }

//     world.push(Arc::new(Sphere::new(
//         Vec3::new(0.0, 1.0, 0.0),
//         1.0,
//         Arc::new(Dielectric::new(1.5)),
//     )));
//     world.push(Arc::new(Sphere::new(
//         Vec3::new(-4.0, 1.0, 0.0),
//         1.0,
//         Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
//             0.4, 0.2, 0.1,
//         ))))),
//     )));
//     world.push(Arc::new(Sphere::new(
//         Vec3::new(4.0, 1.0, 0.0),
//         1.0,
//         Arc::new(Metal::new(
//             Arc::new(ConstantTexture::new(Vec3::new(0.7, 0.6, 0.5))),
//             0.0,
//         )),
//     )));
//     BVHNode::new(world.list.as_mut_slice(), 0.0, 1.0)
// }

// fn perlin_scene() -> BVHNode {
//     let perlin_text = Arc::new(PerlinTexture::new());
//     let material = Arc::new(Lambertian::new(perlin_text));
//     let mut world = HitableList::new();
//     world.push(Arc::new(Sphere::new(
//         Vec3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         material,
//     )));
//     world.push(Arc::new(Sphere::new(
//         Vec3::new(0.0, 2.0, 0.0),
//         2.0,
//         Arc::new(Lambertian::new(Arc::new(PerlinTexture::new()))),
//     )));
//     BVHNode::new(world.list.as_mut_slice(), 0.0, 1.0)
// }

// fn earth_scene() -> BVHNode {
//     let (image_data, width, height) = image::read_png("earthmap.png");
//     let texture = ImageTexture::new(&image_data, width, height);
//     let mut world = HitableList::new();
//     world.push(Arc::new(Sphere::new(
//         Vec3::new(0.0, 0.0, 0.0),
//         2.0,
//         Arc::new(Lambertian::new(Arc::new(texture))),
//     )));
//     BVHNode::new(world.list.as_mut_slice(), 0.0, 1.0)
// }

// fn lighted_scene() -> BVHNode {
//     let mut world = HitableList::new();

//     world.push(Arc::new(Sphere::new(
//         Vec3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
//             0.8, 0.9, 0.6,
//         ))))),
//     )));

//     world.push(Arc::new(Sphere::new(
//         Vec3::new(0.0, 1.4, 0.0),
//         1.1,
//         Arc::new(Metal::new(
//             Arc::new(ConstantTexture::new(Vec3::new(0.9, 0.8, 0.9))),
//             0.1,
//         )),
//     )));

//     world.push(Arc::new(Sphere::new(
//         Vec3::new(1.5, 0.25, 1.0),
//         0.25,
//         Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
//             0.9, 0.1, 0.1,
//         ))))),
//     )));

//     world.push(Arc::new(Sphere::new(
//         Vec3::new(1.5, 0.6, 3.0),
//         0.6,
//         Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
//             0.2, 0.6, 0.9,
//         ))))),
//     )));

//     world.push(Arc::new(Sphere::new(
//         Vec3::new(3.5, 0.8, -1.7),
//         0.8,
//         Arc::new(Dielectric::new(1.5)),
//     )));

//     world.push(Arc::new(Sphere::new(
//         Vec3::new(0.0, 6.2, 2.0),
//         2.0,
//         Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(
//             1.0 * Vec3::new(1.0, 1.0, 1.0),
//         )))),
//     )));

//     world.push(Arc::new(XYRect::new(
//         -1.5,
//         1.0,
//         0.0,
//         3.0,
//         2.0,
//         Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(
//             1.0 * Vec3::new(1.0, 1.0, 1.0),
//         )))),
//     )));

//     BVHNode::new(world.list.as_mut_slice(), 0.0, 1.0)
// }

// fn cornell_box_scene() -> BVHNode {
//     let mut world = HitableList::new();
//     let red = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
//         0.65, 0.05, 0.05,
//     )))));
//     let white = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
//         0.73, 0.73, 0.73,
//     )))));
//     let green = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
//         0.12, 0.45, 0.15,
//     )))));
//     let light = Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(
//         15.0 * Vec3::new(1.0, 1.0, 1.0),
//     ))));

//     world.push(Arc::new(FlipNormals::new(Arc::new(YZRect::new(
//         0.0, 0.0, 555.0, 555.0, 555.0, green,
//     )))));
//     world.push(Arc::new(YZRect::new(0.0, 0.0, 555.0, 555.0, 0.0, red)));
//     world.push(Arc::new(XZRect::new(
//         213.0,
//         227.0,
//         343.0,
//         332.0,
//         554.0,
//         light.clone(),
//     )));
//     // world.push(Arc::new(FlipNormals::new(Arc::new())));
//     world.push(Arc::new(FlipNormals::new(Arc::new(XZRect::new(
//         0.0,
//         0.0,
//         555.0,
//         555.0,
//         555.0,
//         white.clone(),
//     )))));
//     world.push(Arc::new(XZRect::new(
//         0.0,
//         0.0,
//         555.0,
//         555.0,
//         0.0,
//         white.clone(),
//     )));
//     world.push(Arc::new(FlipNormals::new(Arc::new(XYRect::new(
//         0.0,
//         0.0,
//         555.0,
//         555.0,
//         555.0,
//         white.clone(),
//     )))));

//     world.push(Arc::new(Translation::new(
//         Arc::new(YRotation::new(
//             Arc::new(BoxObject::new(
//                 Vec3::new(0.0, 0.0, 0.0),
//                 Vec3::new(165.0, 165.0, 165.0),
//                 white.clone(),
//             )),
//             -18.0,
//         )),
//         Vec3::new(130.0, 0.0, 65.0),
//     )));

//     world.push(Arc::new(Translation::new(
//         Arc::new(YRotation::new(
//             Arc::new(BoxObject::new(
//                 Vec3::new(0.0, 0.0, 0.0),
//                 Vec3::new(165.0, 330.0, 165.0),
//                 white.clone(),
//             )),
//             15.0,
//         )),
//         Vec3::new(265.0, 0.0, 295.0),
//     )));

//     BVHNode::new(world.list.as_mut_slice(), 0.0, 1.0)
// }

// fn cornell_smoke_scene() -> BVHNode {
//     let mut world = HitableList::new();
//     let red = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
//         0.65, 0.05, 0.05,
//     )))));
//     let white = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
//         0.73, 0.73, 0.73,
//     )))));
//     let green = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
//         0.12, 0.45, 0.15,
//     )))));
//     let light = Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(
//         3.0 * Vec3::new(1.0, 1.0, 1.0),
//     ))));

//     world.push(Arc::new(FlipNormals::new(Arc::new(YZRect::new(
//         0.0, 0.0, 555.0, 555.0, 555.0, green,
//     )))));
//     world.push(Arc::new(YZRect::new(0.0, 0.0, 555.0, 555.0, 0.0, red)));
//     world.push(Arc::new(XZRect::new(
//         113.0,
//         127.0,
//         443.0,
//         432.0,
//         554.0,
//         light.clone(),
//     )));

//     world.push(Arc::new(FlipNormals::new(Arc::new(XZRect::new(
//         0.0,
//         0.0,
//         555.0,
//         555.0,
//         555.0,
//         white.clone(),
//     )))));
//     world.push(Arc::new(XZRect::new(
//         0.0,
//         0.0,
//         555.0,
//         555.0,
//         0.0,
//         white.clone(),
//     )));
//     world.push(Arc::new(FlipNormals::new(Arc::new(XYRect::new(
//         0.0,
//         0.0,
//         555.0,
//         555.0,
//         555.0,
//         white.clone(),
//     )))));

//     let box1 = Arc::new(Translation::new(
//         Arc::new(YRotation::new(
//             Arc::new(BoxObject::new(
//                 Vec3::new(0.0, 0.0, 0.0),
//                 Vec3::new(165.0, 165.0, 165.0),
//                 white.clone(),
//             )),
//             -18.0,
//         )),
//         Vec3::new(130.0, 0.0, 65.0),
//     ));

//     let box2 = Arc::new(Translation::new(
//         Arc::new(YRotation::new(
//             Arc::new(BoxObject::new(
//                 Vec3::new(0.0, 0.0, 0.0),
//                 Vec3::new(165.0, 330.0, 165.0),
//                 white.clone(),
//             )),
//             15.0,
//         )),
//         Vec3::new(265.0, 0.0, 295.0),
//     ));

//     world.push(Arc::new(ConstantMedium::new(
//         box1,
//         0.01,
//         Arc::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
//     )));
//     world.push(Arc::new(ConstantMedium::new(
//         box2,
//         0.01,
//         Arc::new(ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0))),
//     )));

//     BVHNode::new(world.list.as_mut_slice(), 0.0, 1.0)
// }

// fn complex_scene() -> BVHNode {
//     let mut world = HitableList::new();
//     let mut boxlist = HitableList::new();
//     let mut boxlist2 = HitableList::new();

//     let white = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
//         0.73, 0.73, 0.73,
//     )))));
//     let ground = Arc::new(Lambertian::new(Arc::new(ConstantTexture::new(Vec3::new(
//         0.48, 0.83, 0.53,
//     )))));
//     let light = Arc::new(DiffuseLight::new(Arc::new(ConstantTexture::new(
//         5.0 * Vec3::new(1.0, 1.0, 1.0),
//     ))));
//     let glass = Arc::new(Dielectric::new(1.5));

//     let mut rng = rand::thread_rng();

//     for i in 0..20 {
//         for j in 0..20 {
//             let w = 100.0;
//             let x0 = -1000.0 + (i as f32) * w;
//             let z0 = -1000.0 + (j as f32) * w;
//             let y0 = 0.0;
//             let x1 = x0 + w;
//             let z1 = z0 + w;
//             let y1 = 100.0 * (rng.gen::<f32>() + 0.01);
//             boxlist.push(Arc::new(BoxObject::new(
//                 Vec3::new(x0, y0, z0),
//                 Vec3::new(x1, y1, z1),
//                 ground.clone(),
//             )));
//         }
//     }
//     world.push(Arc::new(BVHNode::new(
//         boxlist.list.as_mut_slice(),
//         0.0,
//         1.0,
//     )));

//     for j in 0..1000 {
//         boxlist2.push(Arc::new(Sphere::new(
//             Vec3::new(
//                 165.0 * rng.gen::<f32>(),
//                 165.0 * rng.gen::<f32>(),
//                 165.0 * rng.gen::<f32>(),
//             ),
//             10.0,
//             white.clone(),
//         )));
//     }
//     world.push(Arc::new(Translation::new(
//         Arc::new(YRotation::new(
//             Arc::new(BVHNode::new(boxlist2.list.as_mut_slice(), 0.0, 1.0)),
//             15.0,
//         )),
//         Vec3::new(-100.0, 270.0, 395.0),
//     )));

//     world.push(Arc::new(XZRect::new(
//         123.0,
//         147.0,
//         423.0,
//         412.0,
//         554.0,
//         light.clone(),
//     )));

//     world.push(Arc::new(Sphere::new(
//         Vec3::new(260.0, 150.0, 45.0),
//         50.0,
//         glass.clone(),
//     )));
//     world.push(Arc::new(Sphere::new(
//         Vec3::new(0.0, 150.0, 145.0),
//         50.0,
//         Arc::new(Metal::new(
//             Arc::new(ConstantTexture::new(Vec3::new(0.8, 0.8, 0.9))),
//             10.0,
//         )),
//     )));

//     let boundary = Arc::new(Sphere::new(
//         Vec3::new(360.0, 150.0, 145.0),
//         70.0,
//         glass.clone(),
//     ));
//     world.push(boundary.clone());
//     world.push(Arc::new(ConstantMedium::new(
//         boundary.clone(),
//         0.2,
//         Arc::new(ConstantTexture::new(Vec3::new(0.2, 0.4, 0.9))),
//     )));

//     let atmosphere = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5000.0, glass.clone()));
//     world.push(Arc::new(ConstantMedium::new(
//         atmosphere,
//         0.00001,
//         Arc::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
//     )));

//     let (image_data, width, height) = image::read_png("earthmap.png");
//     let texture = ImageTexture::new(&image_data, width, height);
//     world.push(Arc::new(Sphere::new(
//         Vec3::new(400.0, 250.0, 400.0),
//         100.0,
//         Arc::new(Lambertian::new(Arc::new(texture))),
//     )));

//     BVHNode::new(world.list.as_mut_slice(), 0.0, 1.0)
// }

fn threaded_color(r: &Ray, world: &Arc<Hitable>, depth: i32) -> Vec3 {
    match world.hit(r, 0.001, std::f32::MAX) {
        Some(rec) => {
            let emitted = rec.mat.emit(rec.u, rec.v, &rec.p);
            if depth < 50 {
                match rec.mat.scatter(r, &rec) {
                    Some((attenuation, scattered)) => {
                        emitted + attenuation * threaded_color(&scattered, world, depth + 1)
                    }
                    None => emitted,
                }
            } else {
                emitted
            }
        }
        None => Vec3::new(0.0, 0.0, 0.0),
    }
}

fn partial_render(
    camera: Arc<camera::Camera>,
    world: Arc<Hitable>,
    start_x: usize,
    start_y: usize,
    width: usize,
    height: usize,
    n_samples: usize,
    picture_width: usize,
    picture_height: usize,
) -> Vec<u8> {
    let mut data = Vec::with_capacity(width * height * 3);

    let mut rng = rand::thread_rng();
    for y in (start_y..(start_y + height)).rev() {
        for x in start_x..(start_x + width) {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..n_samples {
                let r1: f32 = rng.gen();
                let r2: f32 = rng.gen();
                let u = (x as f32 + r1) / (picture_width as f32);
                let v = (y as f32 + r2) / (picture_height as f32);
                let r = camera.get_ray(u, v);
                col += threaded_color(&r, &world, 0);
            }
            col = col / (n_samples as f32);

            if col.0 > 1.0 {
                data.push(255);
            } else {
                data.push((255.99 * col.0.sqrt()) as u8);
            }

            if col.1 > 1.0 {
                data.push(255);
            } else {
                data.push((255.99 * col.1.sqrt()) as u8);
            }

            if col.2 > 1.0 {
                data.push(255);
            } else {
                data.push((255.99 * col.2.sqrt()) as u8);
            }
        }
    }
    data
}

const BLOCK: usize = 16;

fn main() {
    let start = SystemTime::now();

    let width = 256;
    let height = 256;
    let n_samples = 500;
    let mut data: Vec<u8> = Vec::with_capacity(width * height * 3);
    for _ in 0..(width * height * 3) {
        data.push(0);
    }

    let args: std::vec::Vec<String> = env::args().collect();
    let scene_file = &args[1];
    let output_file = &args[2];

    let active_scene = scene::Scene::new(scene_file);

    let camera = Arc::new(active_scene.camera);
    let mut _world = HitableList::new();
    for shape in active_scene.shapes {
        _world.push(shape.clone());
    }
    let world = Arc::new(BVHNode::new(_world.list.as_mut_slice(), 0.0, 1.0));

    // data = partial_render(camera, world, 0, 0, width, height, n_samples, width, height);
    let pool = ThreadPool::new(num_cpus::get());
    let (sender, receiver) = mpsc::channel();
    for j in (0..height).step_by(BLOCK) {
        for i in (0..width).step_by(BLOCK) {
            let sender = sender.clone();
            let camera_copy = camera.clone();
            let world_copy = world.clone();
            pool.execute(move || {
                let rendered = partial_render(
                    camera_copy,
                    world_copy,
                    i,
                    j,
                    BLOCK,
                    BLOCK,
                    n_samples,
                    width,
                    height,
                );
                sender.send((i, j, rendered)).unwrap();
            });
        }
    }
    for j in (0..height).step_by(BLOCK) {
        for i in (0..width).step_by(BLOCK) {
            let (start_x, start_y, partial_data) = receiver.recv().unwrap();
            let point = (height - start_y - BLOCK) * width * 3 + start_x * 3;
            for y in (0..BLOCK).rev() {
                for x in 0..BLOCK {
                    data[point + y * width * 3 + x * 3] = partial_data[y * BLOCK * 3 + x * 3];
                    data[point + y * width * 3 + x * 3 + 1] =
                        partial_data[y * BLOCK * 3 + x * 3 + 1];
                    data[point + y * width * 3 + x * 3 + 2] =
                        partial_data[y * BLOCK * 3 + x * 3 + 2];
                }
            }
        }
    }

    image::write_to_png(
        output_file,
        data.as_mut_slice(),
        width as u32,
        height as u32,
    );
    println!(
        "Render time: {:?}",
        SystemTime::now().duration_since(start).unwrap()
    );
}
