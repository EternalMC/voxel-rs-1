/// Generate a perlin noise at position x,y,z for a block of size (size, size, size)
/// Point in the block are distant of 1
/// Use the parameter scale_x,y,z to set the scale factor
pub fn perlin(
    x: f32,
    y: f32,
    z: f32,
    size: usize,
    scale_x: f32,
    scale_y: f32,
    scale_z: f32,
    octave: i32,
    persistance: f32,
    mut seed: i32,
) -> Vec<f32> {
    let mut result = vec![0.0; size * size * size];
    let mut p = 1.0;
    let mut div = 0.0; // normalization factor
    let mut factor_x = scale_x;
    let mut factor_y = scale_y;
    let mut factor_z = scale_z;

    for _i in 0..octave {
        value_noise(
            (x, y, z),
            (size, size, size),
            (factor_x, factor_y, factor_z), p, &mut result,
            seed,
        );
        factor_x *= 2.0;
        factor_y *= 2.0;
        factor_z *= 2.0;
        seed += 1;
        div += p;
        p *= persistance;
    }

    for _j in 0..(size * size * size) {
        result[_j] = result[_j] / div;
    }
    return result;
}
/// Horribly not optimized value perlin noise
pub fn value_noise(
    (x, y, z): (f32, f32, f32),
    (size_x, size_y, size_z): (usize, usize, usize),
    (scale_x, scale_y, scale_z): (f32, f32, f32),
    p: f32,
    to_add: &mut Vec<f32>,
    seed: i32,
) {
    let min_x = (x * scale_x).floor() as i32;
    let max_x = ((x + size_x as f32 - 1.0) * scale_x).ceil() as i32;
    let min_y = (y * scale_y).floor() as i32;
    let max_y = ((y + size_y as f32 - 1.0) * scale_y).ceil() as i32;
    let min_z = (z * scale_z).floor() as i32;
    let max_z = ((z + size_z as f32 - 1.0) * scale_z).ceil() as i32;

    let nx = (max_x - min_x + 2) as usize;
    let ny = (max_y - min_y + 2) as usize;
    let nz = (max_z - min_z + 2) as usize;

    let mut values = vec![0.0; nx * ny * nz];
    //let mut res = vec![0.0; size_x * size_y * size_z];

    for i in 0..nx {
        for j in 0..ny {
            for k in 0..nz {
                let px = min_x + i as i32;
                let py = min_y + j as i32;
                let pz = min_z + k as i32;
                values[(i * ny + j) * nz + k] = rand_pos(px, py, pz, seed);
            }
        }
    }

    let mut fx = vec![0.0; size_x];
    let mut fy = vec![0.0; size_y];
    let mut fz = vec![0.0; size_z];

    let mut x_c = vec![0; size_x];
    let mut y_c = vec![0; size_y];
    let mut z_c = vec![0; size_z];

    for i in 0..size_x {
        let xx = (x + i as f32) * scale_x;
        let u_x = xx.floor();
        fx[i] = smoothstep(xx - u_x);
        x_c[i] = ((u_x as i32) - min_x) as usize;
    }


    for j in 0..size_y {
        let yy = (y + j as f32) * scale_y;
        let u_y = yy.floor();
        fy[j] = smoothstep(yy - u_y);
        y_c[j] = ((u_y as i32) - min_y) as usize;
    }

    for k in 0..size_z {
        let zz = (z + k as f32) * scale_z;
        let u_z = zz.floor();
        fz[k] = smoothstep(zz - u_z);
        z_c[k] = ((u_z as i32) - min_z) as usize;
    }

    for i in 0..size_x {
        for j in 0..size_y {
            for k in 0..size_z {
                // plz vectorize this for me
                let a_a_a = values[(x_c[i] * ny + y_c[j]) * nz + z_c[k]];
                let a_a_b = values[(x_c[i] * ny + y_c[j]) * nz + z_c[k] + 1];

                let a_a = a_a_a + (a_a_b - a_a_a) * fz[k];

                let a_b_a = values[(x_c[i] * ny + y_c[j] + 1) * nz + z_c[k]];
                let a_b_b = values[(x_c[i] * ny + y_c[j] + 1) * nz + z_c[k] + 1];

                let a_b = a_b_a + (a_b_b - a_b_a) * fz[k];

                let b_a_a = values[(x_c[i] * ny + y_c[j] + ny) * nz + z_c[k]];
                let b_a_b = values[(x_c[i] * ny + y_c[j] + ny) * nz + z_c[k] + 1];

                let b_a = b_a_a + (b_a_b - b_a_a) * fz[k];

                let b_b_a = values[(x_c[i] * ny + y_c[j] + 1 + ny) * nz + z_c[k]];
                let b_b_b = values[(x_c[i] * ny + y_c[j] + 1 + ny) * nz + z_c[k] + 1];

                let b_b = b_b_a + (b_b_b - b_b_a) * fz[k];

                let a = (a_a) + (a_b - a_a) * fy[j];
                let b = (b_a) + (b_b - b_a) * fy[j];
                to_add[(i * size_y + j) * size_z + k] += p*(a + (b - a) * fx[i]);
            }
        }
    }
}

#[inline(always)]
fn smoothstep(x: f32) -> f32 {
    let x_2 = x * x;
    let x_4 = x_2 * x_2;
    return 6.0 * x * x_4 - 15.0 * x_4 + 10.0 * x * x_2;
}

#[inline(always)]
fn rand_pos(x: i32, y: i32, z: i32, seed: i32) -> f32 {
    let a = hash(x + seed);
    let b = hash(y + a);
    let c = hash(z + b);
    let m = 10000000;
    return (((m + (c % m)) % m) as f32) / (m as f32);
}

#[inline(always)]
pub fn rand_pos_int(x: i32, y: i32, z: i32, seed: i32) -> i32 {
    let a = hash(x + seed);
    let b = hash(y + a);
    return  hash(z + b);;
}

#[inline(always)]
pub fn hash(b: i32) -> i32 {
    let mut a = b;
    a = a.wrapping_sub(a << 6);
    a ^= a >> 17;
    a = a.wrapping_sub(a << 9);
    a ^= a << 4;
    a = a.wrapping_sub(a << 3);
    a ^= a << 10;
    a ^= a >> 15;
    return a;
}