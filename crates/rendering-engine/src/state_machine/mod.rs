pub(crate) mod code_modifier;
pub(crate) mod font_modifier;

use crate::{
    barcode::{Code11, Codes, EAN13},
    command::Command,
    drawable::{Drawable, LineBox, TextField},
    modifier::Modifier,
    IntoZPL, ParamsExt, ParseError, ZPLEntry,
};
use image::{ImageBuffer, Rgb};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use std::{fmt::Debug, path::Path};

use self::{code_modifier::CodeSettings, font_modifier::FontSettings};

pub type DynDrawable<'a> = Box<dyn Drawable<'a, ImageBuffer<Rgb<u8>, Vec<u8>>> + 'a>;

pub struct CurrentState<'a> {
    pub reverse: bool,
    pub density: usize,
    pub origin: Origin,
    pub width: usize,
    pub height: usize,
    pub buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
    pub next_font: Option<FontSettings>,
    pub next_fd: FieldType,
    pub instructions: Vec<&'a str>,
    pub to_draw: Vec<DynDrawable<'a>>,
    pub text_modifier: FontSettings,
    pub code_modifier: CodeSettings,
}

impl<'a> CurrentState<'a> {
    pub fn process_entry(&mut self, entry: ZPLEntry<'a>) -> Result<(), ParseError> {
        match entry {
            ZPLEntry::Command(command) => self.process_command(command),
            ZPLEntry::Modifier(modifier) => self.process_modifier(modifier),
        }
    }

    fn process_command(&'_ mut self, entry: Command<'a>) -> Result<(), ParseError> {
        match entry {
            Command::FD(data) => {
                match self.next_fd.clone() {
                    FieldType::Text => self.add_to_draw_list(TextField::new(data.data, self)),
                    FieldType::Barcode(code) => {
                        self.add_to_draw_list(code.get_drawable(data.data, self))
                    }
                };
            }
            Command::GB(data) => self.add_to_draw_list(LineBox::new(data, self)),
        }
        Ok(())
    }
    fn process_modifier(&mut self, entry: Modifier) -> Result<(), ParseError> {
        match entry {
            Modifier::F0(data) => self.origin = data.into(),
            Modifier::A(data) => self.next_font = Some(data.into()),
            Modifier::CF(data) => {
                self.next_fd = FieldType::Text;
                self.next_font = None;
                self.text_modifier = data.into();
            }
            Modifier::B1(data) => {
                self.next_fd =
                    FieldType::Barcode(Codes::Code11(Code11::new(data.height.unwrap_or(300))))
            }
            Modifier::FR(_) => self.reverse = true,
            Modifier::BE(data) => {
                self.next_fd =
                    FieldType::Barcode(Codes::CodeEAN13(EAN13::new(data.height.unwrap_or(300))))
            }
        }
        Ok(())
    }

    fn add_to_draw_list<T: Drawable<'a, ImageBuffer<Rgb<u8>, Vec<u8>>> + 'a>(&mut self, item: T) {
        self.to_draw.push(Box::new(item));
        self.next_fd = FieldType::Text;
        self.reverse = false;
    }

    pub fn process_format(&mut self, format: &'a str) -> Result<(), ParseError> {
        let splitted = format.split(|a: char| a.eq(&'^') || a.eq(&'~'));
        for mut entry in splitted {
            if entry.is_empty() {
                continue;
            }

            entry = entry.trim_matches(' ').trim_matches('\n');
            self.instructions.push(entry);
            if entry.starts_with("FS") {
                continue;
            }

            if let Ok(entry) = entry.into_zpl() {
                self.process_entry(entry)?;
            }
        }

        Ok(())
    }

    pub fn to_zpl_code(&self) -> String {
        String::from("^")
            + &self
                .instructions
                .clone()
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join("\n^")
    }
}

impl<'a> CurrentState<'a> {
    pub fn new(density: usize, mut width: usize, mut height: usize) -> CurrentState<'a> {
        height *= density;
        width *= density;
        let buffer = ImageBuffer::new(width as _, height as _);
        Self {
            reverse: false,
            density,
            height,
            width,
            buffer,
            origin: Default::default(),
            next_fd: FieldType::Text,
            next_font: Default::default(),
            to_draw: Default::default(),
            instructions: Default::default(),
            text_modifier: FontSettings::default(),
            code_modifier: CodeSettings::default(),
        }
    }

    pub fn render(&mut self) {
        let img = &mut self.buffer;
        draw_filled_rect_mut(
            img,
            Rect::at(0, 0).of_size(img.width(), img.height()),
            Rgb([255, 255, 255]),
        );
        for i in self.to_draw.iter() {
            i.draw(img);
        }
    }

    pub fn to_png(&mut self, path: impl AsRef<Path>) {
        let img = &self.buffer;
        img.save_with_format(path, image::ImageFormat::Png).unwrap();
    }

    pub fn to_jpg(&mut self, path: impl AsRef<Path>) {
        let img = &self.buffer;
        img.save_with_format(path, image::ImageFormat::Jpeg)
            .unwrap();
    }
}

#[derive(Default, Debug)]
pub struct Origin {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    Text,
    Barcode(Codes),
}

#[derive(Debug, Clone, Copy)]
pub struct FontInfo {
    pub current_font: FontName,
    pub orientation: Orientation,
    pub height: u32,
    pub width: u32,
}

impl Default for FontInfo {
    fn default() -> Self {
        Self {
            current_font: 'A'.into(),
            orientation: Default::default(),
            height: 9,
            width: 5,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FontName(pub char);

impl From<char> for FontName {
    fn from(value: char) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Orientation {
    #[default]
    Normal,
    /// Rotated clockwise 90 degrees
    Rotated,
    /// Rotated 180 degrees
    Inverted,
    /// Rotated clockwise 270 degrees
    Bottom,
}

impl TryFrom<char> for Orientation {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let res = match value {
            'N' => Self::Normal,
            'R' => Self::Rotated,
            'I' => Self::Inverted,
            'B' => Self::Bottom,
            _ => Err(ParseError::UnsupportedCommand)?,
        };
        Ok(res)
    }
}

impl<'a> ParamsExt<'a> for FontName {
    type Error = ParseError;

    fn get_param(s: Option<&'a str>, param_name: &str) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let inner = s
            .ok_or(ParseError::MissingParameter(param_name.into()))?
            .trim_matches(' ')
            .chars()
            .next()
            .ok_or(ParseError::MissingParameter(param_name.into()))?;
        Ok(Self(inner))
    }
}

impl<'a> ParamsExt<'a> for Orientation {
    type Error = ParseError;

    fn get_param(s: Option<&'a str>, param_name: &str) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        s.ok_or(ParseError::MissingParameter(param_name.into()))?
            .trim_matches(' ')
            .chars()
            .next()
            .ok_or(ParseError::MissingParameter(param_name.into()))?
            .try_into()
    }
}
