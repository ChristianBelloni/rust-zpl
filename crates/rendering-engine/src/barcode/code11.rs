use image::{ImageBuffer, Rgb};

use crate::{
    drawable::{B1Code, Drawable},
    CurrentState,
};

#[derive(Debug, Clone)]
pub struct Code11 {
    pub height: u32,
}

impl Code11 {
    pub fn new(height: u32) -> Self {
        Self { height }
    }
    pub fn get_drawable<'a>(
        self,
        data: &'a str,
        state: &'_ CurrentState,
    ) -> impl Drawable<'a, ImageBuffer<Rgb<u8>, Vec<u8>>> + 'a {
        let height = self.height;

        B1Code::new(state.origin.x, state.origin.y, height, data)
    }
}
