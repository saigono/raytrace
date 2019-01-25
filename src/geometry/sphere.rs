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

pub struct MovingSphere {
    pub center_start: Vec3,
    pub center_end: Vec3,
    pub time_start: f32,
    pub time_end: f32,
    pub radius: f32,
    pub material: Rc<Material>,
}

impl MovingSphere {
    pub fn new(
        center_start: Vec3,
        center_end: Vec3,
        time_start: f32,
        time_end: f32,
        radius: f32,
        material: Rc<Material>,
    ) -> Self {
        Self {
            center_start: center_start,
            center_end: center_end,
            time_start: time_start,
            time_end: time_end,
            radius: radius,
            material: material,
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.center_start
            + (time - self.time_start) / (self.time_end - self.time_start)
                * (self.center_end - self.center_start)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = &r.origin - &self.center(r.time);
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
                    (p - self.center(r.time)) / self.radius,
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
                    (p - self.center(r.time)) / self.radius,
                    self.material.clone(),
                );
                return Some(rec);
            }
        }
        None
    }
}
