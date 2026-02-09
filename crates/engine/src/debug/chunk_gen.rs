use bevy::prelude::*;
use crate::terrain::generator::TerrainGenerator;
use crate::terrain::constants::*;
use crate::terrain::types::Voxel;
// Use the new component-based types
use crate::terrain::ecs::components::chunk::{Chunk, ChunkCoords, ChunkData};

pub fn spawn_test_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let generator = TerrainGenerator::new(42);
    
    // 1. Initialize our ChunkCoords component
    let coords = ChunkCoords(IVec2::new(0, 0)); 
    
    // 2. Generate the voxel data 
    // (Ensure your generator.generate() now accepts ChunkCoords)
    let generated_data = generator.generate(coords);

    // 3. Calculate world position
    // coords.0 gets the inner IVec2; .y is used for world Z
    let world_offset = Vec3::new(
        (coords.x * CHUNK_WIDTH as i32) as f32,
        0.0, 
        (coords.y * CHUNK_DEPTH as i32) as f32,
    );

    let mesh_handle = meshes.add(Mesh::from(Cuboid::default()));
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.8, 0.4),
        ..default()
    });

    // 4. Spawn the Entity with the new Component architecture
    // This tuple is now a valid Bundle because everything has #[derive(Component)]
    commands.spawn((
        Chunk,
        coords, // ChunkCoords component
        ChunkData { voxels: generated_data.voxels.clone() }, 
        Transform::from_translation(world_offset),
        Visibility::default(),
    )).with_children(|parent| {
        // Nested loops to spawn individual voxel meshes
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_DEPTH {
                for x in 0..CHUNK_WIDTH {
                    // Your indexing formula: x + WIDTH * (z + DEPTH * y)
                    let i = x + CHUNK_WIDTH * (z + CHUNK_DEPTH * y);

                    if generated_data.voxels[i] == Voxel::SOLID {
                        parent.spawn((
                            Mesh3d(mesh_handle.clone()),
                            MeshMaterial3d(material_handle.clone()),
                            Transform::from_xyz(x as f32, y as f32, z as f32),
                        ));
                    }
                }
            }
        }
    });
}