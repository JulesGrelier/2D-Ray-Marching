use macroquad::prelude::*;

mod shape;
mod game;
mod shapes;

use game::Game;

fn conf() -> Conf {
    Conf { window_title: "Ray-marching".to_string(), window_width: 1080, window_height: 720, ..Default::default() }
}


#[macroquad::main(conf)]
async fn main() {

    let mut game = Game::new();

    loop {

        //JUST ONE PRESS IN THE SAME TIME
        let keys : Vec<KeyCode> = get_keys_pressed().extract_if(|key| is_key_pressed(*key)).collect();

        if !keys.is_empty() {
            game.modify_selected_key(keys[0]);
        }

        let pos_mouse = mouse_position();

        if is_mouse_button_pressed(MouseButton::Left) & game.has_a_selected_key() {
            let new_shape = game.must_create_new_shape();
            game.get_new_pos(pos_mouse, new_shape);
        }

        game.draw(pos_mouse);
        next_frame().await;
    }
}
