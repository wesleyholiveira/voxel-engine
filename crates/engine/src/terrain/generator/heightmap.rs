use super::noise::TerrainNoise;

const HEIGHT_MULTIPLIER: f32 = 20.0;
const BASE_HEIGHT: f32 = 50.0;

pub fn generate_height(noise: &TerrainNoise, world_x: f32, world_z: f32) -> i32 {
    let value = noise.sample_2d(world_x, world_z);

    println!("Noise value at ({}, {}): {}", world_x, world_z, value);

    ((value * HEIGHT_MULTIPLIER) + BASE_HEIGHT) as i32
}
