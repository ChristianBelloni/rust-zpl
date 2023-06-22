use crate::{state_machine::Orientation, FromZPL, IntoZPL, ParamsExt, ParseError, ZPLBool};

#[derive(Debug)]
pub struct BE<'a> {
    pub original_format: &'a str,
    pub orientation: Option<Orientation>,
    pub check_digit: Option<ZPLBool>,
    pub height: Option<u32>,
    pub interpretation_line: Option<ZPLBool>,
    pub interpretation_above_code: Option<ZPLBool>,
}

impl<'a> IntoZPL<'a, BE<'a>> for &'a str {
    fn into_zpl(self) -> Result<BE<'a>, ParseError> {
        let value = &self[2..];
        let mut splitted = value.split(',');
        let orientation = Orientation::get_param(splitted.next(), "orientation").ok();
        let check_digit = ZPLBool::get_param(splitted.next(), "check digit").ok();
        let height = u32::get_param(splitted.next(), "height").ok();
        let interpretation_line = ZPLBool::get_param(splitted.next(), "interpretation line").ok();
        let interpretation_above_code = ZPLBool::get_param(splitted.next(), "above code").ok();
        Ok(BE {
            original_format: self,
            check_digit,
            height,
            interpretation_above_code,
            interpretation_line,
            orientation,
        })
    }
}

impl<'a> FromZPL<'a, BE<'a>> for &'a str {
    fn from_zpl(value: &'a BE<'a>) -> Self {
        value.original_format
    }
}
