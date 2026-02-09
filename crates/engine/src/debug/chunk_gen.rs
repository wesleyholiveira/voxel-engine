use bevy::prelude::*;

use crate::terrain::constants::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
use crate::terrain::ecs::components::chunk::{Chunk, ChunkCoords, ChunkData};
use crate::terrain::generator::TerrainGenerator;
use crate::terrain::types::Voxel;

pub fn spawn_test_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let generator = TerrainGenerator::new(42);

    let coords = ChunkCoords(IVec2::new(0, 0));

    let generated_data: ChunkData = generator.generate(coords);
    let voxels_debug = generated_data.voxels.clone(); // debug-only

    // offset do chunk no mundo (coords.x = chunk_x, coords.y = chunk_z)
    let world_offset = Vec3::new(
        (coords.x * CHUNK_WIDTH) as f32,
        0.0,
        (coords.y * CHUNK_DEPTH) as f32,
    );

    let mesh_handle = meshes.add(Mesh::from(Cuboid::default()));
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.8, 0.4),
        ..default()
    });

    commands
        .spawn((
            Chunk,
            coords,
            generated_data,
            Transform::from_translation(world_offset),
            GlobalTransform::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_DEPTH {
                    for x in 0..CHUNK_WIDTH {
                        let i = ChunkData::index(x, y, z);
                        if voxels_debug[i] == Voxel::Solid {
                            parent.spawn((
                                Mesh3d(mesh_handle.clone()),
                                MeshMaterial3d(material_handle.clone()),
                                Transform::from_xyz(x as f32, y as f32, z as f32),
                                GlobalTransform::default(),
                                Visibility::default(),
                            ));
                        }
                    }
                }
            }
        });
}
