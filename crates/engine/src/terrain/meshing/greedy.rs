use crate::terrain::constants::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
use crate::terrain::ecs::components::chunk::ChunkData;
use crate::terrain::ecs::resources::voxel::VoxelRegistry;
use crate::terrain::meshing::mesh_data::MeshData;
use crate::terrain::types::Voxel;

#[derive(Clone, Copy, PartialEq, Eq)]
struct FaceCell {
    voxel: Voxel,
    normal_sign: i8,
}

type MaskCell = Option<FaceCell>;

/// Generates MeshData using greedy meshing.
/// We pass a reference to the VoxelRegistry to check 'is_solid' dynamically.
pub fn greedy_mesh(chunk: &ChunkData, registry: &VoxelRegistry) -> MeshData {
    let dims = [CHUNK_WIDTH, CHUNK_HEIGHT, CHUNK_DEPTH];
    let mut out = MeshData::new();

    for axis in 0..3 {
        let u = (axis + 1) % 3;
        let v = (axis + 2) % 3;

        let plane_w = dims[u];
        let plane_h = dims[v];
        let axis_len = dims[axis];

        let mut mask: Vec<MaskCell> = vec![None; (plane_w * plane_h) as usize];

        for slice in -1..axis_len {
            for j in 0..plane_h {
                for i in 0..plane_w {
                    let mut x = [0i32; 3];
                    x[u] = i;
                    x[v] = j;
                    x[axis] = slice;

                    let neg_side = if slice >= 0 { chunk.get(x[0], x[1], x[2]) } else { Voxel::Air };
                    let pos_side = if slice < axis_len - 1 {
                        chunk.get(
                            x[0] + (axis == 0) as i32,
                            x[1] + (axis == 1) as i32,
                            x[2] + (axis == 2) as i32,
                        )
                    } else { Voxel::Air };

                    // DATA-DRIVEN CHECK: Use the registry to see if the voxel is solid
                    let neg_solid = registry.get(&neg_side).is_solid;
                    let pos_solid = registry.get(&pos_side).is_solid;

                    let idx = (i + j * plane_w) as usize;

                    mask[idx] = match (neg_solid, pos_solid) {
                        (true, false) => Some(FaceCell { voxel: neg_side, normal_sign: 1 }),
                        (false, true) => Some(FaceCell { voxel: pos_side, normal_sign: -1 }),
                        _ => None,
                    };
                }
            }

            // Greedy grouping logic
            let mut j = 0;
            while j < plane_h {
                let mut i = 0;
                while i < plane_w {
                    let idx = (i + j * plane_w) as usize;
                    if let Some(current_cell) = mask[idx] {
                        let mut w = 1;
                        while i + w < plane_w && mask[(i + w + j * plane_w) as usize] == Some(current_cell) {
                            w += 1;
                        }

                        let mut h = 1;
                        'grow_h: while j + h < plane_h {
                            for k in 0..w {
                                if mask[(i + k + (j + h) * plane_w) as usize] != Some(current_cell) {
                                    break 'grow_h;
                                }
                            }
                            h += 1;
                        }

                        emit_quad(
                            &mut out, 
                            axis, u, v, 
                            slice + 1, i, j, w, h, 
                            current_cell.normal_sign, 
                            current_cell.voxel
                        );

                        for y in 0..h {
                            for x in 0..w {
                                mask[(i + x + (j + y) * plane_w) as usize] = None;
                            }
                        }
                        i += w;
                    } else {
                        i += 1;
                    }
                }
                j += 1;
            }
        }
    }
    out
}

fn emit_quad(
    out: &mut MeshData,
    d: usize, u: usize, v: usize,
    plane: i32,
    i: i32, j: i32,
    w: i32, h: i32,
    normal_sign: i8,
    voxel: Voxel,
) {
    let normal = {
        let mut n = [0.0f32; 3];
        n[d] = normal_sign as f32;
        n
    };

    let base = out.positions.len() as u32;

    // Vertex positions
    let corners = if normal_sign == 1 {
        [[i, j], [i + w, j], [i + w, j + h], [i, j + h]]
    } else {
        [[i, j], [i, j + h], [i + w, j + h], [i + w, j]]
    };

    for [uu, vv] in corners {
        let mut pos = [0.0f32; 3];
        pos[d] = plane as f32;
        pos[u] = uu as f32;
        pos[v] = vv as f32;
        out.positions.push(pos);
        out.normals.push(normal);
    }

    // Tiled UVs: Important for greedy quads to look like individual blocks
    out.uvs.extend_from_slice(&[
        [0.0, 0.0],
        [w as f32, 0.0],
        [w as f32, h as f32],
        [0.0, h as f32],
    ]);

    // OPTIONAL: If you add texture indices to your MeshData later:
    // out.texture_indices.push(voxel as u32);

    out.indices.extend_from_slice(&[
        base, base + 1, base + 2, 
        base, base + 2, base + 3
    ]);
}