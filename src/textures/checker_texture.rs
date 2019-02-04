use super::texture::Texture;
use crate::linalg::Vec3;

use std::sync::Arc;

pub struct CheckerTexture {
    odd: Arc<Texture>,
    even: Arc<Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Arc<Texture>, even: Arc<Texture>) -> Self {
        Self {
            odd: odd,
            even: even,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.0).sin() * (10.0 * p.1).sin() * (10.0 * p.2).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
