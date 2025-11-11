use crate::shapes::*;
use macroquad::shapes::draw_rectangle;

pub struct Rectangle {
    x : f32
}

impl Rectangle {
    pub fn new() -> Self {
        Rectangle { x: -1.0 }
    }
}

impl IsShape for Rectangle {
    fn is_default(&self) -> bool {
        todo!()
    }

    fn is_in_progress(&self) -> bool {
        todo!()
    }

    fn return_shape(&self) -> Shape {
        todo!()
    }

    fn get_new_pos(&mut self, x : f32, y : f32) -> bool {
        todo!()
    }

    fn proto_draw(&self, x_mouse : f32, y_mouse : f32) {
        todo!()
    }

    fn draw(&self) {
        todo!()
    }

    fn distance_from(&self, x : f32, y : f32) -> f32 {
        todo!()
    }
}