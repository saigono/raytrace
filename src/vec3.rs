#[derive(Copy, Clone)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn new(a: f32, b: f32, c: f32) -> Vec3 {
        Vec3(a, b, c)
    }

    pub fn length(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn unit(vector: &Vec3) -> Vec3 {
        vector / vector.length()
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl std::ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl std::ops::Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self * other.0, self * other.1, self * other.2)
    }
}
