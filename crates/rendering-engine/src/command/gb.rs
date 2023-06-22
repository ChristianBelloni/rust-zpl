use crate::FromZPL;
use crate::IntoZPL;
use crate::LineColor;
use crate::ParamsExt;
use crate::ParseError;

#[derive(Debug)]
pub struct GB<'a> {
    pub original_format: &'a str,
    pub width: u32,
    pub height: u32,
    pub thickness: u32,
    pub color: LineColor,
    pub rounding: u8,
}

impl<'a> IntoZPL<'a, GB<'a>> for &'a str {
    fn into_zpl(self) -> Result<GB<'a>, ParseError> {
        let value = &self[2..];
        let mut splitted = value.split(',');
        let width = u32::get_param(splitted.next(), "width")?;
        let height = u32::get_param(splitted.next(), "height")?;
        let thickness = u32::get_param(splitted.next(), "thickness")?;
        let color = LineColor::get_param(splitted.next(), "color").unwrap_or(LineColor::Black);
        let rounding = u8::get_param(splitted.next(), "rounding").unwrap_or(0);

        Ok(GB {
            original_format: self,
            width,
            height,
            thickness,
            color,
            rounding,
        })
    }
}

impl<'a> FromZPL<'a, GB<'a>> for &'a str {
    fn from_zpl(value: &'a GB<'a>) -> Self {
        value.original_format
    }
}
