mod fd;
mod gb;

use self::fd::FD;
use crate::FromZPL;

#[derive(Debug)]
pub enum Command<'a> {
    FD(FD<'a>),
    GB(GB<'a>),
}

pub(crate) use gb::GB;

impl<'a> FromZPL<'a, Command<'a>> for &'a str {
    fn from_zpl(value: &'a Command<'a>) -> Self {
        match value {
            Command::FD(data) => Self::from_zpl(data),
            Command::GB(data) => Self::from_zpl(data),
        }
    }
}
