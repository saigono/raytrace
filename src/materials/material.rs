use crate::geometry::hittable::HitRecord;
use crate::linalg::{Ray, Vec3};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
    fn emit(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}
