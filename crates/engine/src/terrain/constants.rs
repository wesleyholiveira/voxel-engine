pub const CHUNK_WIDTH: i32 = 16;
pub const CHUNK_HEIGHT: i32 = 128;
pub const CHUNK_DEPTH: i32 = 16;

pub mod noise {
    pub const PERLIN_SCALE: f32 = 0.05;
    pub const FBM_GAIN: f32 = 0.5;
    pub const FBM_WEIGHTED_STRENGTH: f32 = 0.5;
    pub const FBM_OCTAVES: i32 = 5;
    pub const FBM_LACUNARITY: f32 = 2.0;
}
