use std::ops::Deref;

use bevy::{ecs::component::Component};

use bevy::{math::IVec2};

use crate::terrain::types::Voxel;

//use crate::terrain::types::chunk::{Coordinates, Data};

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
    pub voxels: Vec<Voxel>,
}
