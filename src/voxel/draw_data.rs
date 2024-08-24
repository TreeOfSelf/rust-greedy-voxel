pub struct DrawData {
    pub vertices: Vec<i32>,
    pub indices: Vec<u16>,
}

impl Default for DrawData {
    fn default() -> DrawData {
        DrawData {
            vertices: Vec::new(),
			indices: Vec::new(),
        }
    }
}