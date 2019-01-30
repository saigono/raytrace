use super::hitable::HitRecord;
use super::texture::Texture;
use crate::linalg::{Ray, Vec3};

use rand::Rng;
use std::rc::Rc;

fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    let mut rng = rand::thread_rng();

    loop {
        p = 2.0 * Vec3(rng.gen(), rng.gen(), rng.gen()) - Vec3(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            break;
        }
    }
    p
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
    fn emit(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

pub struct Lambertian {
    albedo: Rc<Texture>,
}

impl Lambertian {
    pub fn new(albedo: Rc<Texture>) -> Self {
        Self { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p, r_in.time);
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        Some((attenuation, scattered))
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(v, n) * (*n)
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        let clamped_fuzz: f32;
        if fuzz < 1.0 {
            clamped_fuzz = fuzz;
        } else {
            clamped_fuzz = 1.0;
        }
        Self {
            albedo: albedo,
            fuzz: clamped_fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&Vec3::unit(&r_in.direction), &rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.time,
        );
        let attenuation = self.albedo.clone();
        if Vec3::dot(&scattered.direction, &rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = Vec3::unit(v);
    let dt = Vec3::dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt())
    } else {
        None
    }
}

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

pub struct DiffuseLight {
    emit_tex: Rc<Texture>
}

impl DiffuseLight {
    pub fn new(emit_tex: Rc<Texture>) -> Self {
        Self {
            emit_tex: emit_tex
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }

    fn emit(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.emit_tex.value(u, v, p)
    }
}