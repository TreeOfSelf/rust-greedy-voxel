use crate::voxel::CHUNK_SIZE;
use crate::voxel::CHUNK_SPACE;
use crate::voxel::world::*;
use js_sys::SharedArrayBuffer;

pub struct Chunk {
    pub volume: [i32; CHUNK_SIZE as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize],
    pub dims: [usize; 3],
    pub vertices: Option<SharedArrayBuffer>
}

impl Default for Chunk {
    fn default() -> Chunk {
        Chunk {
            volume: [0; CHUNK_SIZE as usize * CHUNK_SIZE as usize * CHUNK_SIZE as usize],
			dims: [CHUNK_SIZE as usize, CHUNK_SIZE as usize, CHUNK_SIZE as usize],
            vertices: None,
        }
    }
}

pub fn get_index(x: i32, y: i32, z: i32) -> i32 {
    (x/CHUNK_SIZE)+(y/CHUNK_SIZE)*CHUNK_SPACE+(z/CHUNK_SIZE)*CHUNK_SPACE*CHUNK_SPACE
}

pub fn get_position(x: i32, y: i32, z: i32) -> [i32; 3] {
	[x/CHUNK_SIZE, y/CHUNK_SIZE, z/CHUNK_SIZE]
}

pub fn create(world: &mut World, chunk_index: i32) -> &mut World {
	world.chunks.insert(chunk_index, Chunk::default());
	world
}

