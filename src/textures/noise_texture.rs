use rand::{thread_rng, Rng};

use crate::{
    random_f64,
    vec3::{Color, Point3},
};

use super::texture::Texture;

#[derive(Default)]
pub struct NoiseTexture {
    noise: Perlin,
}

impl Texture for NoiseTexture {
    #[allow(unused_variables)]
    fn value(&self, uv: (f64, f64), p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}

pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut new = Self {
            ranfloat: vec![0.0; 256],
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        };

        for number in &mut new.ranfloat {
            *number = random_f64();
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

        let mut c = vec![vec![vec![0.0, 0.0]; 2]; 2];

        #[allow(clippy::needless_range_loop)]
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        trilinear_interp(c, u, v, w)
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

fn trilinear_interp(c: Vec<Vec<Vec<f64>>>, u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let i_f = i as f64;
                let j_f = j as f64;
                let k_f = k as f64;
                accum += (i_f * u + (1.0 - i_f) * (1.0 - u))
                    * (j_f * v + (1.0 - j_f) * (1.0 - v))
                    * (k_f * w + (1.0 - k_f) * (1.0 - w))
                    * c[i as usize][j as usize][k as usize];
            }
        }
    }

    accum
}
