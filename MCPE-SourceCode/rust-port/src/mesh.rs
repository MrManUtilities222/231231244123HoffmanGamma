use crate::level::Level;
use crate::chunk_codec::{CHUNK_WIDTH, CHUNK_DEPTH, CHUNK_HEIGHT};
use crate::tesselator::Tesselator;

// Base cube vertex positions (36 vertices) grouped by faces (6 verts per face).
const BASE_CUBE_POS: [f32; 36 * 3] = [
    // 0: -Z Face (North)
    0.5, -0.5, -0.5,
    -0.5, -0.5, -0.5,
    -0.5,  0.5, -0.5,
    -0.5,  0.5, -0.5,
    0.5,  0.5, -0.5,
    0.5, -0.5, -0.5,

    // 1: +Z Face (South)
    -0.5, -0.5,  0.5,
    0.5, -0.5,  0.5,
    0.5,  0.5,  0.5,
    0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5,
    -0.5, -0.5,  0.5,

    // 2: -X Face (West)
    -0.5, -0.5, -0.5,
    -0.5, -0.5,  0.5,
    -0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5,
    -0.5,  0.5, -0.5,
    -0.5, -0.5, -0.5,

    // 3: +X Face (East)
    0.5, -0.5,  0.5,
    0.5, -0.5, -0.5,
    0.5,  0.5, -0.5,
    0.5,  0.5, -0.5,
    0.5,  0.5,  0.5,
    0.5, -0.5,  0.5,

    // 4: -Y Face (Bottom)
    -0.5, -0.5, -0.5,
    0.5, -0.5, -0.5,
    0.5, -0.5,  0.5,
    0.5, -0.5,  0.5,
    -0.5, -0.5,  0.5,
    -0.5, -0.5, -0.5,

    // 5: +Y Face (Top)
    -0.5,  0.5,  0.5,
    0.5,  0.5,  0.5,
    0.5,  0.5, -0.5,
    0.5,  0.5, -0.5,
    -0.5,  0.5, -0.5,
    -0.5,  0.5,  0.5,
];

const FACE_UVS: [f32; 6 * 2] = [
    0.0, 1.0,
    1.0, 1.0,
    1.0, 0.0,
    1.0, 0.0,
    0.0, 0.0,
    0.0, 1.0,
];

pub fn generate_chunked_mesh_vertices(level: &Level, chunk_radius: i32) -> Vec<f32> {
    let mut t = Tesselator::new();
    t.begin();
    
    // Ambient occlusion / shading colors per face
    let face_shading = [
        0.8, // -Z
        0.8, // +Z
        0.6, // -X
        0.6, // +X
        0.5, // -Y
        1.0, // +Y
    ];

    for chunk_x in -chunk_radius..=chunk_radius {
        for chunk_z in -chunk_radius..=chunk_radius {
            let base_x = chunk_x * CHUNK_WIDTH as i32;
            let base_z = chunk_z * CHUNK_DEPTH as i32;

            for bx in 0..CHUNK_WIDTH {
                for bz in 0..CHUNK_DEPTH {
                    for by in 0..CHUNK_HEIGHT {
                        let gx = base_x + bx as i32;
                        let gy = by as i32;
                        let gz = base_z + bz as i32;
                        let tile = level.get_tile(gx, gy, gz);
                        if tile == crate::tile::AIR.id {
                            continue;
                        }

                        let n_px = level.get_tile(gx + 1, gy, gz);
                        let n_nx = level.get_tile(gx - 1, gy, gz);
                        let n_pz = level.get_tile(gx, gy, gz + 1);
                        let n_nz = level.get_tile(gx, gy, gz - 1);
                        let n_py = level.get_tile(gx, gy + 1, gz);
                        let n_ny = level.get_tile(gx, gy - 1, gz);

                        let cx = gx as f32 + 0.5;
                        let cy = gy as f32 + 0.5;
                        let cz = gz as f32 + 0.5;

                        // Faces: 0 = -Z, 1 = +Z, 2 = -X, 3 = +X, 4 = -Y, 5 = +Y
                        let neighbors = [n_nz, n_pz, n_nx, n_px, n_ny, n_py];

                        for face in 0..6 {
                            if !crate::tile::is_transparent(neighbors[face]) {
                                continue;
                            }

                            // Calculate Texture Index based on block type and face
                            let tex_id = match face {
                                5 if tile == crate::tile::GRASS.id => 0, // Top
                                4 if tile == crate::tile::GRASS.id => 2, // Bottom
                                _ if tile == crate::tile::GRASS.id => 3, // Sides
                                _ if tile == crate::tile::DIRT.id => 2,
                                _ if tile == crate::tile::STONE.id => 1,
                                _ if tile == crate::tile::SAND.id => 18,
                                5 if tile == crate::tile::CACTUS.id => 69, // Top
                                4 if tile == crate::tile::CACTUS.id => 71, // Bottom
                                _ if tile == crate::tile::CACTUS.id => 70, // Sides
                                5 | 4 if tile == crate::tile::LOG.id => 21, // Top/Bottom rings
                                _ if tile == crate::tile::LOG.id => 20, // Bark sides
                                _ if tile == crate::tile::LEAVES.id => 52, // Leaves
                                _ => 1, // Fallback texture
                            };

                            let tx = (tex_id % 16) as f32 / 16.0;
                            let ty = (tex_id / 16) as f32 / 16.0;
                            
                            // Apply basic directional lighting shading to vertex colors
                            let shade = face_shading[face];
                            t.color(shade, shade, shade);

                            let face_offset = face * 6 * 3;
                            for v in 0..6 {
                                let vx = BASE_CUBE_POS[face_offset + v * 3 + 0] + cx;
                                let vy = BASE_CUBE_POS[face_offset + v * 3 + 1] + cy;
                                let vz = BASE_CUBE_POS[face_offset + v * 3 + 2] + cz;
                                
                                let tex_u = tx + FACE_UVS[v * 2] / 16.0;
                                let tex_v = ty + FACE_UVS[v * 2 + 1] / 16.0;

                                t.vertex_uv(vx, vy, vz, tex_u, tex_v);
                            }
                        }
                    }
                }
            }
        }
    }

    t.end()
}
