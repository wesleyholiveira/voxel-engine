use crate::terrain::{
    constants::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH},
    ecs::components::chunk::{ChunkCoords, ChunkData},
    types::Voxel,
};
use super::{heightmap::generate_height, noise::TerrainNoise};
use bevy::{math::IVec2, platform::collections::HashSet, prelude::Resource};

#[derive(Clone)]
pub struct TerrainConfig {
    pub radius: i32,
    pub threads: usize,
}

#[derive(Clone)]
pub struct TerrainSpiralState {
    pub center: IVec2,
    pub spiral_x: i32,
    pub spiral_y: i32,
    pub dx: i32,
    pub dy: i32,
}

#[derive(Resource, Clone)]
pub struct TerrainManager {
    pub config: TerrainConfig,
    pub spiral_state: TerrainSpiralState,
    pub spawned_chunks: HashSet<IVec2>,
    pub active_permits: usize,
    pub noise_handle: TerrainNoise,
}

impl TerrainManager {
    pub fn new(radius: i32, threads: usize, seed: i32) -> Self {
        Self {
            config: TerrainConfig { radius, threads },
            spiral_state: TerrainSpiralState {
                center: IVec2::ZERO,
                spiral_x: 0,
                spiral_y: 0,
                dx: 0,
                dy: -1,
            },
            spawned_chunks: HashSet::new(),
            active_permits: 0,
            noise_handle: TerrainNoise::new(seed),
        }
    }

    /// Pure generation logic - runs inside background threads
    pub fn run(&self, chunk_coord: ChunkCoords) -> ChunkData {
        let len = (CHUNK_WIDTH * CHUNK_DEPTH * CHUNK_HEIGHT) as usize;
        let mut voxels = vec![Voxel::Air; len].into_boxed_slice();
        let y_stride = (CHUNK_WIDTH * CHUNK_DEPTH) as usize;

        for lz in 0..CHUNK_DEPTH {
            let world_z = (chunk_coord.y * CHUNK_DEPTH + lz) as f32;
            let z_offset = (lz * CHUNK_WIDTH) as usize;

            for lx in 0..CHUNK_WIDTH {
                let world_x = (chunk_coord.x * CHUNK_WIDTH + lx) as f32;
                let height = generate_height(&self.noise_handle, world_x, world_z);
                let fill_to = height.clamp(0, CHUNK_HEIGHT) as usize;

                let mut current_idx = lx as usize + z_offset;
                for _ in 0..fill_to {
                    voxels[current_idx] = Voxel::Solid;
                    current_idx += y_stride;
                }
            }
        }
        ChunkData { voxels }
    }

    /// Increments the spiral and returns the absolute world coordinate
    fn next_coord(&mut self) -> IVec2 {
        let coord = IVec2::new(self.spiral_state.spiral_x, self.spiral_state.spiral_y) + self.spiral_state.center;
        
        if self.spiral_state.spiral_x == self.spiral_state.spiral_y 
            || (self.spiral_state.spiral_x < 0 && self.spiral_state.spiral_x == -self.spiral_state.spiral_y) 
            || (self.spiral_state.spiral_x > 0 && self.spiral_state.spiral_x == 1 - self.spiral_state.spiral_y) 
        {
            let temp = self.spiral_state.dx;
            self.spiral_state.dx = -self.spiral_state.dy;
            self.spiral_state.dy = temp;
        }
        self.spiral_state.spiral_x += self.spiral_state.dx;
        self.spiral_state.spiral_y += self.spiral_state.dy;
        coord
    }

    /// Finds the next chunk that needs spawning
    pub fn try_get_next_chunk(&mut self) -> Option<IVec2> {
        loop {
            if self.spiral_state.spiral_x.abs() > self.config.radius || 
               self.spiral_state.spiral_y.abs() > self.config.radius {
                return None;
            }

            let coord = self.next_coord();

            if !self.spawned_chunks.contains(&coord) {
                return Some(coord);
            }
        }
    }
}