mod image;
mod vec3;

use vec3::Vec3;

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

fn color(r: &Ray) -> Vec3 {
    let t = hit_sphere(&Vec3(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let normal = Vec3::unit(&(r.point_at_parameter(t) - Vec3(0.0, 0.0, -1.0)));
        0.5 * Vec3(normal.0 + 1.0, normal.1 + 1.0, normal.2 + 1.0)
    } else {
        let unit_direction = Vec3::unit(&r.direction);
        let t = 0.5 * (unit_direction.1 + 1.0);
        (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
    }
}

fn dot(a: &Vec3, b: &Vec3) -> f32 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> f32 {
    let oc = &r.origin - center;
    let a = dot(&r.direction, &r.direction);
    let b = 2.0 * dot(&oc, &r.direction);
    let c = dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn main() {
    let mut data: Vec<u8> = Vec::new();
    let width = 320;
    let height = 240;
    let aspect_ratio = width as f32 / height as f32;

    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, horizontal.0 / aspect_ratio, 0.0);
    let origin = Vec3(0.0, 0.0, 0.0);

    for x in (0..height).rev() {
        for y in 0..width {
            // let r = (x % 8) * (y % 16) + (x + y) % 32;
            // let g = (y % 8) * (y % 16) + (x * y) % 64;
            // let r = x;
            // let g = y;
            // let b = 0;
            // data.push(r);
            // data.push(g);
            // data.push(b);
            let u = y as f32 / width as f32;
            let v = x as f32 / height as f32;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&r);
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
