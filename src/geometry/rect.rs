use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use crate::linalg::{Ray, Vec3};

use std::rc::Rc;

pub struct XYRect {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    k: f32,
    material: Rc<Material>,
}

impl XYRect {
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32, k: f32, material: Rc<Material>) -> Self {
        Self {
            x0: x0,
            y0: y0,
            x1: x1,
            y1: y1,
            k: k,
            material: material,
        }
    }
}

impl Hitable for XYRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.2) / r.direction.2;
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.0 + t * r.direction.0;
        let y = r.origin.1 + t * r.direction.1;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        Some(HitRecord::new(
            t,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
            r.point_at_parameter(t),
            Vec3::new(0.0, 0.0, 1.0),
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB::new(
            &Vec3::new(self.x0, self.y0, self.k - 0.0001),
            &Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}
