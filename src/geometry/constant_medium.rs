use super::aabb::AABB;
use super::hitable::{HitRecord, Hitable};
use super::material::{Isotropic, Material};
use super::texture::Texture;

use crate::linalg::{Ray, Vec3};

use std::rc::Rc;

pub struct ConstantMedium {
    boundary: Rc<Hitable>,
    density: f32,
    phase_function: Rc<Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Rc<Hitable>, density: f32, phase_function: Rc<Texture>) -> Self {
        Self {
            boundary: boundary,
            density: density,
            phase_function: Rc::new(Isotropic::new(phase_function)),
        }
    }
}

impl Hitable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.boundary.hit(r, std::f32::MIN, std::f32::MAX) {
            Some(mut rec1) => match self.boundary.hit(r, rec1.t + 0.0001, std::f32::MAX) {
                Some(mut rec2) => {
                    if rec1.t < t_min {
                        rec1.t = t_min;
                    }
                    if rec2.t > t_max {
                        rec2.t = t_max;
                    }
                    if rec1.t >= rec2.t {
                        return None;
                    }
                    if rec1.t < 0.0 {
                        rec1.t = 0.0;
                    }
                    let distance_inside_boundary = (rec2.t - rec1.t) * r.direction.length();
                    let hit_distance = -(1.0 / self.density) * rand::random::<f32>().ln();
                    if hit_distance < distance_inside_boundary {
                        let t = rec1.t + hit_distance / r.direction.length();
                        Some(HitRecord::new(
                            t,
                            0.0,
                            0.0,
                            r.point_at_parameter(t),
                            Vec3::new(1.0, 0.0, 0.0),
                            self.phase_function.clone(),
                        ))
                    } else {
                        None
                    }
                }
                None => None,
            },
            None => None,
        }
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
