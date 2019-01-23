use super::hitable::{HitRecord, Hitable};
use crate::linalg::Ray;

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
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;

        for hitable in self.list.iter() {
            match hitable.hit(r, t_min, closest_so_far) {
                Some(x) => {
                    closest_so_far = x.t;
                    rec = Some(x);
                }
                None => {}
            }
        }

        rec
    }
}
