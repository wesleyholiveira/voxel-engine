use crate::terrain::{
    constants::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH},
    ecs::components::chunk::{ChunkCoords, ChunkData},
    types::Voxel,
};

use super::{heightmap::generate_height, noise::TerrainNoise};

pub struct TerrainGenerator {
    noise: TerrainNoise,
}

impl TerrainGenerator {
    pub fn new(seed: i32) -> Self {
        Self {
            noise: TerrainNoise::new(seed),
        }
    }

    pub fn generate(&self, chunk_coord: ChunkCoords) -> ChunkData {
        let len = (CHUNK_WIDTH * CHUNK_DEPTH * CHUNK_HEIGHT) as usize;
        // Since we are IVec2, we ALWAYS fill the same vertical height
        let mut voxels = vec![Voxel::Air; len];

        for lx in 0..CHUNK_WIDTH {
            let world_x = (chunk_coord.x * CHUNK_WIDTH + lx) as f32;

            for lz in 0..CHUNK_DEPTH {
                // Here, we use .y from the IVec2 as the "Z" coordinate in 3D space
                let world_z = (chunk_coord.y * CHUNK_DEPTH as i32 + lz as i32) as f32;

                let height = generate_height(&self.noise, world_x, world_z);

                // Fill from the floor (0) up to the height calculated
                // We clamp to CHUNK_HEIGHT to avoid index out of bounds
                let fill_to = height.min(CHUNK_HEIGHT);

                for ly in 0..fill_to {
                    voxels[ChunkData::index(lx, ly, lz)] = Voxel::Solid;
                }
            }
        }

        ChunkData {
            voxels: voxels.into_boxed_slice(),
        }
    }
}
