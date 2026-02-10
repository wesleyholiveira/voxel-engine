use crate::terrain::meshing::mesh_data::MeshData;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

pub fn meshdata_to_bevy_mesh(data: MeshData) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, data.positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, data.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, data.uvs);
    mesh.insert_indices(Indices::U32(data.indices));

    mesh
}
