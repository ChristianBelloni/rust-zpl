use crate::{error::ParseError, state_machine::Origin};
use crate::{FromZPL, IntoZPL, ParamsExt};

#[derive(Debug)]
pub struct FO<'a> {
    pub original_format: &'a str,
    pub x: u32,
    pub y: u32,
}

impl<'a> IntoZPL<'a, FO<'a>> for &'a str {
    fn into_zpl(self) -> Result<FO<'a>, ParseError> {
        let value = &self[2..];
        let mut splitted = value.split(',');
        let x = u32::get_param(splitted.next(), "x")?;
        let y = u32::get_param(splitted.next(), "y")?;
        Ok(FO {
            x,
            y,
            original_format: self,
        })
    }
}

impl<'a> FromZPL<'a, FO<'a>> for &'a str {
    fn from_zpl(value: &'a FO<'a>) -> Self {
        value.original_format
    }
}

impl From<FO<'_>> for Origin {
    fn from(value: FO) -> Self {
        let FO {
            x,
            y,
            original_format: _,
        } = value;
        Self { x, y }
    }
}
