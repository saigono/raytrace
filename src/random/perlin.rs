use crate::linalg::Vec3;
use rand::Rng;

const PERLIN_SIZE: usize = 256;
const PERLIN_MASK: usize = PERLIN_SIZE - 1;

pub struct Perlin {
    perm_x: [usize; PERLIN_SIZE],
    perm_y: [usize; PERLIN_SIZE],
    perm_z: [usize; PERLIN_SIZE],
    grid: [Vec3; PERLIN_SIZE]
}

fn smooth(t: f32) -> f32 {
    6.0*t.powf(5.0) - 15.0*t.powf(4.0) + 10.0 * t.powf(3.0)
}

fn lerp(lo: f32, hi: f32, t: f32) -> f32 {
    t * hi + (1.0 - t) * lo
}

fn smooth_lerp(lo: f32, hi: f32, t: f32) -> f32 {
    let smooth_t = smooth(t);
    lerp(lo, hi, smooth_t)
}

fn generate_permutation() -> [usize; PERLIN_SIZE] {
    let mut p = [0_usize; PERLIN_SIZE];
    let mut rng = rand::thread_rng();

    for i in 0..PERLIN_SIZE {
        p[i] = i;
    }

    for i in 0..PERLIN_SIZE {
        let j: usize = rng.gen_range(0, 255);
        p[i] = p[j] + p[i];
        p[j] = p[i] - p[j];
        p[i] = p[i] - p[j];
    }
    p
}

impl Perlin {
    pub fn new() -> Self {
        let mut g = [Vec3::new(0.0, 0.0, 0.0); PERLIN_SIZE];
        let mut rng = rand::thread_rng();
        for i in 0..PERLIN_SIZE {
            g[i] = Vec3::unit(&Vec3::new(2.0 * rng.gen::<f32>() - 1.0, 2.0 * rng.gen::<f32>() - 1.0, 2.0 * rng.gen::<f32>() - 1.0));
//            g[i] = Vec3::unit(&Vec3::new(rng.gen(), rng.gen(), rng.gen()));
        }
        Self {
            grid: g,
            perm_x: generate_permutation(),
            perm_y: generate_permutation(),
            perm_z: generate_permutation(),
        }
    }

    fn hash(&self, x: usize, y: usize, z: usize) -> usize {
        self.perm_x[x] ^ self.perm_y[y] ^ self.perm_z[z]
    }

    pub fn noise(&self, p: &Vec3) -> f32 {
        let xi = (p.0.floor() as usize) & PERLIN_MASK;
        let xi1 = (xi + 1) & PERLIN_MASK;
        let fx = p.0 - p.0.floor(); // this is important to not use .fract because it can be negative
        let fxt = fx - 1.0;

        let yi = (p.1.floor() as usize) & PERLIN_MASK;
        let yi1 = (yi + 1) & PERLIN_MASK;
        let fy = p.1 - p.1.floor();
        let fyt = fy - 1.0;

        let zi = (p.2.floor() as usize) & PERLIN_MASK;
        let zi1 = (zi + 1) & PERLIN_MASK;
        let fz = p.2 - p.2.floor();
        let fzt = fz - 1.0;

        if fx < 0.0 || fy < 0.0 || fz < 0.0 {
            println!("{} {} {}", fx, fy, fz);
        }

        let c000 = self.grid[self.hash(xi, yi, zi)];
        let c001 = self.grid[self.hash(xi, yi, zi1)];
        let c010 = self.grid[self.hash(xi, yi1, zi)];
        let c011 = self.grid[self.hash(xi, yi1, zi1)];
        let c100 = self.grid[self.hash(xi1, yi, zi)];
        let c101 = self.grid[self.hash(xi1, yi, zi1)];
        let c110 = self.grid[self.hash(xi1, yi1, zi)];
        let c111 = self.grid[self.hash(xi1, yi1, zi1)];

        let p000 = Vec3::new(fx, fy, fz);
        let p100 = Vec3::new(fxt, fy, fz);
        let p010 = Vec3::new(fx, fyt, fz);
        let p110 = Vec3::new(fxt, fyt, fz);
        let p001 = Vec3::new(fx, fy, fzt);
        let p101 = Vec3::new(fxt, fy, fzt);
        let p011 = Vec3::new(fx, fyt, fzt);
        let p111 = Vec3::new(fxt, fyt, fzt);



        let r0 = smooth_lerp(Vec3::dot(&p000, &c000), Vec3::dot(&p100, &c100), fx);
        let r1 = smooth_lerp(Vec3::dot(&p010, &c010), Vec3::dot(&p110, &c110), fx);
        let r2 = smooth_lerp(Vec3::dot(&p001, &c001), Vec3::dot(&p101, &c101), fx);
        let r3 = smooth_lerp(Vec3::dot(&p011, &c011), Vec3::dot(&p111, &c111), fx);

        let r4 = smooth_lerp(r0, r1, fy);
        let r5 = smooth_lerp(r2, r3, fy);

        let r6 = smooth_lerp(r4, r5, fz);

        // 3D perlin noise will range from [-sqrt(3)/2; +sqrt(3)/2].
        // 1 / sqrt(3) = 0.57
        r6 * 0.57 + 0.5
    }
}
