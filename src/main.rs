mod camera;
mod geometry;
mod image;
mod linalg;

use geometry::{Hitable, HitableList, Lambertian, Metal, Sphere};
use linalg::{Ray, Vec3};
use rand::Rng;
use std::rc::Rc;

fn color(r: &Ray, world: &Hitable, depth: i32) -> Vec3 {
    match world.hit(r, 0.001, 10000.0) {
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

fn main() {
    let mut data: Vec<u8> = Vec::new();
    let width = 400;
    let height = 200;
    let n_samples = 100;

    let camera = camera::Camera::new();

    let mut world = HitableList::new();
    world.push(Sphere::new(
        Vec3(0.0, 0.0, -1.0),
        0.5,
        Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
    ));
    world.push(Sphere::new(
        Vec3(0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
    ));
    world.push(Sphere::new(
        Vec3(1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2))),
    ));

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
            // let col = v * Vec3(1.0, 1.0, 1.0);

            let r = (255.99 * col.0) as u8;
            let g = (255.99 * col.1) as u8;
            let b = (255.99 * col.2) as u8;
            data.push(r);
            data.push(g);
            data.push(b);
        }
    }
    image::write_to_png("out/render.png", data.as_mut_slice(), width, height);
    // write_to_ppm("out/render.ppm", data.as_mut_slice(), width, height);
}
