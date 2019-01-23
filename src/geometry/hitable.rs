use super::material::Material;
use crate::linalg::{Ray, Vec3};

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Box<Material>,
}

impl HitRecord {
    pub fn new(t: f32, p: Vec3, normal: Vec3, mat: Box<Material>) -> HitRecord {
        HitRecord {
            t: t,
            p: p,
            normal: normal,
            mat: mat,
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}