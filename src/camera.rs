use crate::linalg::{Ray, Vec3};
use rand::Rng;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    #[allow(dead_code)]
    w: Vec3,
    lens_radius: f32,
    t_open: f32,
    t_close: f32,
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut p: Vec3;
    let mut rng = rand::thread_rng();
    loop {
        p = 2.0 * Vec3(rng.gen(), rng.gen(), 0.0) - Vec3(0.0, 0.0, 0.0);
        if Vec3::dot(&p, &p) < 1.0 {
            break;
        }
    }
    p
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
        t_open: f32,
        t_close: f32,
    ) -> Self {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = Vec3::unit(&(look_from - look_at));
        let u = Vec3::unit(&Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        Self {
            lower_left_corner: look_from
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: look_from,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture / 2.0,
            t_open: t_open,
            t_close: t_close,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.0 + self.v * rd.1;
        let time = self.t_open + rand::random::<f32>() * (self.t_close - self.t_open);
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
            time,
        )
    }
}
