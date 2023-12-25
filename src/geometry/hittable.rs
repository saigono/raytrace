use super::aabb::AABB;
use crate::linalg::{Ray, Vec3};
use crate::materials::Material;

use std::sync::Arc;

pub struct HitRecord {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(t: f32, u: f32, v: f32, p: Vec3, normal: Vec3, mat: Arc<dyn Material>) -> Self {
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

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}
