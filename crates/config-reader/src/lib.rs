mod field_type;
mod label;
mod label_details;
mod label_field;
mod labelable;

pub use config_reader_macro::include_label_config;
pub use field_type::{Code, FieldType, TextFieldProperties};
pub use label::Label;
pub use label_details::LabelDetails;
pub use label_field::ItemField;
pub use labelable::Labelable;

#[cfg(test)]
mod test {
    use std::{fs::File, io::Write};

    use super::*;
    include_label_config!("CustomLabel2", "test_configs/basic_config.lbl");
    // cargo test --package config-reader --lib -- test::test_include_label_config --exact --nocapture
    #[test]
    fn test_include_label_config() {
        let my_label = CustomLabel2 {
            EAN_CODE: "82194689124".into(),
            INTERNAL_CODE: "120719237047".into(),
            DESCRIPTION: "My label description".into(),
            Additional_DATA: "".into(),
        };

        let zpl = my_label.to_label_text();
        println!("{zpl}");
        let res = my_label.to_label_img();
        let mut fd = File::create("./response.png").unwrap();
        fd.write_all(&res).unwrap();
    }
}

impl<T> Labelable for T
where
    for<'a> &'a T: Into<Label>,
{
    fn id(&self) -> String {
        let label: Label = Into::<Label>::into(self);
        label.id()
    }

    fn to_label_text(&self) -> String {
        let label: Label = Into::<Label>::into(self);
        label.to_label_text()
    }

    fn to_label_img(&self) -> Vec<u8> {
        let label: Label = Into::<Label>::into(self);
        label.to_label_img()
    }
}

pub fn to_label<'a, I>(label: &'a I) -> Label
where
    &'a I: Into<Label> + 'static,
{
    label.into()
}

pub fn to_label_text<'a, I>(label: &'a I) -> String
where
    &'a I: Into<Label> + 'static,
{
    let label: Label = label.into();
    label.to_label_text()
}
