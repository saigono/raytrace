use super::hitable::{HitRecord, Hitable};
use super::material::Lambertian;
use crate::linalg::{Ray, Vec3};

pub struct HitableList {
    pub list: std::vec::Vec<Box<Hitable>>,
    pub size: usize,
}

impl HitableList {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            size: 0,
        }
    }

    pub fn push<S: Hitable + 'static>(&mut self, hitable: S) -> &mut Self {
        self.list.push(Box::new(hitable));
        self
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(
            0.0,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            Box::new(Lambertian::new(Vec3(1.0, 1.0, 1.0))),
        );
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for hitable in self.list.iter() {
            if hitable.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.t = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
            }
        }

        hit_anything
    }
}
