use std::collections::HashMap;
use crate::voxel::CHUNK_SIZE;
use crate::voxel::chunk;
use crate::voxel::mesh;


pub struct World {
	pub chunks: HashMap<i32, chunk::Chunk>,
}

pub fn create() -> World {
	World { chunks : HashMap::new() }
}

pub fn change_block(world: &mut World, x: i32, y: i32, z: i32, block: i32) {
	let chunk_index: i32 = chunk::get_index(x, y, z);
	
	if !world.chunks.contains_key(&chunk_index) {
		chunk::create(world, chunk_index);
	}
	
	let chunk_position: [i32; 3] = chunk::get_position(x, y, z);
	let block_position: [i32; 3] = [x - chunk_position[0]*CHUNK_SIZE, y - chunk_position[1]*CHUNK_SIZE, z - chunk_position[2]*CHUNK_SIZE];
	let block_id: i32 = block_position[0]+block_position[1]*CHUNK_SIZE+block_position[2]*CHUNK_SIZE*CHUNK_SIZE;
	
	let chunk: &mut chunk::Chunk = world.chunks.get_mut(&chunk_index).unwrap();
	chunk.volume[block_id as usize] = block;
    let chunk_draw_data = mesh::greedy(chunk);
	chunk.vertices = chunk_draw_data.vertices;
}