pub const CHUNK_WIDTH: i32 = 16;
pub const CHUNK_HEIGHT: i32 = 128;
pub const CHUNK_DEPTH: i32 = 16;

pub mod noise {
    pub const PERLIN_SCALE: f64 = 0.01;
    pub const FBM_GAIN: f64 = 0.5;
    pub const FBM_WEIGHTED_STRENGTH: f64 = 1.5;
    pub const FBM_OCTAVES: usize = 8;
    pub const FBM_LACUNARITY: f64 = 2.0;
}
