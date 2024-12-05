use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
mod voxel;
use js_sys::SharedArrayBuffer;

thread_local! {
    static WORLD: RefCell<Option<Rc<RefCell<voxel::world::World>>>> = const { RefCell::new(None) };
}

#[wasm_bindgen(js_namespace = console)]
extern {
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn initialize_world() {
    WORLD.with(|world| {
        *world.borrow_mut() = Some(Rc::new(RefCell::new(voxel::world::create())));
    });
}

#[wasm_bindgen]
pub fn mesh() {
    WORLD.with(|world| {
        if let Some(world) = world.borrow().as_ref() {
            let mut world = world.borrow_mut();
            voxel::world::change_block(&mut world, 0, 0, 0, 1);
            voxel::world::change_block(&mut world, 0, 1, 0, 1);
            voxel::world::change_block(&mut world, 2, 1, 0, 1);
            voxel::world::change_block(&mut world, 2, 2, 0, 1);
            voxel::world::change_block(&mut world, 20, 20, 0, 1);

        } else {
            log("World not initialized");
        }
    });
}

#[wasm_bindgen]
pub fn starter_block() {
    WORLD.with(|world| {
        if let Some(world) = world.borrow().as_ref() {
            let mut world = world.borrow_mut();
            /*voxel::world::change_block(&mut world, 0, 0, 0, 4);
            voxel::world::change_block(&mut world, 0, 1, 0, 3);
            voxel::world::change_block(&mut world, 0, 2, 0, 2);
            */
            voxel::world::change_block(&mut world, 0, 3, 0, 1);
            voxel::world::change_block(&mut world, 1, 3, 0, 1);
            voxel::world::change_block(&mut world, 1, 3, 1, 1);
            voxel::world::change_block(&mut world, 0, 3, 1, 1);


        } else {
            log("World not initialized");
        }
    });  
}


#[wasm_bindgen]
pub fn consume_chunk_buffers(chunk_index: i32, 
    vertice_buffer: SharedArrayBuffer, 
    volume_buffer: SharedArrayBuffer,
    block_type_buffer: SharedArrayBuffer,
    texture_coordinates_buffer: SharedArrayBuffer,
) {
  WORLD.with(|world| {
        if let Some(world) = world.borrow().as_ref() {
            let mut world = world.borrow_mut();
            voxel::world::consume_chunk_buffer(&mut world, chunk_index, 
                vertice_buffer, 
                volume_buffer,
                block_type_buffer,
                texture_coordinates_buffer);
        } else {
            log("World not initialized");
        }
    });

}

