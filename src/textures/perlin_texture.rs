use super::texture::Texture;
use crate::linalg::Vec3;
use crate::random::perlin::Perlin;

pub struct PerlinTexture {
    noise: Perlin,
}

impl PerlinTexture {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
        }
    }
}

impl Texture for PerlinTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        Vec3(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}
