use super::material::Material;
use crate::geometry::hitable::HitRecord;
use crate::linalg::{Ray, Vec3};
use crate::random::utils::random_in_unit_sphere;
use crate::textures::Texture;

use std::sync::Arc;

pub struct Lambertian {
    albedo: Arc<Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<Texture>) -> Self {
        Self { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p, r_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        Some((attenuation, scattered))
    }
}
