use crate::voxel::drawdata::DrawData;
use crate::voxel::chunk;
use js_sys::Uint8Array;
use crate::log;



pub fn greedy(chunk: &chunk::Chunk) {
    fn get_voxel(chunk: &chunk::Chunk, i: i32, j: i32, k: i32) -> i32 {
        let mut return_value: i32 = 0;

        let index: usize = (i + chunk.dims[0] as i32 * (j + chunk.dims[1] as i32 * k)) as usize;
        match &chunk.volume {
            Some(shared_array_buffer) => {
                let volume_array: Uint8Array = Uint8Array::new(shared_array_buffer);
                return_value = volume_array.get_index(index as u32) as i32;
            }        
            None => {}
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
        let mut mask: Vec<u8> = vec![0; chunk.dims[u] * chunk.dims[v]];
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
                    
                    if check_one != check_two {
                        if check_one != 0 {
                            mask[n] = check_one as u8; 
                        } else {
                            mask[n] = check_two as u8;
                        }
                    }
                }
            }

			x[d] += 1;
            
            // Generate mesh for mask using lexicographic ordering
            let mut n: usize = 0;
            for j in 0..chunk.dims[v] {
                let mut i = 0;
                while i < chunk.dims[u] {


                    if mask[n] != 0 {
                        // Compute width
                        let mut w: usize = 1;
                        while i + w < chunk.dims[u] && mask[n + w] == mask[n] {
                            w += 1;
                        }
                        
                        // Compute height
                        let mut h: usize = 1;
                        'outer: while j + h < chunk.dims[v] {
                            for k in 0..w {
                                if mask[n + k + h * chunk.dims[u]] != mask[n] {
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

                        if positive {
                            draw_data.vertices.extend_from_slice(&[x[0], x[1], x[2]]); //0 
                            draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32, x[1]+du[1] as i32, x[2]+du[2] as i32]); //1
                            draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32+dv[0] as i32, x[1]+du[1] as i32+dv[1] as i32, x[2]+du[2] as i32+dv[2] as i32]); //2
                            draw_data.vertices.extend_from_slice(&[x[0]+dv[0] as i32, x[1]+dv[1] as i32, x[2]+dv[2] as i32]); //3

                            draw_data.block_type.extend_from_slice(&[mask[n] as i32 - 1, mask[n] as i32 - 1, mask[n] as i32 - 1, mask[n] as i32 - 1]);
                            draw_data.texture_coordinates.extend_from_slice(&[0,0, w as i32 ,0, w as i32,h as i32, 0,h as i32]);

                        } else {
                            draw_data.vertices.extend_from_slice(&[x[0], x[1], x[2]]); //0 
                            draw_data.vertices.extend_from_slice(&[x[0]+dv[0] as i32, x[1]+dv[1] as i32, x[2]+dv[2] as i32]); //3
                            draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32+dv[0] as i32, x[1]+du[1] as i32+dv[1] as i32, x[2]+du[2] as i32+dv[2] as i32]); //2
                            draw_data.vertices.extend_from_slice(&[x[0]+du[0] as i32, x[1]+du[1] as i32, x[2]+du[2] as i32]); //1
                            
                            draw_data.block_type.extend_from_slice(&[mask[n] as i32 - 1, mask[n] as i32 - 1, mask[n] as i32 - 1, mask[n] as i32 - 1]);
                            draw_data.texture_coordinates.extend_from_slice(&[
                                0,0, 0,h as i32,w as i32,h as i32, w as i32 ,0
                            ]);
                        }
                        
                        // Zero-out mask
                        for l in 0..h {
                            for k in 0..w {
                                mask[n + k + l * chunk.dims[u]] = 0;
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
        Some(vertices_shared_buffer) => {
            let vertices_array = Uint8Array::new(vertices_shared_buffer);
            vertices_array.set_index(0, draw_data.vertices.len() as u8);
            for x in 1..vertices_array.length() {
                if x < (draw_data.vertices.len()+1) as u32 {
                    vertices_array.set_index(x, draw_data.vertices[(x - 1) as usize] as u8);
                } else {
                    break;
                }
            }

        }
        None => {}
    }

    match &chunk.block_type {
        Some(block_type_shared_buffer) => {
            let block_type_array = Uint8Array::new(block_type_shared_buffer);
            block_type_array.set_index(0, draw_data.block_type.len() as u8);
            for x in 1..block_type_array.length() {
                if x < (draw_data.block_type.len()+1) as u32 {
                    block_type_array.set_index(x, draw_data.block_type[(x - 1) as usize] as u8);
                } else {
                    break;
                }
            }

        }
        None => {}
    }

    match &chunk.texture_coordinates {
        Some(texture_coordinates_shared_buffer) => {
            let texture_coordinates_array = Uint8Array::new(texture_coordinates_shared_buffer);
            texture_coordinates_array.set_index(0, draw_data.texture_coordinates.len() as u8);
            for x in 1..texture_coordinates_array.length() {
                if x < (draw_data.texture_coordinates.len()+1) as u32 {
                    texture_coordinates_array.set_index(x, draw_data.texture_coordinates[(x - 1) as usize] as u8);
                } else {
                    break;
                }
            }

        }
        None => {}
    }

}