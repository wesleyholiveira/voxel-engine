use fastnoise2::{generator::prelude::*, SafeNode};

use crate::terrain::constants::noise::*;

pub struct TerrainNoise {
    noise: GeneratorWrapper<SafeNode>,
    seed: i32,
}

impl TerrainNoise {
    pub fn new(seed: i32) -> Self {
        // Using an FBm (Fractal Brownian Motion) fractal
        // This combines multiple layers (octaves) of Perlin noise automatically
        let noise = perlin()
            .with_feature_scale(PERLIN_SCALE)
            .fbm(FBM_GAIN, FBM_WEIGHTED_STRENGTH, FBM_OCTAVES, FBM_LACUNARITY)
            .build();

        Self { noise, seed }
    }

    pub fn sample_2d(&self, x: f32, z: f32) -> f32 {
        // gen_single_2d returns a value in the range [-1.0, 1.0]
        self.noise.gen_single_2d(x, z, self.seed)
    }
}
