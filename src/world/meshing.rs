// The constant associated to the normal direction
const EAST: u32 = 0; // 1x
const WEST: u32 = 1; // -1x
const UP: u32 = 2; // 1y
const DOWN: u32 = 3; // -1y
const SOUTH: u32 = 4; // 1z
const NORTH: u32 = 5; // -1z

use super::chunk::{Chunk, CHUNK_SIZE};
use super::renderer::Vertex;

/// Structure containing information about adjacent chunks for the meshing
/// Order of face 1x, -1x, 1y, -1y, 1z, -1z => the two order component are in the (x,y,z) order
/// Order of edges (yz), (y-z), (-y z), (-y - z), (xz), (x -z), (-x z), (x - z), (xy), (x - y), (-x y) (-x - y)
/// ( xy means variation along z with x, y = (1+chunk_size, 1+chunk_size), -x y means variation along z with x, y= (-1, 1)
/// Order of coins (1,1,1), (1, 1 -1), (1, -1, 1), (1, -1, -1),
///  ... (-1,1,1), (-1, 1 -1), (-1, -1, 1), (-1, -1, -1),
#[derive(Clone, Copy)]
pub struct AdjChunkOccl{
    pub faces : [[[bool; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; 6],
    pub edges : [[bool; CHUNK_SIZE as usize]; 12],
    pub coins : [bool; 8],
}

fn delta(x : i32) -> usize{
    if x == CHUNK_SIZE as i32 {
        0
    }else if x == -1{
        1
    }else{
        0 // unreachable
    }
}

impl AdjChunkOccl{

    /// x, y, z are the position relative to the chunk (0, 0, 0)
    /// Return if the block outside the chunk is opaque
    pub fn is_full(&self, x:i32, y:i32, z:i32) -> bool{

        let mut n_outside = 0;
        if x == -1 || x == CHUNK_SIZE as i32 {
            n_outside += 1;
        }
        if y == -1 || y == CHUNK_SIZE as i32 {
            n_outside += 1;
        }
        if z == -1 || z == CHUNK_SIZE as i32 {
            n_outside += 1;
        }

        if n_outside == 1{
            if x == CHUNK_SIZE as i32 {
                return self.faces[0][y as usize][z as usize];
            }else if x == -1{
                return self.faces[1][y as usize ][z as usize];
            }else if y == CHUNK_SIZE as i32 {
                return self.faces[0][x as usize][z as usize];
            }else if y == -1{
                return self.faces[1][x as usize][z as usize];
            }else if z == CHUNK_SIZE as i32 {
                return self.faces[0][x as usize ][y as usize];
            }else if z == -1{
                return self.faces[1][x as usize][y as usize];
            }
        }else if n_outside == 2{
            if x >= 0 && x < CHUNK_SIZE as i32{
                let i = delta(y)*2 + delta(z);
                return self.edges[i][x as usize];

            }else if y >= 0 && y < CHUNK_SIZE as i32{
                let i = delta(x)*2 + delta(z);
                return self.edges[i+4][ y as usize];

            }else if z >= 0 && z < CHUNK_SIZE as i32{
                let i = delta(x)*2 + delta(y);
                return self.edges[i+8][ z as usize];
            }

        }else if n_outside == 3{
            let i = delta(x)*4 + delta(y)*2 + delta(z);
            return self.coins[i];
        }
        return false;

    }

}

const MESH_EAST: [[f32; 3]; 4] = [
    [1.0, 0.0, 0.0],
    [1.0, 1.0, 0.0],
    [1.0, 0.0, 1.0],
    [1.0, 1.0, 1.0],
];

const MESH_EAST_INDEX: [usize; 6] = [0, 1, 2, 2, 1, 3];

const MESH_WEST: [[f32; 3]; 4] = [
    [0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.0, 1.0, 1.0],
];

const MESH_WEST_INDEX: [usize; 6] = [0, 2, 1, 2, 3, 1];

const MESH_UP: [[f32; 3]; 4] = [
    [0.0, 1.0, 0.0],
    [1.0, 1.0, 0.0],
    [0.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
];

const MESH_UP_INDEX: [usize; 6] = [0, 2, 1, 2, 3, 1];

const MESH_DOWN: [[f32; 3]; 4] = [
    [0.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0],
    [1.0, 0.0, 1.0],
];

const MESH_DOWN_INDEX: [usize; 6] = [0, 1, 2, 2, 1, 3];

const MESH_NORTH: [[f32; 3]; 4] = [
    [0.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [1.0, 1.0, 0.0],
];

const MESH_NORTH_INDEX: [usize; 6] = [0, 2, 1, 2, 3, 1];

const MESH_SOUTH: [[f32; 3]; 4] = [
    [0.0, 0.0, 1.0],
    [1.0, 0.0, 1.0],
    [0.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
];
const MESH_SOUTH_INDEX: [usize; 6] = [0, 1, 2, 2, 1, 3];

/// Return True if full block (taking into account adjacent chunks)
fn is_full(chunk: &Chunk, (i, j, k): (i32, i32, i32), adj : Option<AdjChunkOccl>) -> bool {
    let size = CHUNK_SIZE as i32;
    if i >= 0 && j >= 0 && k >= 0 && i < size && j < size && k < size {
        return chunk.get_data(i as u32, j as u32, k as u32) != 0;
    }else{
        match adj {
            Some(_adj) => _adj.is_full(i,j,k),
            None => false
        }
    }

}

/// Return true if pos (x,y,z) is in block (i,j,k)
fn in_block((i, j, k): (i32, i32, i32), (x, y, z): (f32, f32, f32)) -> bool {
    let dx = x - i as f32;
    let dy = y - j as f32;
    let dz = z - k as f32;
    dx >= 0.0 && dx <= 1.0 && dy >= 0.0 && dy <= 1.0 && dz >= 0.0 && dz <= 1.0
}

/// Ambient occlusion code (cf : https://0fps.net/2013/07/03/ambient-occlusion-for-minecraft-like-worlds/)
fn ambiant_occl(coins: u32, edge: u32) -> u32 {
    if edge == 2 {
        return 0;
    } else if edge == 1 && coins == 1 {
        return 1;
    } else if edge + coins == 1 {
        return 2;
    } else {
        return 3;
    }
}

/// Return a list of vertex a (3*n) indexes array (for n quads)
/// which contains the index of the corresponding quads
/// in the first array
/// Each vertex contains its position and the normal associated to the quad
pub fn meshing(chunk: &mut Chunk, adj : Option<AdjChunkOccl>) -> (Vec<Vertex>, Vec<u32>) {
    let mut res_vertex: Vec<Vertex> = Vec::new();
    let mut res_index: Vec<usize> = Vec::new();

    let mut n_of_different_vertex = 0;

    for i in 0..CHUNK_SIZE {
        for j in 0..CHUNK_SIZE {
            for k in 0..CHUNK_SIZE {
                if chunk.get_data(i, j, k) != 0 {
                    //checking if not void
                    // 1x -- EAST
                    if !is_full(chunk, (i as i32 +1, j as i32, k as i32), adj) {
                        for l in 0..4 {
                            let px = i as f32 + MESH_EAST[l][0];
                            let py = j as f32 + MESH_EAST[l][1];
                            let pz = k as f32 + MESH_EAST[l][2];
                            res_vertex.push(Vertex {
                                pos: [px, py, pz],
                                normal: EAST
                                    + ({
                                        let mut coins = 0;
                                        let mut edge = 0;
                                        for delta1 in -1..=1 {
                                            for delta2 in -1..=1 {
                                                if delta1 != delta2 || delta1 != 0 {
                                                    let p2 = (
                                                        i as i32 + 1,
                                                        j as i32 + delta1,
                                                        k as i32 + delta2,
                                                    );
                                                    if is_full(chunk, p2, adj)
                                                        && in_block(p2, (px, py, pz))
                                                    {
                                                        if delta1.abs() == 1 && delta2.abs() == 1 {
                                                            coins += 1;
                                                        } else {
                                                            edge += 1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        ambiant_occl(coins, edge)
                                    } << 3),
                            });
                        }

                        for l in 0..6 {
                            res_index.push(n_of_different_vertex + MESH_EAST_INDEX[l]);
                        }
                        n_of_different_vertex += 4;
                    }
                    // -1x -- WEST
                    if !is_full(chunk, (i as i32 -1, j as i32, k as i32), adj) {
                        for l in 0..4 {
                            let px = i as f32 + MESH_WEST[l][0];
                            let py = j as f32 + MESH_WEST[l][1];
                            let pz = k as f32 + MESH_WEST[l][2];
                            res_vertex.push(Vertex {
                                pos: [px, py, pz],
                                normal: WEST
                                    + ({
                                        let mut coins = 0;
                                        let mut edge = 0;
                                        for delta1 in -1..=1 {
                                            for delta2 in -1..=1 {
                                                if delta1 != delta2 || delta1 != 0 {
                                                    let p2 = (
                                                        i as i32 - 1,
                                                        j as i32 + delta1,
                                                        k as i32 + delta2,
                                                    );
                                                    if is_full(chunk, p2, adj)
                                                        && in_block(p2, (px, py, pz))
                                                    {
                                                        if delta1.abs() == 1 && delta2.abs() == 1 {
                                                            coins += 1;
                                                        } else {
                                                            edge += 1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        ambiant_occl(coins, edge)
                                    } << 3),
                            });
                        }

                        for l in 0..6 {
                            res_index.push(n_of_different_vertex + MESH_WEST_INDEX[l]);
                        }
                        n_of_different_vertex += 4;
                    }
                    // 1y -- UP
                    if !is_full(chunk, (i as i32, j as i32+1, k as i32), adj) {
                        for l in 0..4 {
                            let px = i as f32 + MESH_UP[l][0];
                            let py = j as f32 + MESH_UP[l][1];
                            let pz = k as f32 + MESH_UP[l][2];
                            res_vertex.push(Vertex {
                                pos: [px, py, pz],
                                normal: UP
                                    + ({
                                        let mut coins = 0;
                                        let mut edge = 0;
                                        for delta1 in -1..=1 {
                                            for delta2 in -1..=1 {
                                                if delta1 != delta2 || delta1 != 0 {
                                                    let p2 = (
                                                        i as i32 + delta1,
                                                        j as i32 + 1,
                                                        k as i32 + delta2,
                                                    );
                                                    if is_full(chunk, p2, adj)
                                                        && in_block(p2, (px, py, pz))
                                                    {
                                                        if delta1.abs() == 1 && delta2.abs() == 1 {
                                                            coins += 1;
                                                        } else {
                                                            edge += 1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        ambiant_occl(coins, edge)
                                    } << 3),
                            });
                        }

                        for l in 0..6 {
                            res_index.push(n_of_different_vertex + MESH_UP_INDEX[l]);
                        }
                        n_of_different_vertex += 4;
                    }
                    // -1y -- DOWN
                    if !is_full(chunk, (i as i32, j as i32-1, k as i32), adj) {
                        for l in 0..4 {
                            let px = i as f32 + MESH_DOWN[l][0];
                            let py = j as f32 + MESH_DOWN[l][1];
                            let pz = k as f32 + MESH_DOWN[l][2];
                            res_vertex.push(Vertex {
                                pos: [px, py, pz],
                                normal: DOWN
                                    + ({
                                        let mut coins = 0;
                                        let mut edge = 0;
                                        for delta1 in -1..=1 {
                                            for delta2 in -1..=1 {
                                                if delta1 != delta2 || delta1 != 0 {
                                                    let p2 = (
                                                        i as i32 + delta1,
                                                        j as i32 - 1,
                                                        k as i32 + delta2,
                                                    );
                                                    if is_full(chunk, p2, adj)
                                                        && in_block(p2, (px, py, pz))
                                                    {
                                                        if delta1.abs() == 1 && delta2.abs() == 1 {
                                                            coins += 1;
                                                        } else {
                                                            edge += 1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        ambiant_occl(coins, edge)
                                    } << 3),
                            });
                        }

                        for l in 0..6 {
                            res_index.push(n_of_different_vertex + MESH_DOWN_INDEX[l]);
                        }
                        n_of_different_vertex += 4;
                    }
                    // 1z -- SOUTH
                    if !is_full(chunk, (i as i32, j as i32, k as i32+1), adj) {
                        for l in 0..4 {
                            let px = i as f32 + MESH_SOUTH[l][0];
                            let py = j as f32 + MESH_SOUTH[l][1];
                            let pz = k as f32 + MESH_SOUTH[l][2];
                            res_vertex.push(Vertex {
                                pos: [px, py, pz],
                                normal: SOUTH
                                    + ({
                                        let mut coins = 0;
                                        let mut edge = 0;
                                        for delta1 in -1..=1 {
                                            for delta2 in -1..=1 {
                                                if delta1 != delta2 || delta1 != 0 {
                                                    let p2 = (
                                                        i as i32 + delta1,
                                                        j as i32 + delta2,
                                                        k as i32 + 1,
                                                    );
                                                    if is_full(chunk, p2, adj)
                                                        && in_block(p2, (px, py, pz))
                                                    {
                                                        if delta1.abs() == 1 && delta2.abs() == 1 {
                                                            coins += 1;
                                                        } else {
                                                            edge += 1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        ambiant_occl(coins, edge)
                                    } << 3),
                            });
                        }

                        for l in 0..6 {
                            res_index.push(n_of_different_vertex + MESH_SOUTH_INDEX[l]);
                        }
                        n_of_different_vertex += 4;
                    }
                    // -1z -- NORTH
                    if !is_full(chunk, (i as i32, j as i32, k as i32-1), adj) {
                        for l in 0..4 {
                            let px = i as f32 + MESH_NORTH[l][0];
                            let py = j as f32 + MESH_NORTH[l][1];
                            let pz = k as f32 + MESH_NORTH[l][2];
                            res_vertex.push(Vertex {
                                pos: [px, py, pz],
                                normal: NORTH
                                    + ({
                                        let mut coins = 0;
                                        let mut edge = 0;
                                        for delta1 in -1..=1 {
                                            for delta2 in -1..=1 {
                                                if delta1 != delta2 || delta1 != 0 {
                                                    let p2 = (
                                                        i as i32 + delta1,
                                                        j as i32 + delta2,
                                                        k as i32 - 1,
                                                    );
                                                    if is_full(chunk, p2, adj)
                                                        && in_block(p2, (px, py, pz))
                                                    {
                                                        if delta1.abs() == 1 && delta2.abs() == 1 {
                                                            coins += 1;
                                                        } else {
                                                            edge += 1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        ambiant_occl(coins, edge)
                                    } << 3),
                            });
                        }

                        for l in 0..6 {
                            res_index.push(n_of_different_vertex + MESH_NORTH_INDEX[l]);
                        }
                        n_of_different_vertex += 4;
                    }
                }
            }
        }
    }

    let res_index: Vec<u32> = res_index.iter().map(|x| *x as u32).collect();
    (res_vertex, res_index)
}