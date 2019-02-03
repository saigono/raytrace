use super::texture::Texture;
use crate::linalg::Vec3;

pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl ImageTexture {
    pub fn new(data: &[u8], width: usize, height: usize) -> Self {
        Self {
            data: Vec::from(data),
            width: width,
            height: height,
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
