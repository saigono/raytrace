use crate::linalg::{Ray, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

#[inline(always)]
fn fmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

#[inline(always)]
fn fmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

impl AABB {
    pub fn new(a: &Vec3, b: &Vec3) -> Self {
        Self { min: *a, max: *b }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut t0 = fmin(
            (self.min.0 - r.origin.0) / r.direction.0,
            (self.max.0 - r.origin.0) / r.direction.0,
        );
        let mut t1 = fmax(
            (self.min.0 - r.origin.0) / r.direction.0,
            (self.max.0 - r.origin.0) / r.direction.0,
        );
        let mut _tmin = fmax(t0, t_min);
        let mut _tmax = fmin(t1, t_max);
        if _tmax <= _tmin {
            return false;
        }

        t0 = fmin(
            (self.min.1 - r.origin.1) / r.direction.1,
            (self.max.1 - r.origin.1) / r.direction.1,
        );
        t1 = fmax(
            (self.min.1 - r.origin.1) / r.direction.1,
            (self.max.1 - r.origin.1) / r.direction.1,
        );
        _tmin = fmax(t0, _tmin);
        _tmax = fmin(t1, _tmax);
        if _tmax <= _tmin {
            return false;
        }

        t0 = fmin(
            (self.min.2 - r.origin.2) / r.direction.2,
            (self.max.2 - r.origin.2) / r.direction.2,
        );
        t1 = fmax(
            (self.min.2 - r.origin.2) / r.direction.2,
            (self.max.2 - r.origin.2) / r.direction.2,
        );
        _tmin = fmax(t0, _tmin);
        _tmax = fmin(t1, _tmax);
        if _tmax <= _tmin {
            return false;
        }
        true
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Vec3::new(
        fmin(box0.min.0, box1.min.0),
        fmin(box0.min.1, box1.min.1),
        fmin(box0.min.2, box1.min.2),
    );
    let big = Vec3::new(
        fmax(box0.max.0, box1.max.0),
        fmax(box0.max.1, box1.max.1),
        fmax(box0.max.2, box1.max.2),
    );
    AABB::new(&small, &big)
}
