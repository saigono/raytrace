mod camera;
mod geometry;
mod image;
mod linalg;
mod materials;
mod random;
mod textures;

use geometry::box_object::BoxObject;
use geometry::bvh_node::BVHNode;
use geometry::constant_medium::ConstantMedium;
use geometry::flip_normals::FlipNormals;
use geometry::hitable::Hitable;
use geometry::hitable_list::HitableList;
use materials::{Dielectric, DiffuseLight, Lambertian, Metal};
use geometry::rect::{XYRect, XZRect, YZRect};
use geometry::sphere::Sphere;
use geometry::transform::{Translation, YRotation};
use linalg::{Ray, Vec3};
use rand::Rng;
use std::rc::Rc;
use textures::{CheckerTexture, ConstantTexture, ImageTexture, PerlinTexture};

fn color(r: &Ray, world: &Hitable, depth: i32) -> Vec3 {
    match world.hit(r, 0.001, std::f32::MAX) {
        Some(rec) => {
            let emitted = rec.mat.emit(rec.u, rec.v, &rec.p);
            if depth < 50 {
                match rec.mat.scatter(r, &rec) {
                    Some((attenuation, scattered)) => {
                        emitted + attenuation * color(&scattered, world, depth + 1)
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

fn perlin_scene() -> BVHNode {
    let perlin_text = Rc::new(PerlinTexture::new());
    let material = Rc::new(Lambertian::new(perlin_text));
    let mut world = HitableList::new();
    world.push(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        material,
    )));
    world.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new(Rc::new(PerlinTexture::new()))),
    )));
    BVHNode::new(world.list.as_mut_slice(), 0.0, 1.0)
}

fn earth_scene() -> BVHNode {
    let (image_data, width, height) = image::read_png("earthmap.png");
    let texture = ImageTexture::new(&image_data, width, height);
    let mut world = HitableList::new();
    world.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        Rc::new(Lambertian::new(Rc::new(texture))),
    )));
    BVHNode::new(world.list.as_mut_slice(), 0.0, 1.0)
}

fn lighted_scene() -> BVHNode {
    let mut world = HitableList::new();

    world.push(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
            0.8, 0.9, 0.6,
        ))))),
    )));

    world.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 1.4, 0.0),
        1.1,
        Rc::new(Metal::new(Vec3::new(0.9, 0.8, 0.9), 0.1)),
    )));

    world.push(Rc::new(Sphere::new(
        Vec3::new(1.5, 0.25, 1.0),
        0.25,
        Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
            0.9, 0.1, 0.1,
        ))))),
    )));

    world.push(Rc::new(Sphere::new(
        Vec3::new(1.5, 0.6, 3.0),
        0.6,
        Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
            0.2, 0.6, 0.9,
        ))))),
    )));

    world.push(Rc::new(Sphere::new(
        Vec3::new(3.5, 0.8, -1.7),
        0.8,
        Rc::new(Dielectric::new(1.5)),
    )));

    world.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 6.2, 2.0),
        2.0,
        Rc::new(DiffuseLight::new(Rc::new(ConstantTexture::new(
            1.0 * Vec3::new(1.0, 1.0, 1.0),
        )))),
    )));

    world.push(Rc::new(XYRect::new(
        -1.5,
        1.0,
        0.0,
        3.0,
        2.0,
        Rc::new(DiffuseLight::new(Rc::new(ConstantTexture::new(
            1.0 * Vec3::new(1.0, 1.0, 1.0),
        )))),
    )));

    BVHNode::new(world.list.as_mut_slice(), 0.0, 1.0)
}

