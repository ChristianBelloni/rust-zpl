use crate::Label;

pub trait Labelable {
    fn id(&self) -> String;
    fn to_label_text(&self) -> String;
    fn to_label_img(&self) -> Vec<u8>;
}

impl Labelable for Label {
    fn id(&self) -> String {
        let nxt = self
            .contents
            .iter()
            .find(|a| matches!(&a.contents, crate::FieldType::Code(_)))
            .unwrap_or_else(|| panic!("label should contain at least one code field"));
        match &nxt.contents {
            crate::FieldType::Code(code) => match code {
                crate::Code::EAN13(txt) => txt.clone(),
            },
            _ => panic!("id field should be a code"),
        }
    }

    fn to_label_text(&self) -> String {
        let inner = self
            .contents
            .iter()
            .map(|a| format!("{}\n", a.to_label_field_text()))
            .collect::<String>();
        format!("^XA\n{inner}\n^XZ")
    }

    fn to_label_img(&self) -> Vec<u8> {
        let client = reqwest::blocking::Client::builder().build().unwrap();
        let zpl = self.to_label_text();
        client
            .post("http://api.labelary.com/v1/printers/8dpmm/labels/4x6/0")
            .body(zpl)
            .send()
            .unwrap()
            .bytes()
            .unwrap()
            .into()
    }
}
