use super::hitable::{HitRecord, Hitable};
use super::material::Material;
use crate::linalg::{Ray, Vec3};

use std::rc::Rc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Rc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<Material>) -> Self {
        Self {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = &r.origin - &self.center;
        let a = Vec3::dot(&r.direction, &r.direction);
        let b = Vec3::dot(&oc, &r.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let rec = HitRecord::new(
                    temp,
                    p,
                    (p - self.center) / self.radius,
                    self.material.clone(),
                );
                return Some(rec);
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let rec = HitRecord::new(
                    temp,
                    p,
                    (p - self.center) / self.radius,
                    self.material.clone(),
                );
                return Some(rec);
            }
        }
        None
    }
}
