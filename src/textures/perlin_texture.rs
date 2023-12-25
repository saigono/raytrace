use super::texture::Texture;
use crate::linalg::Vec3;
use crate::random::perlin::Perlin;

pub struct PerlinTexture {
    noise: Perlin,
}

impl PerlinTexture {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
        }
    }
}

impl Texture for PerlinTexture {
    fn value(&self, _u: f32, _v: f32, p: &Vec3) -> Vec3 {
        Vec3(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}
