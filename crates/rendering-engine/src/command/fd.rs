use crate::FromZPL;
use crate::IntoZPL;
use crate::ParseError;

#[derive(Debug)]
pub struct FD<'a> {
    pub original_format: &'a str,
    pub data: &'a str,
}

impl<'a> IntoZPL<'a, FD<'a>> for &'a str {
    fn into_zpl(self) -> Result<FD<'a>, ParseError> {
        let value = &self[2..];
        Ok(FD {
            original_format: self,
            data: value,
        })
    }
}

impl<'a> FromZPL<'a, FD<'a>> for &'a str {
    fn from_zpl(value: &'a FD<'a>) -> Self {
        value.original_format
    }
}