fn cornell_box_scene() -> BVHNode {
    let mut world = HitableList::new();
    let red = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
        0.73, 0.73, 0.73,
    )))));
    let green = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
        0.12, 0.45, 0.15,
    )))));
    let light = Rc::new(DiffuseLight::new(Rc::new(ConstantTexture::new(
        15.0 * Vec3::new(1.0, 1.0, 1.0),
    ))));

    world.push(Rc::new(FlipNormals::new(Rc::new(YZRect::new(
        0.0, 0.0, 555.0, 555.0, 555.0, green,
    )))));
    world.push(Rc::new(YZRect::new(0.0, 0.0, 555.0, 555.0, 0.0, red)));
    world.push(Rc::new(XZRect::new(
        213.0,
        227.0,
        343.0,
        332.0,
        554.0,
        light.clone(),
    )));
    // world.push(Rc::new(FlipNormals::new(Rc::new())));
    world.push(Rc::new(FlipNormals::new(Rc::new(XZRect::new(
        0.0,
        0.0,
        555.0,
        555.0,
        555.0,
        white.clone(),
    )))));
    world.push(Rc::new(XZRect::new(
        0.0,
        0.0,
        555.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.push(Rc::new(FlipNormals::new(Rc::new(XYRect::new(
        0.0,
        0.0,
        555.0,
        555.0,
        555.0,
        white.clone(),
    )))));

    world.push(Rc::new(Translation::new(
        Rc::new(YRotation::new(
            Rc::new(BoxObject::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 165.0, 165.0),
                white.clone(),
            )),
            -18.0,
        )),
        Vec3::new(130.0, 0.0, 65.0),
    )));

    world.push(Rc::new(Translation::new(
        Rc::new(YRotation::new(
            Rc::new(BoxObject::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 330.0, 165.0),
                white.clone(),
            )),
            15.0,
        )),
        Vec3::new(265.0, 0.0, 295.0),
    )));

    BVHNode::new(world.list.as_mut_slice(), 0.0, 1.0)
}

fn cornell_smoke_scene() -> BVHNode {
    let mut world = HitableList::new();
    let red = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
        0.65, 0.05, 0.05,
    )))));
    let white = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
        0.73, 0.73, 0.73,
    )))));
    let green = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
        0.12, 0.45, 0.15,
    )))));
    let light = Rc::new(DiffuseLight::new(Rc::new(ConstantTexture::new(
        3.0 * Vec3::new(1.0, 1.0, 1.0),
    ))));

    world.push(Rc::new(FlipNormals::new(Rc::new(YZRect::new(
        0.0, 0.0, 555.0, 555.0, 555.0, green,
    )))));
    world.push(Rc::new(YZRect::new(0.0, 0.0, 555.0, 555.0, 0.0, red)));
    world.push(Rc::new(XZRect::new(
        113.0,
        127.0,
        443.0,
        432.0,
        554.0,
        light.clone(),
    )));

    world.push(Rc::new(FlipNormals::new(Rc::new(XZRect::new(
        0.0,
        0.0,
        555.0,
        555.0,
        555.0,
        white.clone(),
    )))));
    world.push(Rc::new(XZRect::new(
        0.0,
        0.0,
        555.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.push(Rc::new(FlipNormals::new(Rc::new(XYRect::new(
        0.0,
        0.0,
        555.0,
        555.0,
        555.0,
        white.clone(),
    )))));

    let box1 = Rc::new(Translation::new(
        Rc::new(YRotation::new(
            Rc::new(BoxObject::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 165.0, 165.0),
                white.clone(),
            )),
            -18.0,
        )),
        Vec3::new(130.0, 0.0, 65.0),
    ));

    let box2 = Rc::new(Translation::new(
        Rc::new(YRotation::new(
            Rc::new(BoxObject::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 330.0, 165.0),
                white.clone(),
            )),
            15.0,
        )),
        Vec3::new(265.0, 0.0, 295.0),
    ));

    world.push(Rc::new(ConstantMedium::new(
        box1,
        0.01,
        Rc::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
    )));
    world.push(Rc::new(ConstantMedium::new(
        box2,
        0.01,
        Rc::new(ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0))),
    )));

    BVHNode::new(world.list.as_mut_slice(), 0.0, 1.0)
}

