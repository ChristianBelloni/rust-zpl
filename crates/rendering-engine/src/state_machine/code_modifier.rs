#[derive(Debug)]
pub struct CodeSettings {
    pub width: u32,
    pub height: u32,
    pub ration: f32,
}

impl Default for CodeSettings {
    fn default() -> Self {
        Self {
            width: 2,
            height: 10,
            ration: 3.0,
        }
    }
}
