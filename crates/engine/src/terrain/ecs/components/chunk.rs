use std::ops::Deref;

use bevy::{ecs::component::Component, math::IVec2};

use crate::terrain::{constants::*, types::Voxel};

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
    // Fixed-size, contíguo, sem realocação acidental
    pub voxels: Box<[Voxel]>,
}

impl ChunkData {
    pub fn new() -> Self {
        let len = (CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH) as usize;
        Self {
            voxels: vec![Voxel::Air; len].into_boxed_slice(),
        }
    }

    /// Layout: X varia mais rápido, depois Z, depois Y
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

    /// Get "seguro": fora do chunk retorna Air
    #[inline]
    pub fn get(&self, x: i32, y: i32, z: i32) -> Voxel {
        if !Self::in_bounds(x, y, z) {
            return Voxel::Air;
        }
        self.voxels[Self::index(x, y, z)]
    }

    /// Set "seguro": ignora se estiver fora
    #[inline]
    pub fn set(&mut self, x: i32, y: i32, z: i32, v: Voxel) {
        if !Self::in_bounds(x, y, z) {
            return;
        }
        let i = Self::index(x, y, z);
        self.voxels[i] = v;
    }

    // ---------------------------
    // Helpers úteis (geração/teste)
    // ---------------------------

    /// Preenche o chunk inteiro com um voxel (ex: tudo sólido pra teste)
    #[inline]
    pub fn fill(&mut self, v: Voxel) {
        self.voxels.fill(v);
    }

    /// Atalho comum: limpa tudo pra Air
    #[inline]
    pub fn clear_air(&mut self) {
        self.fill(Voxel::Air);
    }

    /// Preenche tudo com Solid até uma certa altura (Y < height)
    /// Ex: height=32 cria "solo" de 32 blocos de altura.
    pub fn fill_layer_below(&mut self, height: i32, v: Voxel) {
        let h = height.clamp(0, CHUNK_HEIGHT);

        for y in 0..CHUNK_HEIGHT {
            let fill_this_y = y < h;
            for z in 0..CHUNK_DEPTH {
                for x in 0..CHUNK_WIDTH {
                    self.set(x, y, z, if fill_this_y { v } else { Voxel::Air });
                }
            }
        }
    }
}
