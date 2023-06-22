use crate::{ItemField, LabelDetails};

#[derive(Debug)]
pub struct Label {
    pub name: String,
    pub details: LabelDetails,
    pub contents: Vec<ItemField>,
}
