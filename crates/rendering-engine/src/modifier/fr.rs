use crate::{FromZPL, IntoZPL, ParseError};

#[derive(Debug)]
pub struct FR<'a> {
    pub original_format: &'a str,
}

impl<'a> IntoZPL<'a, FR<'a>> for &'a str {
    fn into_zpl(self) -> Result<FR<'a>, ParseError> {
        Ok(FR {
            original_format: self,
        })
    }
}

impl<'a> FromZPL<'a, FR<'a>> for &'a str {
    fn from_zpl(value: &'a FR<'a>) -> Self {
        value.original_format
    }
}
