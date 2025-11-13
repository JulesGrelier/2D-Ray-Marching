use macroquad::color::{SKYBLUE, RED};
use macroquad::input::KeyCode;
use macroquad::math::Vec2;
use macroquad::shapes::{draw_circle, draw_circle_lines};

use crate::shape::{IsShape, Shape};
use crate::shapes::{circle::Circle, rectangle::Rectangle};

type DynShape = Box<dyn IsShape>;

pub struct Game {
    shapes : Vec<DynShape>,
    proto_shape : Option<DynShape>,
    selected_shape : Option<Shape>,


    player : Vec2,
    orientation : f32,
}

impl Game {
    pub fn new() -> Self {
        Self { shapes: Vec::new(), proto_shape: None, selected_shape: None, player: Vec2::new(500.0, 500.0), orientation: 0.0}
    }



    pub fn draw(&self, mouse : (f32, f32)) {
        for shape in &self.shapes {
            shape.draw();
        }
        if let Some(shape) = &self.proto_shape {
            shape.proto_draw(mouse.0, mouse.1);
        }
    }



    /* --------------------------- SHAPES MANAGEMENT --------------------------- */

    pub fn modify_selected_key(&mut self, key : KeyCode) {

        if key == KeyCode::Escape {
            self.selected_shape = None;
            self.proto_shape = None;
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



    /* --------------------------- RAY-MARCHING MANAGEMENT --------------------------- */

    fn distance_nearest_shape(&self, x : f32, y : f32) -> f32 {

        let mut nearest_distance = 999999.9;

        for shape in &self.shapes {
            nearest_distance = f32::min(shape.distance_from(x, y), nearest_distance)
        }

        println!("La plus proche distance est : {}", nearest_distance);

        nearest_distance
    }



    fn propagate(&self, pos : Vec2, distance : f32, degree : f32) -> Vec2 {
        //Vec2::new(0.0, 0.0)
        let dx = degree.to_radians().cos() * distance;
        let dy = degree.to_radians().sin() * distance;

        let delta = Vec2::new(dx, dy);

        println!("{} -> {} à {}° pour une distance de {}", pos, pos+delta, degree, distance);

        return pos + delta;
    }



    pub fn animation(&mut self) {

        self.orientation += 0.2;

        println!("----------------------------------------");

        let mut pos = self.player;

        draw_circle(pos.x, pos.y, 10.0, RED);

        for _ in 0..20 {
            let distance = self.distance_nearest_shape(pos.x, pos.y);

            let new_pos = self.propagate(pos, distance, self.orientation);

            draw_circle_lines(pos.x, pos.y, distance, 2.0, SKYBLUE);
            draw_circle(new_pos.x, new_pos.y, 5.0, RED);

            pos = new_pos;
            if distance < 1.0 { break; }
        }
    }
}