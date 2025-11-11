pub trait IsShape {
    fn is_default(&self) -> bool;
    fn is_in_progress(&self) -> bool;

    fn return_shape(&self) -> Shape;
    ///Return if there are enough pos for the shape
    fn get_new_pos(&mut self, x : f32, y : f32) -> bool;

    fn proto_draw(&self, x_mouse : f32, y_mouse : f32);
    fn draw(&self);

    fn distance_from(&self, x : f32, y : f32) -> f32;
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Shape {
    Circle,
    Rectangle,
}