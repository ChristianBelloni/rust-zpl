use crate::{state_machine::font_modifier::FontSettings, CurrentState};
use image::{GenericImage, Rgb};
use imageproc::drawing::draw_text_mut;
use lazy_static::lazy_static;
use rusttype::{Font, Scale};

lazy_static! {
    // pub static ref FONT_DATA: Vec<u8> =
    //     andrew::text::load_font_file("/Library/Fonts/Arial Unicode.ttf");
}

use super::Drawable;

#[derive(Debug)]
pub struct TextField<'a> {
    pub x: u32,
    pub y: u32,
    pub font: FontSettings,
    pub contents: &'a str,
}

impl<'a> TextField<'a> {
    pub fn new(data: &'a str, state: &'_ CurrentState<'_>) -> Self {
        let font = state
            .next_font
            .as_ref()
            .unwrap_or(&state.text_modifier)
            .clone();
        Self {
            x: state.origin.x,
            y: state.origin.y,
            font,
            contents: data,
        }
    }
}

impl<'a, G: GenericImage<Pixel = Rgb<u8>>> Drawable<'a, G> for TextField<'a> {
    /// `TODO! make sure proportions are mantained when converting zpl units in pixels`
    fn draw(&self, target: &mut G) {
        let font_data: &[u8] = include_bytes!("/Library/fonts/Arial Unicode.ttf");
        let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();
        draw_text_mut(
            target,
            Rgb::<u8>([0, 0, 0]),
            self.x as _,
            self.y as _,
            Scale {
                x: self.font.width as _,
                y: self.font.height as _,
            },
            &font,
            self.contents,
        );
    }
}
