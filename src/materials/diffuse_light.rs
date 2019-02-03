use super::material::Material;
use crate::geometry::hitable::HitRecord;
use crate::linalg::{Ray, Vec3};
use crate::textures::Texture;

use std::rc::Rc;

pub struct DiffuseLight {
    emit_tex: Rc<Texture>,
}

impl DiffuseLight {
    pub fn new(emit_tex: Rc<Texture>) -> Self {
        Self { emit_tex: emit_tex }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }

    fn emit(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.emit_tex.value(u, v, p)
    }
}
