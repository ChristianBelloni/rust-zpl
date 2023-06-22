mod a;
mod b1;
mod be;
mod cf;
mod fo;
mod fr;

use crate::FromZPL;

pub use self::fo::FO;
use self::{a::A, b1::B1, be::BE, cf::CF, fr::FR};

#[derive(Debug)]
pub enum Modifier<'a> {
    F0(FO<'a>),
    A(A<'a>),
    CF(CF<'a>),
    FR(FR<'a>),
    B1(B1<'a>),
    BE(BE<'a>),
}

impl<'a> FromZPL<'a, Modifier<'a>> for &'a str {
    fn from_zpl(value: &'a Modifier<'a>) -> Self
    where
        Modifier<'a>: 'a,
    {
        match value {
            Modifier::F0(data) => Self::from_zpl(data),
            Modifier::A(data) => Self::from_zpl(data),
            Modifier::CF(data) => Self::from_zpl(data),
            Modifier::FR(data) => Self::from_zpl(data),
            Modifier::B1(data) => Self::from_zpl(data),
            Modifier::BE(data) => Self::from_zpl(data),
        }
    }
}
