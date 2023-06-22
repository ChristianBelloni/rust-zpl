use crate::{
    state_machine::{font_modifier::FontSettings, FontName},
    FromZPL, IntoZPL, ParamsExt, ParseError,
};

#[derive(Debug, Clone)]
pub struct CF<'a> {
    pub original_format: &'a str,
    pub font_name: FontName,
    pub height: u32,
    pub width: u32,
}

impl<'a> IntoZPL<'a, CF<'a>> for &'a str {
    fn into_zpl(self) -> Result<CF<'a>, ParseError> {
        let value = &self[2..];
        let mut splitted = value.split(',');
        let font_name = FontName::get_param(splitted.next(), "font name")?;
        let height = u32::get_param(splitted.next(), "height")?;
        let width = u32::get_param(splitted.next(), "width").unwrap_or(height);
        Ok(CF {
            font_name,
            height,
            width,
            original_format: self,
        })
    }
}

impl<'a> FromZPL<'a, CF<'a>> for &'a str {
    fn from_zpl(value: &'a CF<'a>) -> Self {
        value.original_format
    }
}

impl<'a> From<CF<'a>> for FontSettings {
    fn from(val: CF) -> Self {
        FontSettings {
            current_font: val.font_name,
            orientation: Default::default(),
            height: val.height,
            width: val.width,
        }
    }
}
