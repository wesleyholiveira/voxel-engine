use bevy::log::{info, warn};
use bevy::pbr::wireframe::Wireframe;
use bevy::tasks::futures_lite::future;

use bevy::{
    asset::Assets,
    color::Color,
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res, ResMut},
    },
    math::{IVec2, Vec3},
    mesh::{Mesh, Mesh3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::{GlobalTransform, InheritedVisibility, ViewVisibility, Visibility},
    tasks::AsyncComputeTaskPool,
    transform::components::Transform,
    utils::default,
};
// Add the following import or define TerrainGenerator if it's in another module
use crate::terrain::{
    constants::{CHUNK_DEPTH, CHUNK_WIDTH},
    ecs::{
        components::chunk::{Chunk, ChunkCompute, ChunkCoords},
        resources::voxel::VoxelRegistry,
    },
    generator::TerrainManager,
    meshing::{bevy_meshing::meshdata_to_bevy_mesh, greedy::greedy_mesh},
};

pub struct TerrainTask;

impl TerrainTask {
    /// System that spawns the background work
    pub fn queue(
        mut commands: Commands,
        mut manager: ResMut<TerrainManager>,
        registry: Res<VoxelRegistry>,
    ) {
        // We use a while loop to fill the thread budget (e.g., up to 4)
        while manager.active_permits < manager.config.threads {
            
            // try_get_next_chunk handles the spiral logic and HashSet check
            if let Some(coord) = manager.try_get_next_chunk() {
                // Take the permit and mark as spawned
                manager.active_permits += 1;
                manager.spawned_chunks.insert(coord);

                let chunk_coords = ChunkCoords(coord);
                let thread_pool = AsyncComputeTaskPool::get();
                
                let manager_clone = manager.clone();
                let registry_clone = registry.clone();

                let task = thread_pool.spawn(async move {
                    let data = manager_clone.run(chunk_coords);
                    let mesh_data = greedy_mesh(&data, &registry_clone);
                    (chunk_coords, data, mesh_data)
                });

                commands.spawn(ChunkCompute(task));
            } else {
                // If None, we've hit the max radius (1024)
                break;
            }
        }
    }

    /// System that collects finished background work
    pub fn process(
        mut commands: Commands,
        mut manager: ResMut<TerrainManager>,
        mut tasks: Query<(Entity, &mut ChunkCompute)>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        for (entity, mut task) in &mut tasks {
            if let Some((coords, data, mesh_data)) =
                future::block_on(future::poll_once(&mut task.0))
            {
                if manager.active_permits > 0 {
                    manager.active_permits -= 1;
                }
                let vert_count = mesh_data.positions.len();
                info!("{:?} finished. Vertices: {}", coords, vert_count);

                if vert_count == 0 {
                    warn!("Chunk {:?} generated an empty mesh!", coords);
                    commands.entity(entity).despawn();
                    continue;
                }

                let bevy_mesh = meshdata_to_bevy_mesh(mesh_data);
                let mesh_handle = meshes.add(bevy_mesh);

                // Debug material: Bright red and unlit (no lights needed)
                let material_handle = materials.add(StandardMaterial {
                    base_color: Color::srgb(1.0, 0.0, 0.0),
                    unlit: true,
                    ..default()
                });

                let world_offset = Vec3::new(
                    (coords.x * CHUNK_WIDTH) as f32,
                    0.0,
                    (coords.y * CHUNK_DEPTH) as f32,
                );

                commands
                    .entity(entity)
                    .insert((
                        Chunk,
                        coords,
                        data,
                        Mesh3d(mesh_handle),
                        MeshMaterial3d(material_handle),
                        Transform::from_translation(world_offset),
                        GlobalTransform::default(),
                        Visibility::default(),
                        InheritedVisibility::default(),
                        ViewVisibility::default(),
                        Wireframe::default(),
                    ))
                    .remove::<ChunkCompute>();
            }
        }
    }
}
