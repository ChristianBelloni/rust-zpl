mod code11;
mod ean13;
use crate::{drawable::CodeDrawable, CurrentState};

#[derive(Debug, Clone)]
pub enum Codes {
    Code11(Code11),
    CodeEAN13(EAN13),
}

pub use self::code11::Code11;
pub use self::ean13::EAN13;

impl Codes {
    pub fn get_drawable<'a>(self, data: &'a str, state: &'_ CurrentState) -> CodeDrawable<'a> {
        match self {
            Codes::Code11(code) => CodeDrawable::new(code.get_drawable(data, state)),
            Codes::CodeEAN13(code) => CodeDrawable::new(code.get_drawable(data, state)),
        }
    }
}
