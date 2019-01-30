use crate::linalg::Vec3;
use crate::random::perlin::Perlin;

use std::rc::Rc;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

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

pub struct CheckerTexture {
    odd: Rc<Texture>,
    even: Rc<Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Rc<Texture>, even: Rc<Texture>) -> Self {
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
