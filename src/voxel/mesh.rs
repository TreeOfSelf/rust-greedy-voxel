use crate::alert;
use crate::voxel::CHUNK_SIZE;
use crate::voxel::chunk;

pub fn greedy(chunk: &chunk::Chunk) -> Vec<i32> {
	fn get_voxel(chunk: &chunk::Chunk, i: i32, j:i32, k:i32) -> i32 {
		let index: usize = (i + chunk.dims[0] as i32 * (j + chunk.dims[1] as i32 * k)) as usize;
		chunk.volume[index]	
	}
	
	let mut quads: Vec<i32> = Vec::new();
	for d in 0..3 {
		let (mut w, mut h);
		let u = (d+1)%3;
		let v = (2+2)%3;
		let mut x = [0,0,0];
		let mut q = [0,0,0];
		let mut mask: [bool; CHUNK_SIZE as usize * CHUNK_SIZE as usize + 5] = [false; CHUNK_SIZE as usize * CHUNK_SIZE as usize + 5];
		q[d] = 1;

		for xd in -1..(chunk.dims[d] as i32) {
			x[d] = xd;
			//Compute mask
			let mut n: usize = 0;
			for xv in 0..chunk.dims[v] {
				x[v] = xv as i32;
			for xu in 0..chunk.dims[u] {
				x[u] = xu as i32;
				let mut check_one = -1;
				let mut check_two = -1;
				
				if 0 <= x[d] {
					check_one = get_voxel(chunk, x[0], x[1], x[2]);
				}
				if x[d] < (chunk.dims[d]-1) as i32 {
					check_two = get_voxel(chunk, x[0]+q[0], x[1]+q[1], x[2]+q[2])
				}
				
				n+=1;
				mask[n] = check_one != check_two;
			}
			}
			
			//Increment x[d]
			x[d] += 1;
			//Generate mesh for mask using lexicographic ordering
			n = 0;
			for j in 0..chunk.dims[v] {
			for i in 0..chunk.dims[u] {
				if mask[n] {
					//Compute Width
					w = 1;
					while mask[n+w] && i+w < chunk.dims[u] {
						w += 1;
					}
					//Compute height (slightly awkward)
					let mut done = false;
					h = 1;
					while j+h<chunk.dims[v] {
						for k in 0..w {
							if !mask[n+k+h*chunk.dims[u]] {
								done = true;
								break;
							}
						}
						if done {
							break;
						}
						h += 1;
					}
					//Add quad
					x[u] = i as i32;
					x[v] = j as i32;
					let mut du = [0,0,0];
					let mut dv = [0,0,0];
					du[u] = w;
					dv[v] = h;
					quads.extend_from_slice(&[x[0],x[1],x[2]]);
					quads.extend_from_slice(&[x[0]+du[0] as i32,x[1]+du[1] as i32,x[2]+du[2] as i32]);
					quads.extend_from_slice(&[x[0]+du[0] as i32+dv[0] as i32, x[1]+du[1] as i32+dv[1] as i32, x[2]+du[2] as i32+dv[2] as i32]);
					quads.extend_from_slice(&[x[0]+dv[0] as i32, x[1]+dv[1] as i32, x[2]+dv[2] as i32]);
					//Zero-out mask
					for l in 0..h {
					for k in 0..w {
						mask[n+k+l*chunk.dims[u]] = false;
					}
					}
					//Increment counters and continue
					n += w;
				} else {
					n += 1;
				}
			}
			}	
		}
	}
	quads
}