use super::material::Material;
use crate::geometry::hitable::HitRecord;
use crate::linalg::{Ray, Vec3};
use crate::random::utils::random_in_unit_sphere;
use crate::textures::Texture;

use std::sync::Arc;

pub struct Isotropic {
    albedo: Arc<Texture>,
}

impl Isotropic {
    pub fn new(albedo: Arc<Texture>) -> Self {
        Self { albedo: albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        Some((
            self.albedo.value(rec.u, rec.v, &rec.p),
            Ray::new(rec.p, random_in_unit_sphere(), r_in.time),
        ))
    }
}