fn complex_scene() -> BVHNode {
    let mut world = HitableList::new();
    let mut boxlist = HitableList::new();
    let mut boxlist2 = HitableList::new();

    let white = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
        0.73, 0.73, 0.73,
    )))));
    let ground = Rc::new(Lambertian::new(Rc::new(ConstantTexture::new(Vec3::new(
        0.48, 0.83, 0.53,
    )))));
    let light = Rc::new(DiffuseLight::new(Rc::new(ConstantTexture::new(
        5.0 * Vec3::new(1.0, 1.0, 1.0),
    ))));
    let glass = Rc::new(Dielectric::new(1.5));

    let mut rng = rand::thread_rng();

    for i in 0..20 {
        for j in 0..20 {
            let w = 100.0;
            let x0 = -1000.0 + (i as f32) * w;
            let z0 = -1000.0 + (j as f32) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let z1 = z0 + w;
            let y1 = 100.0 * (rng.gen::<f32>() + 0.01);
            boxlist.push(Rc::new(BoxObject::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    world.push(Rc::new(BVHNode::new(boxlist.list.as_mut_slice(), 0.0, 1.0)));

    for j in 0..1000 {
        boxlist2.push(Rc::new(Sphere::new(
            Vec3::new(
                165.0 * rng.gen::<f32>(),
                165.0 * rng.gen::<f32>(),
                165.0 * rng.gen::<f32>(),
            ),
            10.0,
            white.clone(),
        )));
    }
    world.push(Rc::new(Translation::new(
        Rc::new(YRotation::new(
            Rc::new(BVHNode::new(boxlist2.list.as_mut_slice(), 0.0, 1.0)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    world.push(Rc::new(XZRect::new(
        123.0,
        147.0,
        423.0,
        412.0,
        554.0,
        light.clone(),
    )));

    world.push(Rc::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        glass.clone(),
    )));
    world.push(Rc::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 10.0)),
    )));

    let boundary = Rc::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        glass.clone(),
    ));
    world.push(boundary.clone());
    world.push(Rc::new(ConstantMedium::new(
        boundary.clone(),
        0.2,
        Rc::new(ConstantTexture::new(Vec3::new(0.2, 0.4, 0.9))),
    )));

    let atmosphere = Rc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5000.0, glass.clone()));
    world.push(Rc::new(ConstantMedium::new(
        atmosphere,
        0.00001,
        Rc::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
    )));

    let (image_data, width, height) = image::read_png("earthmap.png");
    let texture = ImageTexture::new(&image_data, width, height);
    world.push(Rc::new(Sphere::new(
        Vec3::new(400.0, 250.0, 400.0),
        100.0,
        Rc::new(Lambertian::new(Rc::new(texture))),
    )));

    BVHNode::new(world.list.as_mut_slice(), 0.0, 1.0)
}

fn main() {
    let mut data: Vec<u8> = Vec::new();
    let width = 300;
    let height = 300;
    let n_samples = 100;
    // Store real textures in materials, using DiffuseLight<T: Texture> definition
    // because actually we don't really need to store references
    let look_from = Vec3(478.0, 278.0, -600.0);
    let look_at = Vec3(278.0, 278.0, 0.0);
    let dist_to_focus = 278.0;

    let camera = camera::Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        (width as f32) / (height as f32),
        0.1,
        dist_to_focus,
        0.0,
        1.0,
    );

    let world = complex_scene();

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

            // let r = (255.99 * col.0.sqrt()) as u8;
            // let g = (255.99 * col.1.sqrt()) as u8;
            // let b = (255.99 * col.2.sqrt()) as u8;
            // data.push(r);
            // data.push(g);
            // data.push(b);
        }
    }
    image::write_to_png("out/render.png", data.as_mut_slice(), width, height);
    // write_to_ppm("out/render.ppm", data.as_mut_slice(), width, height);
}
