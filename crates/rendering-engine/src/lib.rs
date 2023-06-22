mod barcode;
mod command;
mod drawable;
mod error;
mod modifier;
mod state_machine;

use command::Command;
pub use error::ParseError;
use modifier::Modifier;
use std::{assert_eq, num::ParseIntError, println};

pub use state_machine::CurrentState;

pub fn render_to_bytes(format: &str) -> Result<(), ParseError> {
    let mut state = CurrentState::new(203, 4, 6);
    state.process_format(format)?;

    let binding = state.to_zpl_code();
    let reparsed = binding.replace([' ', '\n'], "");
    println!("{reparsed}");
    assert_eq!(format.replace([' ', '\n'], ""), reparsed);
    state.render();
    state.to_png("./file.png");
    state.to_jpg("./file2.jpeg");

    Ok(())
}

#[derive(Debug)]
pub enum ZPLEntry<'a> {
    Command(Command<'a>),
    Modifier(Modifier<'a>),
}

impl<'a> IntoZPL<'a, ZPLEntry<'a>> for &'a str {
    fn into_zpl(self) -> Result<ZPLEntry<'a>, ParseError> {
        let entry = self;
        Ok(match entry {
            _ if entry.starts_with("FO") => ZPLEntry::Modifier(Modifier::F0(entry.into_zpl()?)),
            _ if entry.starts_with("FD") => ZPLEntry::Command(Command::FD(entry.into_zpl()?)),
            _ if entry.starts_with('A') && !entry.starts_with("A@") => {
                ZPLEntry::Modifier(Modifier::A(entry.into_zpl()?))
            }
            _ if entry.starts_with("CF") => ZPLEntry::Modifier(Modifier::CF(entry.into_zpl()?)),
            _ if entry.starts_with("GB") => ZPLEntry::Command(Command::GB(entry.into_zpl()?)),
            _ if entry.starts_with("FR") => ZPLEntry::Modifier(Modifier::FR(entry.into_zpl()?)),
            _ if entry.starts_with("B1") => ZPLEntry::Modifier(Modifier::B1(entry.into_zpl()?)),
            _ if entry.starts_with("BE") => ZPLEntry::Modifier(Modifier::BE(entry.into_zpl()?)),
            _ => Err(ParseError::UnsupportedCommand)?,
        })
    }
}

impl<'a> FromZPL<'a, ZPLEntry<'a>> for &'a str {
    fn from_zpl(value: &'a ZPLEntry<'a>) -> Self {
        match value {
            ZPLEntry::Command(data) => Self::from_zpl(data),
            ZPLEntry::Modifier(data) => Self::from_zpl(data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_format() {
        _ = render_to_bytes(
            "^XA
^FX Top section with logo, name and address.
^CF0,60
^FO50,50^GB100,100,100^FS
^FO75,75^FR^GB100,100,100^FS
^FO93,93^GB40,40,40^FS
^FO220,50^FDIntershipping, Inc.^FS
^CF0,30
^FO220,115^FD1000 Shipping Lane^FS
^FO220,155^FDShelbyville TN 38102^FS
^FO220,195^FDUnited States (USA)^FS
^FO50,250^GB700,3,3^FS

^FX Second section with recipient address and permit information.
^CFA,30
^FO50,300^FDJohn Doe^FS
^FO50,340^FD100 Main Street^FS
^FO50,380^FDSpringfield TN 39021^FS
^FO50,420^FDUnited States (USA)^FS
^CFA,15
^FO600,300^GB150,150,3^FS
^FO638,340^FDPermit^FS
^FO638,390^FD123456^FS
^FO50,500^GB700,3,3^FS

^FX Third section with bar code.
^BY5,2,270
^FO100,550^BE^FD123456789111^FS

^FX Fourth section (the two boxes on the bottom).
^FO50,900^GB700,250,3^FS
^FO400,900^GB3,250,3^FS
^CF0,40
^FO100,960^FDCtr. X34B-1^FS
^FO100,1010^FDREF1 F00B47^FS
^FO100,1060^FDREF2 BL4H8^FS
^CF0,190
^FO470,955^FDCA^FS
        ^XZ",
        );
    }
}

pub trait ZPLExt<'a, T>: IntoZPL<'a, T> + FromZPL<'a, T> {}

impl<'a, T> ZPLExt<'a, T> for &'a str
where
    Self: IntoZPL<'a, T>,
    Self: FromZPL<'a, T>,
{
}

pub trait IntoZPL<'a, T> {
    fn into_zpl(self) -> Result<T, ParseError>;
}

pub trait FromZPL<'a, T> {
    fn from_zpl(value: &'a T) -> Self;
}

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

#[derive(Debug)]
pub enum LineColor {
    Black,
    White,
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

#[derive(Debug)]
pub enum ZPLBool {
    Yes,
    No,
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
