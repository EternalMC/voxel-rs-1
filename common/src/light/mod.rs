use crate::world::chunk::{Chunk, CHUNK_SIZE};
use crate::world::HighestOpaqueBlock;
use std::collections::VecDeque;

// TODO : Add block that are source of light

pub struct LightData {
    pub light_level: [u8; (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize],
}

impl LightData {
    pub fn new() -> Self {
        Self {
            light_level: [0; (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize],
        }
    }
}

/// Take a 3x3x3 chunks bloc and 3x3 HighestOpaqueBlock and compute the light by using a BFS
pub fn compute_light(
    chunks: Vec<Option<&Chunk>>,
    highest_opaque_blocks: Vec<HighestOpaqueBlock>,
    bfs_queue: &mut VecDeque<(usize, usize, usize, u8)>,
) -> LightData {
    let mut res = LightData::new();
    bfs_queue.clear();

    const MAX_LIGHT: u32 = 15;

    let mut light_data = [0; (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE * 27) as usize];
    let mut opaque = [false; (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE * 27) as usize];
    let csize = CHUNK_SIZE as usize;

    let mut transparent_count = 0;
    let c = chunks[9 + 3 + 1].unwrap();

    for i in 0..CHUNK_SIZE {
        for j in 0..CHUNK_SIZE {
            for k in 0..CHUNK_SIZE {
                if c.get_block_at((i, j, k)) == 0 {
                    // TODO: block registry
                    transparent_count += 1;
                }
            }
        }
    }

    let y0 = c.pos.py; // Center chunk height

    for cx in 0..3 {
        for cy in 0..3 {
            for cz in 0..3 {
                let chunk = chunks[cx * 9 + cy * 3 + cz];
                let highest_opaque_block = &highest_opaque_blocks[cx * 3 + cz];
                // First we compute the range of the blocks we have to check in the chunk.
                let mut i_range = 0..CHUNK_SIZE;
                let mut j_range = 0..CHUNK_SIZE;
                let mut k_range = 0..CHUNK_SIZE;
                if cx == 0 {
                    i_range = (CHUNK_SIZE - MAX_LIGHT + 1)..CHUNK_SIZE;
                } else if cx == 2 {
                    i_range = 0..(MAX_LIGHT - 1);
                }
                if cy == 0 {
                    j_range = (CHUNK_SIZE - MAX_LIGHT + 1)..CHUNK_SIZE;
                } else if cy == 2 {
                    j_range = 0..(MAX_LIGHT - 1);
                }
                if cz == 0 {
                    k_range = (CHUNK_SIZE - MAX_LIGHT + 1)..CHUNK_SIZE;
                } else if cz == 2 {
                    k_range = 0..(MAX_LIGHT - 1);
                }
                // Then we fill the BFS queue
                match chunk {
                    None => {
                        for i in i_range {
                            for k in j_range.clone() {
                                for j in k_range.clone() {
                                    let s = (cx * csize + i as usize) * csize * csize * 9
                                        + (cy * csize + j as usize) * csize * 3
                                        + (cz * csize + k as usize);
                                    if (y0 + cy as i64 - 1) * CHUNK_SIZE as i64 + j as i64
                                        > highest_opaque_block.y[(i * CHUNK_SIZE + k) as usize]
                                    {
                                        light_data[s] = 15;
                                        bfs_queue.push_back((
                                            cx * csize + i as usize,
                                            cy * csize + j as usize,
                                            cz * csize + k as usize,
                                            15,
                                        ));
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    Some(c) => {
                        for i in i_range {
                            for j in j_range.clone() {
                                for k in k_range.clone() {
                                    let s = (cx * csize + i as usize) * csize * csize * 9
                                        + (cy * csize + j as usize) * csize * 3
                                        + (cz * csize + k as usize);
                                    if c.get_block_at((i, j, k)) != 0 {
                                        // TODO : replace by is opaque
                                        opaque[s] = true;
                                    } else if c.pos.py * CHUNK_SIZE as i64 + j as i64
                                        > highest_opaque_block.y[(i * CHUNK_SIZE + k) as usize]
                                    {
                                        light_data[s] = 15;
                                        bfs_queue.push_back((
                                            cx * csize + i as usize,
                                            cy * csize + j as usize,
                                            cz * csize + k as usize,
                                            15,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    const MIN_VAL: isize = CHUNK_SIZE as isize - MAX_LIGHT as isize + 1;
    const MAX_VAL: isize = 2 * CHUNK_SIZE as isize + MAX_LIGHT as isize;
    const DX: [isize; 6] = [1, -1, 0, 0, 0, 0];
    const DY: [isize; 6] = [0, 0, 1, -1, 0, 0];
    const DZ: [isize; 6] = [0, 0, 0, 0, 1, -1];

    while !bfs_queue.is_empty() && transparent_count > 0 {
        let (x, y, z, ll) = bfs_queue.pop_front().unwrap();
        if x / csize == 1
            && y / csize == 1
            && z / csize == 1
            && !opaque[x * csize * csize * 9 + y * csize * 3 + z]
        {
            transparent_count -= 1;
        }

        for i in 0..6 {
            let (nx, ny, nz) = (x as isize + DX[i], y as isize + DY[i], z as isize + DZ[i]);
            let s = (nx as usize) * csize * csize * 9 + (ny as usize)*csize*3+(nz as usize);
            if MIN_VAL <= nx
                && nx < MAX_VAL
                && MIN_VAL <= ny
                && ny < MAX_VAL
                && MIN_VAL <= nz
                && nz < MAX_VAL
                && light_data[s as usize] < ll - 1
            {
                light_data[s as usize] = ll - 1;
                if ll > 1 {
                    bfs_queue.push_back((nx as usize, ny as usize, nz as usize, ll - 1));
                }
            }
        }
    }

    for i in 0..csize {
        for j in 0..csize {
            for k in 0..csize {
                res.light_level[i * csize * csize + j * csize + k] = light_data
                    [(i + csize) * csize * csize * 9 + (j + csize) * 3 * csize + (k + csize)];
            }
        }
    }

    return res;
}
