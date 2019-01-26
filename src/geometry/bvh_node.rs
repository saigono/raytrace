use super::aabb::{surrounding_box, AABB};
use super::hitable::{HitRecord, Hitable};
use crate::linalg::Ray;

use std::rc::Rc;

pub struct BVHNode {
    bbox: AABB,
    left: Rc<Hitable>,
    right: Rc<Hitable>,
}

fn box_x_compare(a: &Rc<Hitable>, b: &Rc<Hitable>) -> std::cmp::Ordering {
    let left_box = a.bounding_box(0.0, 0.0).unwrap();
    let right_box = b.bounding_box(0.0, 0.0).unwrap();
    if left_box.min.0 - right_box.min.0 < 0.0 {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    }
}

fn box_y_compare(a: &Rc<Hitable>, b: &Rc<Hitable>) -> std::cmp::Ordering {
    let left_box = a.bounding_box(0.0, 0.0).unwrap();
    let right_box = b.bounding_box(0.0, 0.0).unwrap();
    if left_box.min.1 - right_box.min.1 < 0.0 {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    }
}

fn box_z_compare(a: &Rc<Hitable>, b: &Rc<Hitable>) -> std::cmp::Ordering {
    let left_box = a.bounding_box(0.0, 0.0).unwrap();
    let right_box = b.bounding_box(0.0, 0.0).unwrap();
    if left_box.min.2 - right_box.min.2 < 0.0 {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    }
}

impl BVHNode {
    pub fn new(list: &mut [Rc<Hitable>], time0: f32, time1: f32) -> Self {
        let axis = (3.0 * rand::random::<f32>()) as usize;
        if axis == 0 {
            list.sort_by(box_x_compare);
        } else if axis == 1 {
            list.sort_by(box_y_compare);
        } else {
            list.sort_by(box_z_compare);
        }
        let left: Rc<Hitable>;
        let right: Rc<Hitable>;

        let size = list.len();

        if size == 1 {
            left = list[0].clone();
            right = list[0].clone();
        } else if size == 2 {
            left = list[0].clone();
            right = list[1].clone();
        } else {
            let half_size = size / 2;
            left = Rc::new(BVHNode::new(&mut list[..half_size], time0, time1));
            right = Rc::new(BVHNode::new(&mut list[half_size..size], time0, time1));
        }
        let left_box = left.bounding_box(time0, time1).unwrap();
        let right_box = right.bounding_box(time0, time1).unwrap();
        Self {
            bbox: surrounding_box(&left_box, &right_box),
            left: left,
            right: right,
        }
    }
}

impl Hitable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(r, t_min, t_max) {
            // let right_rec = self.right.hit(r, t_min, t_max)?;
            match self.left.hit(r, t_min, t_max) {
                Some(left_rec) => match self.right.hit(r, t_min, t_max) {
                    Some(right_rec) => {
                        if left_rec.t < right_rec.t {
                            Some(left_rec)
                        } else {
                            Some(right_rec)
                        }
                    }
                    None => Some(left_rec),
                },
                None => match self.right.hit(r, t_min, t_max) {
                    Some(right_rec) => Some(right_rec),
                    None => None,
                },
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(self.bbox)
    }
}
