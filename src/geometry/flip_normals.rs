use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
use crate::linalg::Ray;

use std::sync::Arc;

pub struct FlipNormals {
    ptr: Arc<Hitable>,
}

impl FlipNormals {
    pub fn new(ptr: Arc<Hitable>) -> Self {
        Self { ptr: ptr }
    }
}

impl Hitable for FlipNormals {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.ptr.hit(r, t_min, t_max) {
            Some(mut rec) => {
                rec.normal = -1.0 * rec.normal;
                Some(rec)
            }
            None => None,
        }
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.ptr.bounding_box(t0, t1)
    }
}
