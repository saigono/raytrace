use super::aabb::{surrounding_box, AABB};
use super::hitable::{HitRecord, Hitable};
use crate::linalg::Ray;

use std::rc::Rc;

pub struct HitableList {
    pub list: std::vec::Vec<Rc<Hitable>>,
    pub size: usize,
}

impl HitableList {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            size: 0,
        }
    }

    pub fn push(&mut self, hitable: Rc<Hitable>) -> &mut Self {
        self.list.push(hitable.clone());
        self.size += 1;
        self
    }
}

impl Hitable for HitableList {
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
