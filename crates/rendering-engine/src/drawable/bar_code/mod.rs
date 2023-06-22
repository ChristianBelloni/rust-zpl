mod b1_barcode;
mod ean13_barcode;
pub use b1_barcode::B1Code;
pub use ean13_barcode::EAN13Code;
use image::{ImageBuffer, Rgb};
use imageproc::drawing::draw_line_segment_mut;

use super::Drawable;

#[derive(Debug)]
pub struct CodeDrawable<'a> {
    inner: Box<dyn Drawable<'a, ImageBuffer<Rgb<u8>, Vec<u8>>> + 'a>,
}

impl<'a> CodeDrawable<'a> {
    pub fn new(inner: impl Drawable<'a, ImageBuffer<Rgb<u8>, Vec<u8>>> + 'a) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl<'a> Drawable<'a, ImageBuffer<Rgb<u8>, Vec<u8>>> for CodeDrawable<'a> {
    fn draw(&self, target: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        self.inner.draw(target);
    }
}

pub fn draw_encoding(
    x: u32,
    y: f32,
    height: f32,
    scaling: f32,
    encoding: String,
    target: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
) {
    let mut pos = x as f32;
    for c in encoding.chars() {
        if c == '1' {
            for _ in 0..(scaling as i32) {
                draw_line_segment_mut(target, (pos, y), (pos, y + height), Rgb([0, 0, 0]));
                pos += 1.0;
            }
        } else {
            pos += scaling;
        }
    }
}
