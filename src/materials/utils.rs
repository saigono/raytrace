use crate::linalg::Vec3;

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(v, n) * (*n)
}

pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = Vec3::unit(v);
    let dt = Vec3::dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt())
    } else {
        None
    }
}
