use super::material::Material;
use crate::geometry::hittable::HitRecord;
use crate::linalg::{Ray, Vec3};
use crate::textures::Texture;

use std::sync::Arc;

pub struct DiffuseLight {
    emit_tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit_tex: Arc<dyn Texture>) -> Self {
        Self { emit_tex: emit_tex }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }

    fn emit(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.emit_tex.value(u, v, p)
    }
}
