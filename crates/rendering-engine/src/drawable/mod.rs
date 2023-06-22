mod bar_code;
mod line_box;
mod text_field;
use image::{GenericImage, Rgb};
use std::fmt::Debug;

pub trait Drawable<'a, G: GenericImage<Pixel = Rgb<u8>>>: Debug {
    fn draw(&self, target: &mut G);
}

pub use bar_code::*;
pub use line_box::LineBox;
pub use text_field::TextField;
