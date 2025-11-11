use crate::shapes::*;
use macroquad::{color::WHITE, shapes::draw_rectangle};

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

        self.width = self.x - x.abs();
        self.height = self.y - y.abs();

        if self.x > x { self.x = x }
        if self.y < y { self.y = y }

        return true;
    }

    fn proto_draw(&self, x_mouse : f32, y_mouse : f32) {

        let x = f32::max(self.x, x_mouse);
        let y = f32::min(self.y, y_mouse);


        let width = self.x - x_mouse.abs();
        let height = self.y - y_mouse.abs();

        draw_rectangle(x, y, width, height, WHITE);

    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, WHITE);
    }

    fn distance_from(&self, x : f32, y : f32) -> f32 {
        todo!()
    }
}