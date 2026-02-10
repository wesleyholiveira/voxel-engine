use std::ops::Deref;

use bevy::{ecs::component::Component, math::IVec2, tasks::Task};

use crate::terrain::{constants::*, meshing::mesh_data::MeshData, types::Voxel};

#[derive(Component)]
pub struct Chunk;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoords(pub IVec2);

impl Deref for ChunkCoords {
    type Target = IVec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Component)]
pub struct ChunkData {
    // Fixed-size array of voxels, stored in xzy order (x changes fastest, then z, then y)
    pub voxels: Box<[Voxel]>,
}

impl ChunkData {
    pub fn new() -> Self {
        let len = (CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH) as usize;
        Self {
            // Initialize with 0 (Assuming 0 is always Air in your Global Palette)
            voxels: vec![Voxel::Air; len].into_boxed_slice(),
        }
    }

    #[inline]
    pub fn index(x: i32, y: i32, z: i32) -> usize {
        debug_assert!(
            Self::in_bounds(x, y, z),
            "ChunkData::index out of bounds: x={x} y={y} z={z}"
        );
        (x + z * CHUNK_WIDTH + y * CHUNK_WIDTH * CHUNK_DEPTH) as usize
    }

    #[inline]
    pub fn in_bounds(x: i32, y: i32, z: i32) -> bool {
        (0..CHUNK_WIDTH).contains(&x)
            && (0..CHUNK_HEIGHT).contains(&y)
            && (0..CHUNK_DEPTH).contains(&z)
    }

    /// Safe get. Does not panic if out of bounds, returns Air instead.
    #[inline]
    pub fn get(&self, x: i32, y: i32, z: i32) -> Voxel {
        if !Self::in_bounds(x, y, z) {
            return Voxel::Air; 
        }
        self.voxels[Self::index(x, y, z)]
    }

    /// Safe set. Does nothing if out of bounds.
    #[inline]
    pub fn set(&mut self, x: i32, y: i32, z: i32, pallete: Voxel) {
        if !Self::in_bounds(x, y, z) { return; }
        let i = Self::index(x, y, z);
        self.voxels[i] = pallete;
    }

    /// Fill the entire chunk with a single voxel type. Useful for initialization or resetting.
    #[inline]
    pub fn fill(&mut self, pallete: Voxel) {
        self.voxels.fill(pallete);
    }

    /// Common shorthand to fill with Air, since it's the most common "reset" state
    #[inline]
    pub fn clear_air(&mut self) {
        self.fill(Voxel::Air);
    }

    pub fn fill_layer_below(&mut self, height: i32, pallete: Voxel) {
        let h = height.clamp(0, CHUNK_HEIGHT) as usize;
        let layer_size = (CHUNK_WIDTH * CHUNK_DEPTH) as usize;
        let split_point = h * layer_size;

        self.voxels[..split_point].fill(pallete);
        self.voxels[split_point..].fill(Voxel::Air);
    }
}

#[derive(Component)]
pub struct ChunkCompute(pub Task<(ChunkCoords, ChunkData, MeshData)>);
