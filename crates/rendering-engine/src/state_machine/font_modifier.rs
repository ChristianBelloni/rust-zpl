use super::{FontName, Orientation};

#[derive(Debug, Clone)]
pub struct FontSettings {
    pub current_font: FontName,
    pub orientation: Orientation,
    pub height: u32,
    pub width: u32,
}

impl Default for FontSettings {
    fn default() -> Self {
        Self {
            current_font: FontName('A'),
            orientation: Orientation::Normal,
            height: 9,
            width: 5,
        }
    }
}
