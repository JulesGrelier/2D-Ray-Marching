use std::fmt::Debug;

use macroquad::input::KeyCode;

use crate::shape::{IsShape, Shape};
use crate::shapes::{circle::Circle, rectangle::Rectangle};

pub struct Input {
    x : f32,
    y : f32,
    pub shape : Shape
}

#[derive(Debug, PartialEq)]
pub enum Action {
    CreateNewShape,
    AddNewPos,
}

type DynShape = Box<dyn IsShape>;

pub struct Game {
    pub shapes : Vec<DynShape>,
    proto_shape : Option<DynShape>,
    selected_shape : Option<Shape>,
}

impl Game {
    pub fn new() -> Self {
        Self { shapes: Vec::new(), proto_shape: None, selected_shape: None }
    }



    pub fn draw(&self, mouse : (f32, f32)) {
        for shape in &self.shapes {
            shape.draw();
        }
        if let Some(shape) = &self.proto_shape {
            shape.proto_draw(mouse.0, mouse.1);
        }
    }



    pub fn modify_selected_key(&mut self, key : KeyCode) {
        let shape = match key {
            KeyCode::C => Shape::Circle,
            KeyCode::R => Shape::Rectangle,
            _ => return,
        };

        self.selected_shape = Some(shape)
    }



    pub fn create_input(&self, x_mouse : f32, y_mouse : f32) -> Option<Input> {
        // [selected_shape] already have the selected_shape, but it's better to centralize infos in one struct
        return self.selected_shape.map(|shape| Input { x: x_mouse, y: y_mouse, shape });
    }



    pub fn return_action(&self, shape : Shape) -> Action {

        let same_shape = self.proto_shape.as_ref().map_or( false, 
            |x| x.return_shape() == shape
        );

        if same_shape { return Action::AddNewPos; } else { return Action::CreateNewShape; }
    }



    fn concretise_shape(&self, shape : Shape) -> DynShape {
        match shape {
            Shape::Circle => { return Box::new(Circle::new()); },
            Shape::Rectangle => { return Box::new(Rectangle::new()); },
        };
    }



    pub fn get_new_pos(&mut self, input : &Input, action : Action) {
        if action == Action::CreateNewShape {
            self.proto_shape = Some(self.concretise_shape(input.shape));
        }

        let enough_pos = self.proto_shape.as_mut().unwrap().get_new_pos(input.x, input.y);
        if enough_pos { self.move_proto_shape(); }
    }


    fn move_proto_shape(&mut self) {
        let young_shape = self.proto_shape.take().unwrap();
        self.shapes.push(young_shape);
    }
}


// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn check_response_to_input_shape_with_empty_transitory_shape() {
//         let game = Game::new();


//         assert_eq!(game.response_to_input(Shape::Circle), Action::CreateNewShape);
//         assert_eq!(game.response_to_input(Shape::NoShape), Action::DoNothing);
//     }

// }