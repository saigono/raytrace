use super::aabb::AABB;
use super::hittable::{HitRecord, Hittable};
use crate::linalg::{Ray, Vec3};

use std::sync::Arc;

pub struct Translation {
    hitable: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translation {
    #[allow(dead_code)]
    pub fn new(hitable: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Self {
            hitable: hitable,
            offset: offset,
        }
    }
}

impl Hittable for Translation {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved = Ray::new(r.origin - self.offset, r.direction, r.time);
        match self.hitable.hit(&moved, t_min, t_max) {
            Some(mut rec) => {
                rec.p += self.offset;
                Some(rec)
            }
            None => None,
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        match self.hitable.bounding_box(t0, t1) {
            Some(bbox) => Some(AABB::new(
                &(bbox.min + self.offset),
                &(bbox.max + self.offset),
            )),
            None => None,
        }
    }
}

pub struct YRotation {
    hitable: Arc<dyn Hittable>,
    cos_theta: f32,
    sin_theta: f32,
    bbox: AABB,
}

impl YRotation {
    #[allow(dead_code)]
    pub fn new(hitable: Arc<dyn Hittable>, angle: f32) -> Self {
        let radians = (std::f32::consts::PI / 180.0) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = hitable.bounding_box(0.0, 1.0).unwrap();

        let mut min = Vec3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX);
        let mut max = Vec3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f32) * bbox.max.0 + (1.0 - i as f32) * bbox.min.0;
                    let y = (j as f32) * bbox.max.1 + (1.0 - j as f32) * bbox.min.1;
                    let z = (k as f32) * bbox.max.2 + (1.0 - k as f32) * bbox.min.2;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    if new_x > max.0 {
                        max.0 = new_x;
                    }
                    if new_x < min.0 {
                        min.0 = new_x;
                    }

                    if y > max.1 {
                        max.1 = y;
                    }
                    if y < min.1 {
                        min.1 = y;
                    }

                    if new_z > max.2 {
                        max.2 = new_z;
                    }
                    if new_z < min.2 {
                        min.2 = new_z;
                    }
                }
            }
        }

        Self {
            hitable: hitable,
            cos_theta: cos_theta,
            sin_theta: sin_theta,
            bbox: AABB::new(&min, &max),
        }
    }
}

impl Hittable for YRotation {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = r.origin.clone();
        let mut direction = r.direction.clone();
        origin.0 = self.cos_theta * r.origin.0 - self.sin_theta * r.origin.2;
        origin.2 = self.sin_theta * r.origin.0 + self.cos_theta * r.origin.2;

        direction.0 = self.cos_theta * r.direction.0 - self.sin_theta * r.direction.2;
        direction.2 = self.sin_theta * r.direction.0 + self.cos_theta * r.direction.2;

        let rotated_ray = Ray::new(origin, direction, r.time);

        match self.hitable.hit(&rotated_ray, t_min, t_max) {
            Some(mut rec) => {
                let mut p = rec.p;
                let mut normal = rec.normal;
                p.0 = self.cos_theta * rec.p.0 + self.sin_theta * rec.p.2;
                p.2 = -self.sin_theta * rec.p.0 + self.cos_theta * rec.p.2;

                normal.0 = self.cos_theta * rec.normal.0 + self.sin_theta * rec.normal.2;
                normal.2 = -self.sin_theta * rec.normal.0 + self.cos_theta * rec.normal.2;

                rec.p = p;
                rec.normal = normal;
                Some(rec)
            }
            None => None,
        }
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.bbox)
    }
}
