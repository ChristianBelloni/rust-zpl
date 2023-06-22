use std::num::ParseIntError;

use crate::{LineColor, ParseError, ZPLBool};

pub trait ParamsExt<'a> {
    type Error: Into<ParseError>;
    fn get_param(s: Option<&'a str>, param_name: &str) -> Result<Self, ParseError>
    where
        Self: Sized;
}

impl<'a> ParamsExt<'a> for u32 {
    type Error = ParseIntError;

    fn get_param(s: Option<&'a str>, param_name: &str) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        Ok(s.ok_or(ParseError::MissingParameter(param_name.into()))?
            .trim_matches(' ')
            .parse()?)
    }
}

impl<'a> ParamsExt<'a> for u8 {
    type Error = ParseIntError;

    fn get_param(s: Option<&'a str>, param_name: &str) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        Ok(s.ok_or(ParseError::MissingParameter(param_name.into()))?
            .trim_matches(' ')
            .parse()?)
    }
}

impl<'a> ParamsExt<'a> for LineColor {
    type Error = ParseError;

    fn get_param(s: Option<&'a str>, param_name: &str) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let val = s
            .ok_or(ParseError::MissingParameter(param_name.into()))?
            .trim_matches(' ');
        let val = match val {
            "B" => Self::Black,
            "W" => Self::White,
            _ => Err(ParseError::UnsupportedCommand)?,
        };
        Ok(val)
    }
}

impl<'a> ParamsExt<'a> for ZPLBool {
    type Error = ParseError;

    fn get_param(s: Option<&'a str>, param_name: &str) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let val = s
            .ok_or(ParseError::MissingParameter(param_name.into()))?
            .trim_matches(' ');
        let val = match val {
            "Y" => Self::Yes,
            "N" => Self::No,
            _ => Err(ParseError::UnsupportedCommand)?,
        };
        Ok(val)
    }
}
