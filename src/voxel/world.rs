use std::collections::HashMap;
use crate::voxel::CHUNK_SIZE;
use crate::voxel::chunk;
use crate::voxel::mesh;
use crate::log;
use js_sys::{SharedArrayBuffer, Uint8Array};


pub struct World {
	pub chunks: HashMap<i32, chunk::Chunk>,
}

pub fn create() -> World {
	World { chunks : HashMap::new() }
}

pub fn change_block(world: &mut World, x: i32, y: i32, z: i32, block: u8) {
	let chunk_index: i32 = chunk::get_index(x, y, z);
	
	if !world.chunks.contains_key(&chunk_index) {
		chunk::create(world, chunk_index);
	}
	
	let chunk_position: [i32; 3] = chunk::get_position(x, y, z);
	let block_position: [i32; 3] = [x - chunk_position[0]*CHUNK_SIZE, y - chunk_position[1]*CHUNK_SIZE, z - chunk_position[2]*CHUNK_SIZE];
	let block_id: i32 = block_position[0]+block_position[1]*CHUNK_SIZE+block_position[2]*CHUNK_SIZE*CHUNK_SIZE;
	
	let chunk: &mut chunk::Chunk = world.chunks.get_mut(&chunk_index).unwrap();
    match &chunk.volume {
        Some(shared_array_buffer) => {
            let volume_array: Uint8Array = Uint8Array::new(shared_array_buffer);
            volume_array.set_index(block_id as u32,block as u8);
        }        
        None => {
            log("Didn't find");
        }
    }
    mesh::greedy(chunk);

    log(&chunk_index.to_string());
}

pub fn consume_chunk_buffer(world: &mut World, chunk_index: i32, 
    vertice_buffer: SharedArrayBuffer,
    volume_buffer: SharedArrayBuffer) { 

    if !world.chunks.contains_key(&chunk_index) {
		chunk::create(world, chunk_index);
	}

    let chunk: &mut chunk::Chunk = world.chunks.get_mut(&chunk_index).unwrap();

    chunk.vertices = Some(vertice_buffer);
    chunk.volume = Some(volume_buffer);

}