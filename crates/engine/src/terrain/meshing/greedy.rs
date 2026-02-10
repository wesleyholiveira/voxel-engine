use crate::terrain::constants::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
use crate::terrain::ecs::components::chunk::{ChunkData};
use crate::terrain::meshing::mesh_data::MeshData;
use crate::terrain::types::Voxel;

#[derive(Clone, Copy, PartialEq, Eq)]
struct FaceCell {
    // por enquanto, só 1 material (Solid). Depois vira u8/u16.
    mat: u8,
    // +1 ou -1 no eixo atual
    normal_sign: i8,
}

type MaskCell = Option<FaceCell>;

#[inline]
fn is_solid(v: Voxel) -> bool {
    v.is_solid()
}

/// Gera MeshData usando greedy meshing (1 mesh por chunk)
pub fn greedy_mesh(chunk: &ChunkData) -> MeshData {
    let dims = [CHUNK_WIDTH, CHUNK_HEIGHT, CHUNK_DEPTH];
    let mut out = MeshData::new();

    // d = eixo normal (0=X, 1=Y, 2=Z)
    for axis in 0..3 {
        let u = (axis + 1) % 3;
        let v = (axis + 2) % 3;

        let plane_w = dims[u];
        let plane_h = dims[v];
        let axis_len = dims[axis];

        // mask é um plano plane_w x plane_h
        let mut mask: Vec<MaskCell> = vec![None; ( plane_w * plane_h) as usize];

        // varre "entre" células: slice = -1.-1
        // plane = slice+1 é onde o quad fica
        for slice in -1..axis_len {
            // 1) monta máscara 2D
            for j in 0..plane_h {
                for i in 0.. plane_w {
                    let mut x = [0i32; 3];
                    x[u] = i;
                    x[v] = j;
                    x[axis] = slice;

                    // neg_side = voxel do "lado de trás" (plane)
                    let neg_side = if slice >= 0 {
                        chunk.get(x[0], x[1], x[2])
                    } else {
                        Voxel::Air
                    };

                    // pos_side = voxel do "lado da frente" (plane+1)
                    let pos_side = if slice < axis_len - 1 {
                        chunk.get(x[0] + (axis == 0) as i32, x[1] + (axis == 1) as i32, x[2] + (axis == 2) as i32)
                    } else {
                        Voxel::Air
                    };

                    let idx = (i + j * plane_w) as usize;

                    mask[idx] = match (is_solid(neg_side), is_solid(pos_side)) {
                        (true, false) => Some(FaceCell { mat: 1, normal_sign: 1 }),  // face +axis
                        (false, true) => Some(FaceCell { mat: 1, normal_sign: -1 }), // face -axis
                        _ => None,
                    };
                }
            }

            // 2) greedy 2D na máscara
            let mut j = 0;
            while j < plane_h {
                let mut i = 0;
                while i <  plane_w {
                    let idx = (i + j * plane_w) as usize;
                    let cell = mask[idx];

                    if cell.is_none() {
                        i += 1;
                        continue;
                    }
                    let cell = cell.unwrap();

                    // largura w
                    let mut w = 1;
                    while i + w <  plane_w {
                        let idx2 = ((i + w) + j * plane_w) as usize;
                        if mask[idx2] == Some(cell) {
                            w += 1;
                        } else {
                            break;
                        }
                    }

                    // altura h
                    let mut h = 1;
                    'grow_h: while j + h < plane_h {
                        for k in 0..w {
                            let idx3 = ((i + k) + (j + h) * plane_w) as usize;
                            if mask[idx3] != Some(cell) {
                                break 'grow_h;
                            }
                        }
                        h += 1;
                    }

                    // 3) emite 1 quad (no plano plane = slice+1)
                    let plane = slice + 1;
                    emit_quad(&mut out, axis, u, v, plane, i, j, w, h, cell.normal_sign);

                    // 4) zera células consumidas
                    for y in 0..h {
                        for x in 0..w {
                            let idx4 = ((i + x) + (j + y) * plane_w) as usize;
                            mask[idx4] = None;
                        }
                    }

                    i += w;
                }
                j += 1;
            }
        }
    }

    out
}

/// Converte um retângulo (i,j,w,h) no plano do eixo `d` em 2 triângulos
fn emit_quad(
    out: &mut MeshData,
    d: usize, u: usize, v: usize,
    plane: i32,
    i: i32, j: i32,
    w: i32, h: i32,
    normal_sign: i8,
) {
    let normal = {
        let mut n = [0.0f32; 3];
        n[d] = normal_sign as f32;
        n
    };

    // canto inicial/final no grid do plano
    let u0 = i;
    let v0 = j;
    let u1 = i + w;
    let v1 = j + h;

    // 4 vértices no plano
    // ordem CCW para a normal “pra fora”
    let corners = if normal_sign == 1 {
        [(u0, v0), (u1, v0), (u1, v1), (u0, v1)]
    } else {
        // inverte winding
        [(u0, v0), (u0, v1), (u1, v1), (u1, v0)]
    };

    let base = out.positions.len() as u32;

    for (uu, vv) in corners {
        let mut p = [0.0f32; 3];
        p[d] = plane as f32;
        p[u] = uu as f32;
        p[v] = vv as f32;

        out.positions.push(p);
        out.normals.push(normal);
    }

    // UV simples (0..1). Depois você troca por atlas/material.
    out.uvs.extend_from_slice(&[
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
    ]);

    out.indices.extend_from_slice(&[
        base, base + 1, base + 2,
        base, base + 2, base + 3,
    ]);
}