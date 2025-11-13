use crate::shapes::*;
use macroquad::shapes::draw_rectangle_lines;

pub struct Rectangle {

    // Bottom-left
    x : f32,
    y : f32,

    // Top-right
    width : f32,
    height : f32
}

impl Rectangle {
    pub fn new() -> Self {
        Self { x: -1.0, y: -1.0, width: -1.0, height: -1.0 }
    }
}

impl IsShape for Rectangle {
    fn is_default(&self) -> bool {
        self.x == -1.0 && self.y == -1.0 && self.width == -1.0 && self.height == -1.0
    }

    fn is_in_progress(&self) -> bool {
        todo!()
    }

    fn return_shape(&self) -> Shape {
        Shape::Rectangle
    }

    fn get_new_pos(&mut self, x : f32, y : f32) -> bool {
        if self.is_default() {
            self.x = x;
            self.y = y;

            return false;
        }

        let left = f32::min(self.x, x);
        let right = f32::max(self.x, x);
        let top = f32::min(self.y, y);
        let bottom = f32::max(self.y, y);

        self.x = left;
        self.y = top;

        self.width = right - left;
        self.height = bottom - top;

        return true;
    }

    fn proto_draw(&self, x_mouse : f32, y_mouse : f32) {

        let left = f32::min(self.x, x_mouse);
        let right = f32::max(self.x, x_mouse);
        let top = f32::min(self.y, y_mouse);
        let bottom = f32::max(self.y, y_mouse);

        let width = right - left;
        let height = bottom - top;

        draw_rectangle_lines(left, top, width, height, 2.0, WHITE);
    }

    fn draw(&self) {
        draw_rectangle_lines(self.x, self.y, self.width, self.height, 2.0, WHITE);
    }

    fn distance_from(&self, x : f32, y : f32) -> f32 {
        todo!()
    }
}