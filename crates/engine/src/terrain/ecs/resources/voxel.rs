use bevy::prelude::*;

use std::{collections::HashMap, sync::Arc};

use crate::terrain::{defs::voxel::VoxelDefinition, types::Voxel};

#[derive(Resource, Clone)]
pub struct VoxelRegistry {
    // Maps the Enum/ID to the actual metadata
    pub definitions: Arc<HashMap<Voxel, VoxelDefinition>>,
}

impl VoxelRegistry {
    pub fn get(&self, voxel: &Voxel) -> &VoxelDefinition {
        self.definitions.get(voxel)
            .unwrap_or_else(|| self.definitions.get(&Voxel::Air).expect("Air must be registered"))
    }
}