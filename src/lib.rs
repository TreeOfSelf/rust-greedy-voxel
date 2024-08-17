use wasm_bindgen::prelude::*;
mod voxel;


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn mesh() -> Vec<i32> {
	let mut world: voxel::world::World = voxel::world::create();
    voxel::world::change_block(&mut world,0,0,0,5);
    world.chunks.get(&0).unwrap().mesh.clone()
}

