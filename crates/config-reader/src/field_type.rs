#[derive(Debug)]
pub enum FieldType {
    Text(TextFieldProperties),
    Code(Code),
    Image(String),
}

#[derive(Debug)]
pub enum Code {
    EAN13(String),
}

#[derive(Debug)]
pub struct TextFieldProperties {
    pub x_size: u64,
    pub y_size: u64,
    pub contents: String,
}

impl FieldType {
    pub fn to_label_field_text(&self) -> String {
        match &self {
            FieldType::Text(text) => {
                let TextFieldProperties {
                    x_size,
                    y_size,
                    contents,
                } = &text;
                format!("^ADN,{x_size},{y_size}^FD{contents}^FS")
            }

            FieldType::Code(code) => match code {
                Code::EAN13(data) => format!("^BY2\n^B2N, 150, N,N\n^FD{data}^FS"),
            },
            FieldType::Image(data) => format!("^GFA,8192,8192,32,,{data}^FS"),
        }
    }
}
