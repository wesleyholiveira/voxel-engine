pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_DEPTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 128;


pub mod noise {
    pub const PERLIN_SCALE: f32 = 0.1;
    pub const FBM_GAIN: f32 = 0.5;
    pub const FBM_WEIGHTED_STRENGTH: f32 = 0.5;
    pub const FBM_OCTAVES: i32 = 5;
    pub const FBM_LACUNARITY: f32 = 2.0;
}
