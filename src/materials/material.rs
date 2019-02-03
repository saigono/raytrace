use crate::geometry::hitable::HitRecord;
use crate::linalg::{Ray, Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
    fn emit(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}
