use macroquad::input::KeyCode;

use crate::shape::{IsShape, Shape};
use crate::shapes::{circle::Circle, rectangle::Rectangle};

type DynShape = Box<dyn IsShape>;

pub struct Game {
    shapes : Vec<DynShape>,
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

        if key == KeyCode::Escape {
            self.selected_shape = None;
        }

        let shape = match key {
            KeyCode::C => Shape::Circle,
            KeyCode::R => Shape::Rectangle,
            _ => return,
        };

        self.selected_shape = Some(shape)
    }



    pub fn has_a_selected_key(&self) -> bool {
        self.selected_shape.is_some()
    }



    pub fn must_create_new_shape(&self) -> bool {
        match &self.proto_shape {
            Some(x) => x.return_shape() != self.selected_shape.unwrap(),
            None => true
        }
    }



    fn concretise_shape(&self, shape : Shape) -> DynShape {
        match shape {
            Shape::Circle => { return Box::new(Circle::new()); },
            Shape::Rectangle => { return Box::new(Rectangle::new()); },
        };
    }



    pub fn get_new_pos(&mut self, pos_mouse : (f32, f32), new_shape : bool) {
        if new_shape {
            self.proto_shape = Some(self.concretise_shape(self.selected_shape.unwrap()));
        }

        let enough_pos = self.proto_shape.as_mut().unwrap().get_new_pos(pos_mouse.0, pos_mouse.1);
        if enough_pos { self.move_proto_shape(); }
    }



    fn move_proto_shape(&mut self) {
        let young_shape = self.proto_shape.take().unwrap();
        self.shapes.push(young_shape);
    }
}