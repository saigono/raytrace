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

pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize
}

impl ImageTexture {
    pub fn new(data: &[u8], width: usize, height: usize) -> Self {
        Self {
            data: Vec::from(data),
            width: width,
            height: height
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let mut i = (u * (self.width as f32)) as i32;
        let mut j = ((1.0 - v) * (self.height as f32) - 0.001) as i32;
        if i < 0 {
            i = 0;
        }
        if j < 0 {
            j = 0;
        }

        if i >= (self.width as i32) {
            i = self.width as i32 - 1;
        }
        if j >= (self.height as i32) {
            j = self.height as i32 - 1;
        }
        Vec3::new(
            (self.data[4 * (i as usize) + 4 * self.width * (j as usize)] as f32) / 255.0,
            (self.data[4 * (i as usize) + 4 * self.width * (j as usize) + 1] as f32) / 255.0,
            (self.data[4 * (i as usize) + 4 * self.width * (j as usize) + 2] as f32) / 255.0,
        )
    }
}