use crate::{
    state_machine::{font_modifier::FontSettings, FontName, Orientation},
    FromZPL, IntoZPL, ParamsExt, ParseError,
};

#[derive(Debug)]
pub struct A<'a> {
    pub original_format: &'a str,
    pub font_name: FontName,
    pub orientation: Orientation,
    pub height: u32,
    pub width: u32,
}

impl<'a> IntoZPL<'a, A<'a>> for &'a str {
    fn into_zpl(self) -> Result<A<'a>, ParseError> {
        let value = &self[1..];
        let mut splitted = value.split(',');
        let font_name = FontName::get_param(splitted.next(), "font name")?;
        let orientation = Orientation::get_param(splitted.next(), "font orientation")?;
        let height = u32::get_param(splitted.next(), "height")?;
        let width = u32::get_param(splitted.next(), "width").unwrap_or(height);
        Ok(A {
            font_name,
            orientation,
            height,
            width,
            original_format: self,
        })
    }
}

impl<'a> FromZPL<'a, A<'a>> for &'a str {
    fn from_zpl(value: &'a A<'a>) -> Self {
        value.original_format
    }
}

impl<'a> From<A<'a>> for FontSettings {
    fn from(val: A) -> Self {
        FontSettings {
            current_font: val.font_name,
            orientation: val.orientation,
            height: val.height,
            width: val.width,
        }
    }
}
