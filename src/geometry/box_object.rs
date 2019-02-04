use super::aabb::AABB;
use super::flip_normals::FlipNormals;
use super::hitable::{HitRecord, Hitable};
use super::hitable_list::HitableList;
use super::rect::{XYRect, XZRect, YZRect};
use crate::materials::Material;

use crate::linalg::{Ray, Vec3};

use std::sync::Arc;

pub struct BoxObject {
    p_min: Vec3,
    p_max: Vec3,
    faces: HitableList,
}

impl BoxObject {
    pub fn new(p0: Vec3, p1: Vec3, material: Arc<Material>) -> Self {
        let mut faces = HitableList::new();
        faces.push(Arc::new(XYRect::new(
            p0.0,
            p0.1,
            p1.0,
            p1.1,
            p1.2,
            material.clone(),
        )));
        faces.push(Arc::new(FlipNormals::new(Arc::new(XYRect::new(
            p0.0,
            p0.1,
            p1.0,
            p1.1,
            p0.2,
            material.clone(),
        )))));
        faces.push(Arc::new(XZRect::new(
            p0.0,
            p0.2,
            p1.0,
            p1.2,
            p1.1,
            material.clone(),
        )));
        faces.push(Arc::new(FlipNormals::new(Arc::new(XZRect::new(
            p0.0,
            p0.2,
            p1.0,
            p1.2,
            p0.1,
            material.clone(),
        )))));
        faces.push(Arc::new(YZRect::new(
            p0.1,
            p0.2,
            p1.1,
            p1.2,
            p1.0,
            material.clone(),
        )));
        faces.push(Arc::new(FlipNormals::new(Arc::new(YZRect::new(
            p0.1,
            p0.2,
            p1.1,
            p1.2,
            p0.0,
            material.clone(),
        )))));

        Self {
            p_min: p0,
            p_max: p1,
            faces: faces,
        }
    }
}

impl Hitable for BoxObject {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.faces.hit(r, t_min, t_max)
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB::new(&self.p_min, &self.p_max))
    }
}
