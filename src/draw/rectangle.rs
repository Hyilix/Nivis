use crate::color::Color;
use crate::canvas::Canvas;
use crate::geometry::rectangle::Rectangle;

// Draw a filled rectangle on a canvas with a given color
pub fn filled(surf: &mut Canvas, rect: &Rectangle, color: Color) {
    surf.fill_rect(rect, color);
}

// Draw an outline with the given width of a rectangle on a canvas with a given color
pub fn outlined(surf: &mut Canvas, rect: &Rectangle, color: Color, width: i16) {
    // TODO: Implement
}

// TODO: rounded rectangle corners
pub fn rounded_filled() {

}

pub fn rounded_outlined() {

}
