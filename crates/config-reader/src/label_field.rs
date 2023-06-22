use crate::field_type::FieldType;

#[derive(Debug)]
pub struct ItemField {
    pub placeholder: String,
    pub x_pos: u64,
    pub y_pos: u64,
    pub contents: FieldType,
}

impl ItemField {
    pub fn to_label_field_text(&self) -> String {
        let x_pos = self.x_pos;
        let y_pos = self.y_pos;
        let inner = self.contents.to_label_field_text();
        format!("^FO{x_pos},{y_pos}{inner}")
    }
}
