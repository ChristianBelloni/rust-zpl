use crate::drawable::{draw_encoding, Drawable};
use barcode_rs::BarcodeFormat;
use image::{ImageBuffer, Rgb};

#[derive(Debug)]
pub struct EAN13Code<'a> {
    pub x: u32,
    pub y: u32,
    pub height: u32,
    pub data: &'a str,
}

impl<'a> EAN13Code<'a> {
    #[allow(dead_code)]
    pub fn new(x: u32, y: u32, height: u32, data: &'a str) -> EAN13Code<'a> {
        Self { x, y, height, data }
    }
}

impl<'a> Drawable<'a, ImageBuffer<Rgb<u8>, Vec<u8>>> for EAN13Code<'a> {
    fn draw(&self, target: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let encoded = self.data.chars().collect::<Vec<char>>();
        let encoding = barcode_rs::encode(&encoded, BarcodeFormat::EAN13)
            .unwrap()
            .into_iter()
            .map(|a| if a { '1' } else { '0' })
            .collect::<String>();

        println!("encoding {encoding}");
        let scaling = 5.0;
        let x = self.x;
        let y = self.y as _;
        let height = self.height as f32;

        draw_encoding(x, y, height, scaling, encoding, target);
    }
}
