use crate::shapes::*;
use macroquad::{color::WHITE, shapes::{draw_circle, draw_circle_lines}};

// x & y are coordinate of the center of the circle
pub struct Circle {
    x : f32,
    y : f32,
    radius : f32
}

impl Circle {
    pub fn new() -> Self {
        Self { x: -1.0, y: -1.0, radius: -1.0 }
    }
}

impl IsShape for Circle {

    fn is_default(&self) -> bool {
        return self.x == -1.0 && self.y == -1.0 && self.radius == -1.0;
    }

    fn is_in_progress(&self) -> bool {
        return self.radius == -1.0;
    }

    fn return_shape(&self) -> Shape {
        Shape::Circle
    }

    fn proto_draw(&self, pos_x_mouse : f32, pos_y_mouse : f32) {
        let dx = self.x - pos_x_mouse;
        let dy = self.y - pos_y_mouse;

        let radius = dx.hypot(dy);


        draw_circle_lines(self.x, self.y, radius, 1.0, WHITE);
    }

    fn get_new_pos(&mut self, x : f32, y : f32) -> bool {
        if self.is_default() {
            self.x = x;
            self.y = y;
            return false;
        }

        let dx = self.x - x;
        let dy = self.y - y;

        self.radius = dx.hypot(dy);

        return true;
    }

    fn draw(&self) {
        draw_circle_lines(self.x, self.y, self.radius, 1.0, WHITE);
    }

    fn distance_from(&self, x : f32, y : f32) -> f32 {

        if self.is_in_progress() {
            panic!("Distance par rapport au cercle indéterminable");
        }

        let dx = self.x - x;
        let dy = self.y - y;

        return dx.hypot(dy) - self.radius;
    }
    
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_get_new_pos() {
        let mut circle = Circle::new();

        assert_eq!(circle.get_new_pos(0.0, 1.0), false);
        assert_eq!(circle.get_new_pos(0.0, 7.0), true);
    }

}