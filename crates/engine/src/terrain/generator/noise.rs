use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

use crate::terrain::constants::noise::*;

#[derive(Clone)]
pub struct TerrainNoise {
    noise: Fbm<Perlin>,
}

impl TerrainNoise {
    pub fn new(seed: i32) -> Self {
        let noise = Fbm::<Perlin>::new(seed as u32)
            .set_frequency(PERLIN_SCALE)
            .set_octaves(FBM_OCTAVES)
            .set_persistence(FBM_GAIN)
            .set_lacunarity(FBM_LACUNARITY);

        Self { noise }
    }

    pub fn sample_2d(&self, x: f32, z: f32) -> f32 {
        // Perlin noise returns a value in the range [-1.0, 1.0]
        (self.noise.get([
            x as f64 * FBM_WEIGHTED_STRENGTH as f64,
            z as f64 * FBM_WEIGHTED_STRENGTH as f64,
        ]) as f32
            + 1.)
            / 2.
    }
}
