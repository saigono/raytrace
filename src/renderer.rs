use std::sync::{mpsc, Arc};

use rand::Rng;

use threadpool::ThreadPool;

use crate::{
    camera,
    geometry::{bvh_node::BVHNode, hittable::Hittable, hittable_list::HittableList},
    linalg::{Ray, Vec3},
    scene,
};

pub fn threaded_color(r: &Ray, world: &Arc<dyn Hittable>, depth: i32) -> Vec3 {
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

pub fn partial_render(
    camera: Arc<camera::Camera>,
    world: Arc<dyn Hittable>,
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

pub fn render(
    width: usize,
    height: usize,
    n_samples: usize,
    block_size: usize,
    scene_file: &String,
) -> Vec<u8> {
    let mut data: Vec<u8> = vec![0; width * height * 3];

    let active_scene = scene::Scene::new(scene_file);

    let camera = Arc::new(active_scene.camera);
    let mut _world = HittableList::new();
    for shape in active_scene.shapes {
        _world.push(shape.clone());
    }
    let world = Arc::new(BVHNode::new(_world.list.as_mut_slice(), 0.0, 1.0));

    // data = partial_render(camera, world, 0, 0, width, height, n_samples, width, height);
    let pool = ThreadPool::new(num_cpus::get());
    let (sender, receiver) = mpsc::channel();
    for j in (0..height).step_by(block_size) {
        for i in (0..width).step_by(block_size) {
            let sender = sender.clone();
            let camera_copy = camera.clone();
            let world_copy = world.clone();
            pool.execute(move || {
                let rendered = partial_render(
                    camera_copy,
                    world_copy,
                    i,
                    j,
                    block_size,
                    block_size,
                    n_samples,
                    width,
                    height,
                );
                sender.send((i, j, rendered)).unwrap();
            });
        }
    }
    for _j in (0..height).step_by(block_size) {
        for _i in (0..width).step_by(block_size) {
            let (start_x, start_y, partial_data) = receiver.recv().unwrap();
            let point = (height - start_y - block_size) * width * 3 + start_x * 3;
            for y in (0..block_size).rev() {
                for x in 0..block_size {
                    data[point + y * width * 3 + x * 3] = partial_data[y * block_size * 3 + x * 3];
                    data[point + y * width * 3 + x * 3 + 1] =
                        partial_data[y * block_size * 3 + x * 3 + 1];
                    data[point + y * width * 3 + x * 3 + 2] =
                        partial_data[y * block_size * 3 + x * 3 + 2];
                }
            }
        }
    }
    return data;
}
