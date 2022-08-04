use rand::{thread_rng, Rng};

use crate::vec3::{dot, random_vector_in_range, unit_vector, Color, Point3, Vec3};

use super::texture::Texture;

#[derive(Default)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl Texture for NoiseTexture {
    #[allow(unused_variables)]
    fn value(&self, uv: (f64, f64), p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

impl NoiseTexture {
    pub fn new(sc: f64) -> Self {
        Self {
            noise: Perlin::default(),
            scale: sc,
        }
    }
}

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut new = Self {
            ranvec: Vec::with_capacity(256),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        };

        for _ in 0..256 {
            new.ranvec
                .push(unit_vector(random_vector_in_range(-1.0, 1.0)))
        }

        new
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = vec![vec![vec![Vec3::default(), Vec3::default()]; 2]; 2];

        #[allow(clippy::needless_range_loop)]
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        perlin_interp(c, u, v, w)
    }

    fn turb(&self, p: &Point3, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

fn perlin_generate_perm() -> Vec<i32> {
    let mut result = vec![0; 256];

    for (i, item) in result.iter_mut().enumerate() {
        *item = i as i32;
    }

    for i in (1..result.len()).rev() {
        let target = thread_rng().gen_range(0..i);
        result.swap(i, target);
    }

    result
}

fn perlin_interp(c: Vec<Vec<Vec<Vec3>>>, u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;

    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let i_f = i as f64;
                let j_f = j as f64;
                let k_f = k as f64;

                let weight_v = Vec3::new(u - i_f, v - j_f, w - k_f);

                accum += (i_f * uu + (1.0 - i_f) * (1.0 - uu))
                    * (j_f * vv + (1.0 - j_f) * (1.0 - vv))
                    * (k_f * ww + (1.0 - k_f) * (1.0 - ww))
                    * dot(&c[i as usize][j as usize][k as usize], &weight_v);
            }
        }
    }

    accum
}
