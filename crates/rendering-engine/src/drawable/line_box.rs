use super::Drawable;
use crate::{command::GB, CurrentState, LineColor};
use image::{GenericImage, Pixel, Rgb};
use imageproc::{drawing::Canvas, rect::Rect};

#[derive(Debug)]
pub struct LineBox {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub thickness: u32,
    pub color: LineColor,
    pub rounding: u8,
    pub reversed: bool,
}

impl LineBox {
    pub fn new(data: GB, state: &'_ CurrentState<'_>) -> Self {
        Self {
            x: state.origin.x,
            y: state.origin.y,
            width: data.width,
            height: data.height,
            thickness: data.thickness,
            color: data.color,
            rounding: data.rounding,
            reversed: state.reverse,
        }
    }
}

impl<'a, G: GenericImage<Pixel = Rgb<u8>>> Drawable<'a, G> for LineBox {
    fn draw(&self, target: &mut G) {
        draw_hollow_rect_mut_with_thickness(
            target,
            Rect::at(self.x as _, self.y as _).of_size(self.width, self.height),
            match self.color {
                LineColor::Black => Rgb([0, 0, 0]),
                LineColor::White => Rgb([255, 255, 255]),
            },
            self.thickness,
            self.reversed,
        )
    }
}

pub fn draw_hollow_rect_mut_with_thickness<C>(
    canvas: &mut C,
    rect: Rect,
    color: C::Pixel,
    thickness: u32,
    reversed: bool,
) where
    C: Canvas<Pixel = Rgb<u8>>,
{
    let (left, top, width, height) = (
        rect.left() as u32,
        rect.top() as u32,
        rect.width(),
        rect.height(),
    );

    for x in left..(left + width) {
        for y in top..(top + height) {
            if (x < left + thickness || x > left + width - thickness)
                || (y < top + thickness || y > top + height - thickness)
            {
                if !reversed {
                    canvas.draw_pixel(x, y, color);
                } else {
                    let mut px = canvas.get_pixel(x, y);
                    px.invert();
                    canvas.draw_pixel(x, y, px);
                }
            }
        }
    }
}
