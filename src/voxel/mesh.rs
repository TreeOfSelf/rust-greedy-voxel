use crate::log;
use crate::voxel::drawdata::DrawData;
use crate::voxel::chunk;
use js_sys::{Int32Array, Uint8Array};


pub fn greedy(chunk: &chunk::Chunk) {
    fn get_voxel(chunk: &chunk::Chunk, i: i32, j: i32, k: i32) -> i32 {
        let mut return_value: i32 = 0;

        let index: usize = (i + chunk.dims[0] as i32 * (j + chunk.dims[1] as i32 * k)) as usize;
        match &chunk.volume {
            Some(shared_array_buffer) => {
                let volume_array: Uint8Array = Uint8Array::new(shared_array_buffer);
                return_value = volume_array.get_index(index as u32) as i32;
            }        
            None => {
                log("Didn't find");
            }
        }
        return_value
    }
    
    let mut draw_data: DrawData = DrawData::default();
    
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
            let mut positive = false;

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

                    if check_one != 0 && check_two == 0 {
                        positive = true;
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

                        log(format!("Axis d: {}, positive: {:?}", d, positive).as_str()); // Add this line to check the state of q
                        match (d, positive) {
                            (0,true) => {
                                draw_data.vertices.extend_from_slice(&[x[0], x[1], x[2]]); //0 
                                draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32, x[1]+du[1] as i32, x[2]+du[2] as i32]); //1
                                draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32+dv[0] as i32, x[1]+du[1] as i32+dv[1] as i32, x[2]+du[2] as i32+dv[2] as i32]); //2
                                draw_data.vertices.extend_from_slice(&[x[0]+dv[0] as i32, x[1]+dv[1] as i32, x[2]+dv[2] as i32]); //3
                            },
                            (0,false) => {
                                draw_data.vertices.extend_from_slice(&[x[0], x[1], x[2]]); //0 
                                draw_data.vertices.extend_from_slice(&[x[0]+dv[0] as i32, x[1]+dv[1] as i32, x[2]+dv[2] as i32]); //3
                                draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32+dv[0] as i32, x[1]+du[1] as i32+dv[1] as i32, x[2]+du[2] as i32+dv[2] as i32]); //2
                                draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32, x[1]+du[1] as i32, x[2]+du[2] as i32]); //1
                            },
                            (1,true) => {
                                draw_data.vertices.extend_from_slice(&[x[0], x[1], x[2]]); //0 
                                draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32, x[1]+du[1] as i32, x[2]+du[2] as i32]); //1
                                draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32+dv[0] as i32, x[1]+du[1] as i32+dv[1] as i32, x[2]+du[2] as i32+dv[2] as i32]); //2
                                draw_data.vertices.extend_from_slice(&[x[0]+dv[0] as i32, x[1]+dv[1] as i32, x[2]+dv[2] as i32]); //3
                            },
                            (1,false) => {
                                draw_data.vertices.extend_from_slice(&[x[0], x[1], x[2]]); //0 
                                draw_data.vertices.extend_from_slice(&[x[0]+dv[0] as i32, x[1]+dv[1] as i32, x[2]+dv[2] as i32]); //3
                                draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32+dv[0] as i32, x[1]+du[1] as i32+dv[1] as i32, x[2]+du[2] as i32+dv[2] as i32]); //2
                                draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32, x[1]+du[1] as i32, x[2]+du[2] as i32]); //1
                            },
                            (2,true) => {
                                draw_data.vertices.extend_from_slice(&[x[0], x[1], x[2]]); //0 
                                draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32, x[1]+du[1] as i32, x[2]+du[2] as i32]); //1
                                draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32+dv[0] as i32, x[1]+du[1] as i32+dv[1] as i32, x[2]+du[2] as i32+dv[2] as i32]); //2
                                draw_data.vertices.extend_from_slice(&[x[0]+dv[0] as i32, x[1]+dv[1] as i32, x[2]+dv[2] as i32]); //3
                            },
                            (2,false) => {
                                draw_data.vertices.extend_from_slice(&[x[0], x[1], x[2]]); //0 
                                draw_data.vertices.extend_from_slice(&[x[0]+dv[0] as i32, x[1]+dv[1] as i32, x[2]+dv[2] as i32]); //3
                                draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32+dv[0] as i32, x[1]+du[1] as i32+dv[1] as i32, x[2]+du[2] as i32+dv[2] as i32]); //2
                                draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32, x[1]+du[1] as i32, x[2]+du[2] as i32]); //1

                            },
                            _ => {
                                log("Invalid axis value.");
                            }
                        }
                
                        
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
    match &chunk.vertices {
        Some(shared_array_buffer) => {
            let int_array = Int32Array::new(shared_array_buffer);
            int_array.set_index(0, draw_data.vertices.len() as i32);
            for x in 1..int_array.length() {
                if x < (draw_data.vertices.len()+1) as u32 {
                    int_array.set_index(x, draw_data.vertices[(x - 1) as usize]);
                } else {
                    log("Reached end of draw_data.vertices");
                    break;
                }
            }

        }
        None => {
            log("Didn't find");
        }
    }

}