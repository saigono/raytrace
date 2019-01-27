mod camera;
mod geometry;
mod image;
mod linalg;

use geometry::bvh_node::BVHNode;
use geometry::texture::{CheckerTexture, ConstantTexture};
use geometry::{Dielectric, Hitable, HitableList, Lambertian, Metal, MovingSphere, Sphere};
use linalg::{Ray, Vec3};
use rand::Rng;
use std::rc::Rc;

fn color(r: &Ray, world: &Hitable, depth: i32) -> Vec3 {
    match world.hit(r, 0.001, std::f32::MAX) {
        Some(rec) => {
            if depth < 50 {
                match rec.mat.scatter(r, &rec) {
                    Some((attenuation, scattered)) => {
                        attenuation * color(&scattered, world, depth + 1)
                    }
                    None => Vec3(0.0, 0.0, 0.0),
                }
            } else {
                Vec3(0.0, 0.0, 0.0)
            }
        }
        None => {
            let unit_direction = Vec3::unit(&r.direction);
            let t = 0.5 * (unit_direction.1 + 1.0);
            (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
        }
    }
}
fn random_scene() -> BVHNode {
    let mut world = HitableList::new();
    world.push(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(Rc::new(CheckerTexture::new(
            Rc::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
            Rc::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
        )))),
    )));
    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let off_x: f32 = rng.gen();
            let off_z: f32 = rng.gen();
            let center = Vec3::new((a as f32) + 0.9 * off_x, 0.2, (b as f32) + 0.9 * off_z);
            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.push(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        ))))),
                    )));
                } else if choose_mat < 0.95 {
                    world.push(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                            ),
                            0.5 * (1.0 + rng.gen::<f32>()),
                        )),
                    )));
                } else {
                    world.push(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    world.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.push(Rc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
            0.4, 0.2, 0.1,
        ))))),
    )));
    world.push(Rc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));
    BVHNode::new(world.list.as_mut_slice(), 0.0, 1.0)
}

fn main() {
    let mut data: Vec<u8> = Vec::new();
    let width = 300;
    let height = 150;
    let n_samples = 100;

    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;

    let camera = camera::Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (width as f32) / (height as f32),
        0.1,
        dist_to_focus,
        0.0,
        1.0,
    );

    let world = random_scene();
    // world.push(Sphere::new(
    //     Vec3(0.0, 0.0, -1.0),
    //     0.5,
    //     Rc::new(Lambertian::new(Vec3::new(0.1, 0.1, 0.9))),
    // ));
    // world.push(Sphere::new(
    //     Vec3(0.0, -100.5, -1.0),
    //     100.0,
    //     Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    // ));
    // world.push(Sphere::new(
    //     Vec3(1.0, 0.0, -1.0),
    //     0.5,
    //     Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)),
    // ));
    // world.push(Sphere::new(
    //     Vec3(-1.0, 0.0, -1.0),
    //     0.5,
    //     Rc::new(Dielectric::new(1.5)),
    // ));
    // world.push(Sphere::new(
    //     Vec3(-1.0, 0.0, -1.0),
    //     -0.45,
    //     Rc::new(Dielectric::new(1.5)),
    // ));
    // let R = std::f32::consts::FRAC_PI_4.cos();
    // world.push(Sphere::new(
    //     Vec3::new(-R, 0.0, -1.0),
    //     R,
    //     Rc::new(Lambertian::new(Vec3::new(1.0, 0.0, 0.0))),
    // ));
    // world.push(Sphere::new(
    //     Vec3::new(R, 0.0, -1.0),
    //     R,
    //     Rc::new(Lambertian::new(Vec3::new(0.0, 0.0, 1.0))),
    // ));

    let mut rng = rand::thread_rng();

    for x in (0..height).rev() {
        for y in 0..width {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for _s in 0..n_samples {
                let r1: f32 = rng.gen();
                let r2: f32 = rng.gen();
                let u = (y as f32 + r1) / (width as f32);
                let v = (x as f32 + r2) / (height as f32);
                let r = camera.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            col = col / (n_samples as f32);

            let r = (255.99 * col.0.sqrt()) as u8;
            let g = (255.99 * col.1.sqrt()) as u8;
            let b = (255.99 * col.2.sqrt()) as u8;
            data.push(r);
            data.push(g);
            data.push(b);
        }
    }
    image::write_to_png("out/render.png", data.as_mut_slice(), width, height);
    // write_to_ppm("out/render.ppm", data.as_mut_slice(), width, height);
}
