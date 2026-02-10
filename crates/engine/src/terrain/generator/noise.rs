use noise::{NoiseFn, Perlin};

use crate::terrain::constants::noise::*;

#[derive(Clone)]
pub struct TerrainNoise {
    noise: Perlin,
}

impl TerrainNoise {
    pub fn new(seed: i32) -> Self {
        let noise = Perlin::new(seed as u32);

        Self { noise }
    }

    pub fn sample_2d(&self, x: f32, z: f32) -> f32 {
        // Perlin noise returns a value in the range [-1.0, 1.0]
        (self.noise.get([
            x as f64 * PERLIN_SCALE as f64,
            z as f64 * PERLIN_SCALE as f64,
        ]) as f32
            + 1.)
            / 2.
    }
}
