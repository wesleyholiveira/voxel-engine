use bevy::prelude::*;

use crate::terrain::constants::{CHUNK_DEPTH, CHUNK_WIDTH};
use crate::terrain::generator::TerrainGenerator;
use crate::terrain::ecs::components::chunk::{Chunk, ChunkCoords, ChunkData};
use crate::terrain::meshing::bevy_meshing::meshdata_to_bevy_mesh;
use crate::terrain::meshing::greedy::greedy_mesh;

pub fn spawn_test_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let generator = TerrainGenerator::new(42);

    let coords = ChunkCoords(IVec2::new(0, 0));

    let chunk_data: ChunkData = generator.generate(coords);

    // greedy -> MeshData -> Bevy Mesh
    let mesh_data = greedy_mesh(&chunk_data);
    let debug_mesh = mesh_data.clone(); // Para debug (printar info depois)
    let bevy_mesh = meshdata_to_bevy_mesh(mesh_data);

    info!(
    "verts={} tris={} indices={}",
        debug_mesh.positions.len(),
        debug_mesh.indices.len() / 3,
        debug_mesh.indices.len()
    );

    let mesh_handle = meshes.add(bevy_mesh);
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.8, 0.4),
        ..default()
    });

    let world_offset = Vec3::new(
        (coords.x * CHUNK_WIDTH) as f32,
        0.0,
        (coords.y * CHUNK_DEPTH) as f32,
    );

    commands.spawn((
        Chunk,
        coords,
        chunk_data,
        Mesh3d(mesh_handle),
        MeshMaterial3d(material_handle),
        Transform::from_translation(world_offset),
        GlobalTransform::default(),
        Visibility::default(),
    ));
}
