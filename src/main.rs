mod camera;
mod geometry;
mod image;
mod linalg;
mod materials;
mod random;
mod renderer;
mod scene;
mod textures;
mod ui;

use geometry::bvh_node::BVHNode;
use geometry::hittable::Hittable;
use geometry::hittable_list::HittableList;
use linalg::{Ray, Vec3};

use std::env;
use std::sync::{mpsc, Arc};
use std::time::SystemTime;

use rand::Rng;
use threadpool::ThreadPool;

use eframe::egui;
use egui_extras;
use ui::RaytraceUI;

use crate::renderer::render;

extern crate json;
extern crate num_cpus;

const BLOCK: usize = 16;

fn run_ui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Raytrace",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<RaytraceUI>::default()
        }),
    )
}

fn main() {
    // run_ui();
    let start = SystemTime::now();

    let width = 512;
    let height = 512;
    let n_samples = 250;

    let args: std::vec::Vec<String> = env::args().collect();
    let scene_file = &args[1];
    let output_file = &args[2];

    let mut data: Vec<u8> = render(width, height, n_samples, BLOCK, scene_file);

    println!(
        "Render time: {:?}",
        SystemTime::now().duration_since(start).unwrap()
    );

    image::write_to_png(
        output_file,
        data.as_mut_slice(),
        width as u32,
        height as u32,
    );
}
