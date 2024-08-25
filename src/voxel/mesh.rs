use crate::voxel::drawdata::DrawData;
use crate::voxel::chunk;

pub fn greedy(chunk: &chunk::Chunk) -> DrawData {
    fn get_voxel(chunk: &chunk::Chunk, i: i32, j: i32, k: i32) -> i32 {
        let index: usize = (i + chunk.dims[0] as i32 * (j + chunk.dims[1] as i32 * k)) as usize;
        chunk.volume[index]    
    }
    
    let mut draw_data = DrawData::default();
    
    //Sweep over 3-axis
    for d in 0..3 {
        let u: usize = (d+1)%3;
        let v: usize = (d+2)%3;
		let mut x: [i32; 3] = [0, 0, 0];
        let mut q: [i32; 3] = [0,0,0];
        let mut mask: Vec<bool> = vec![false; chunk.dims[u] * chunk.dims[v]];
        q[d] = 1;
		x[d] = -1;

		while x[d] < chunk.dims[d] as i32 {
            
            // Compute mask
            for xv in 0..chunk.dims[v] {
                for xu in 0..chunk.dims[u] {
                    x[v] = xv as i32;
                    x[u] = xu as i32;
                    let n: usize = xu + xv * chunk.dims[u];
                    let mut check_one: i32 = 0;
                    let mut check_two: i32 = 0;
                    
                    if 0 <= x[d] {
                        check_one = get_voxel(chunk, x[0], x[1], x[2]);
                    }
                    if x[d] < (chunk.dims[d]-1) as i32 {
                        check_two = get_voxel(chunk, x[0]+q[0], x[1]+q[1], x[2]+q[2]);
                    }
                    
                    mask[n] = check_one != check_two;
                }
            }

			x[d] += 1;
            
            // Generate mesh for mask using lexicographic ordering
            let mut n: usize = 0;
            for j in 0..chunk.dims[v] {
                let mut i = 0;
                while i < chunk.dims[u] {
                    if mask[n] {
                        // Compute width
                        let mut w: usize = 1;
                        while i + w < chunk.dims[u] && mask[n + w] {
                            w += 1;
                        }
                        
                        // Compute height
                        let mut h: usize = 1;
                        'outer: while j + h < chunk.dims[v] {
                            for k in 0..w {
                                if !mask[n + k + h * chunk.dims[u]] {
                                    break 'outer;
                                }
                            }
                            h += 1;
                        }
                        
                        // Add quad
                        x[u] = i as i32;
                        x[v] = j as i32;
                        let mut du: [usize; 3] = [0,0,0];
                        let mut dv: [usize; 3] = [0,0,0];
                        du[u] = w;
                        dv[v] = h;

                        draw_data.vertices.extend_from_slice(&[x[0], x[1], x[2]]);
                        draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32, x[1]+du[1] as i32, x[2]+du[2] as i32]);
                        draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32+dv[0] as i32, x[1]+du[1] as i32+dv[1] as i32, x[2]+du[2] as i32+dv[2] as i32]);
                        draw_data.vertices.extend_from_slice(&[x[0]+dv[0] as i32, x[1]+dv[1] as i32, x[2]+dv[2] as i32]);
                        
                        // Zero-out mask
                        for l in 0..h {
                            for k in 0..w {
                                mask[n + k + l * chunk.dims[u]] = false;
                            }
                        }
                        
                        // Increment counters and continue
                        i += w;
                        n += w;
                    } else {
                        i += 1;
                        n += 1;
                    }
                }
            }
        }
    }
    draw_data
}