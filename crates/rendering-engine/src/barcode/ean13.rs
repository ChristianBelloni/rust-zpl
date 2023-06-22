use image::{ImageBuffer, Rgb};

use crate::{
    drawable::{Drawable, EAN13Code},
    CurrentState,
};

#[derive(Debug, Clone)]
pub struct EAN13 {
    pub height: u32,
}

impl EAN13 {
    pub fn new(height: u32) -> Self {
        Self { height }
    }
    pub fn get_drawable<'a>(
        self,
        data: &'a str,
        state: &'_ CurrentState,
    ) -> impl Drawable<'a, ImageBuffer<Rgb<u8>, Vec<u8>>> + 'a {
        let height = self.height;

        EAN13Code::new(state.origin.x, state.origin.y, height, data)
    }
}
