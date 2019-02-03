use super::material::Material;
use super::utils::reflect;
use crate::geometry::hitable::HitRecord;
use crate::linalg::{Ray, Vec3};
use crate::random::utils::random_in_unit_sphere;

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        let clamped_fuzz: f32;
        if fuzz < 1.0 {
            clamped_fuzz = fuzz;
        } else {
            clamped_fuzz = 1.0;
        }
        Self {
            albedo: albedo,
            fuzz: clamped_fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&Vec3::unit(&r_in.direction), &rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.time,
        );
        let attenuation = self.albedo.clone();
        if Vec3::dot(&scattered.direction, &rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
