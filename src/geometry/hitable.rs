use super::aabb::AABB;
use crate::materials::Material;
use crate::linalg::{Ray, Vec3};

use std::rc::Rc;

pub struct HitRecord {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Rc<Material>,
}

impl HitRecord {
    pub fn new(t: f32, u: f32, v: f32, p: Vec3, normal: Vec3, mat: Rc<Material>) -> Self {
        Self {
            t: t,
            u: u,
            v: v,
            p: p,
            normal: normal,
            mat: mat,
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}
