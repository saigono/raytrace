use super::aabb::{surrounding_box, AABB};
use super::hittable::{HitRecord, Hittable};
use crate::linalg::Ray;

use std::sync::Arc;

pub struct HittableList {
    pub list: std::vec::Vec<Arc<dyn Hittable>>,
    pub size: usize,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            size: 0,
        }
    }

    pub fn push(&mut self, hitable: Arc<dyn Hittable>) -> &mut Self {
        self.list.push(hitable.clone());
        self.size += 1;
        self
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;

        for hitable in self.list.iter() {
            match hitable.hit(r, t_min, closest_so_far) {
                Some(x) => {
                    closest_so_far = x.t;
                    rec = Some(x);
                }
                None => {}
            }
        }

        rec
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.list.len() < 1 {
            return None;
        }
        let mut _box: AABB;
        match self.list[0].bounding_box(t0, t1) {
            Some(b) => {
                _box = b;
            }
            None => {
                return None;
            }
        }
        for i in 1..self.list.len() {
            match self.list[i].bounding_box(t0, t1) {
                Some(b) => {
                    _box = surrounding_box(&b, &_box);
                }
                None => {
                    return None;
                }
            }
        }
        Some(_box)
    }
}
