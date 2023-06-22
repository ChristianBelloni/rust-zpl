mod barcode;
mod command;
mod drawable;
mod error;
mod modifier;
mod params_ext;
mod state_machine;
mod zpl_parser;

use command::Command;
pub use error::ParseError;
use modifier::Modifier;
pub use params_ext::ParamsExt;
pub(crate) use state_machine::CurrentState;
pub use zpl_parser::ZPLParser;

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

    #[test]
    pub fn test_format() {
        _ = "^XA
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
        ^XZ";
    }
}

pub trait EntryExt<'a, T>: IntoZPL<'a, T> + FromZPL<'a, T> {}

impl<'a, T> EntryExt<'a, T> for &'a str
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

#[derive(Debug)]
pub enum LineColor {
    Black,
    White,
}

#[derive(Debug)]
pub enum ZPLBool {
    Yes,
    No,
}
