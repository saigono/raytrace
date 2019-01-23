use super::hitable::HitRecord;
use crate::linalg::{Ray, Vec3};

use rand::Rng;

fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    let mut rng = rand::thread_rng();

    loop {
        p = 2.0 * Vec3(rng.gen(), rng.gen(), rng.gen()) - Vec3(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            break;
        }
    }
    p
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray);
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p);
        let attenuation = self.albedo.clone();
        (true, attenuation, scattered)
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(v, n) * (*n)
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo: albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
        let reflected = reflect(&Vec3::unit(&r_in.direction), &rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo.clone();
        (
            Vec3::dot(&scattered.direction, &rec.normal) > 0.0,
            attenuation,
            scattered,
        )
    }
}
