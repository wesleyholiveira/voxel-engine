use bevy::app::{App, Plugin};
use std::collections::HashMap;
use std::thread;

use bevy::prelude::*;

use crate::terrain::ecs::resources::voxel::VoxelRegistry;

use crate::terrain::tasks::TerrainTask;
use crate::terrain::{defs::voxel::VoxelDefinition, types::Voxel};
use crate::terrain::generator::TerrainManager;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        let mut definitions = HashMap::new();

        // Register your "Palette" here
        definitions.insert(Voxel::Air, VoxelDefinition {
            name: "Air",
            is_solid: false,
        });

        definitions.insert(Voxel::Solid, VoxelDefinition {
            name: "Stone",
            is_solid: true,
        });

        app.insert_resource(VoxelRegistry { definitions: definitions.into() });
    }
}

pub struct TerrainTaskPlugin;

impl Plugin for TerrainTaskPlugin {
    fn build(&self, app: &mut App) {
        // 1. Create the generator with a seed
        let manager = TerrainManager::new(64, (thread::available_parallelism().unwrap().get() / 2) as usize, 42); 

        // 2. Insert it as a resource so systems can find it
        app.insert_resource(manager);
        // 3. Register your systems
        app.add_systems(Update, (
            TerrainTask::queue,
            TerrainTask::process,
        ));
    }
}

