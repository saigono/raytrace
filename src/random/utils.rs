use crate::linalg::Vec3;

use rand::Rng;

pub fn random_in_unit_sphere() -> Vec3 {
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
