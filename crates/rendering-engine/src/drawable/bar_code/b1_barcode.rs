use barcode_rs::BarcodeFormat;
use image::{ImageBuffer, Rgb};

use crate::drawable::{draw_encoding, Drawable};

#[derive(Debug)]
pub struct B1Code<'a> {
    pub x: u32,
    pub y: u32,
    pub height: u32,
    pub data: &'a str,
}

impl<'a> B1Code<'a> {
    pub fn new(x: u32, y: u32, height: u32, data: &'a str) -> B1Code<'a> {
        Self { x, y, height, data }
    }
}

impl<'a> Drawable<'a, ImageBuffer<Rgb<u8>, Vec<u8>>> for B1Code<'a> {
    fn draw(&self, target: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let chars = self.data.chars();

        let encoded =
            barcode_rs::encode(&chars.collect::<Vec<_>>(), BarcodeFormat::Code11).unwrap();
        let encoding = encoded
            .into_iter()
            .map(|a| if a { '1' } else { '0' })
            .collect::<String>();
        let scaling = 5.0;
        let x = self.x;
        let y = self.y as _;
        let height = self.height as f32;

        draw_encoding(x, y, height, scaling, encoding, target);
    }
}
