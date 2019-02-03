use super::texture::Texture;
use crate::linalg::Vec3;

pub struct ConstantTexture {
    color: Vec3,
}

impl ConstantTexture {
    pub fn new(color: Vec3) -> Self {
        Self { color: color }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.color
    }
}
