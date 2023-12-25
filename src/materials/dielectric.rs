use super::material::Material;
use super::utils::{reflect, refract};
use crate::geometry::hittable::HitRecord;
use crate::linalg::{Ray, Vec3};

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Self { ref_idx: ref_idx }
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powf(2.0);
    r0 + (1.0 - r0 * r0) * (1.0 - cosine).powf(5.0)
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let outward_normal: Vec3;
        let reflected = reflect(&r_in.direction, &rec.normal);

        let ni_over_nt: f32;
        let attenuation = Vec3(1.0, 1.0, 1.0);

        let reflect_prob: f32;
        let cosine: f32;

        if Vec3::dot(&r_in.direction, &rec.normal) > 0.0 {
            outward_normal = -1.0 * rec.normal;
            ni_over_nt = self.ref_idx;
            cosine =
                self.ref_idx * Vec3::dot(&r_in.direction, &rec.normal) / r_in.direction.length();
        } else {
            outward_normal = 1.0 * rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -Vec3::dot(&r_in.direction, &rec.normal) / r_in.direction.length();
        }
        match refract(&r_in.direction, &outward_normal, ni_over_nt) {
            Some(refracted) => {
                reflect_prob = schlick(cosine, self.ref_idx);
                if reflect_prob < rand::random::<f32>() {
                    return Some((attenuation, Ray::new(rec.p, refracted, r_in.time)));
                }
            }
            None => {}
        }
        Some((attenuation, Ray::new(rec.p, reflected, r_in.time)))
    }
}
