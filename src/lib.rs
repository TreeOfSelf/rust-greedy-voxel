use wasm_bindgen::prelude::*;
mod voxel;


#[wasm_bindgen(js_namespace = console)]
extern {
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn mesh() -> Vec<i32> {
	let mut world: voxel::world::World = voxel::world::create();
    voxel::world::change_block(&mut world,0,0,0,1);
    voxel::world::change_block(&mut world,0,1,0,1);
    voxel::world::change_block(&mut world,2,1,0,1);
    voxel::world::change_block(&mut world,2,2,0,1);

    world.chunks.get(&0).unwrap().mesh.clone()
}

