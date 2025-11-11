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
        clear_background(BLUE);

        //JUST ONE PRESS IN THE SAME TIME
        let mut keys = get_keys_pressed();
        let extracted : Vec<KeyCode> = keys.extract_if(|v| is_key_pressed(*v)).collect();

        if !extracted.is_empty() {
            game.modify_selected_key(extracted[0]);
            println!("Nouvelle touche : {:?}", extracted[0])
        }

        let mouse = mouse_position();

        if is_mouse_button_pressed(MouseButton::Left) {
            println!("Bouton préssé");
            if let Some(input) = game.create_input(mouse.0, mouse.1) {
                let action = game.return_action(input.shape);
                game.get_new_pos(&input, action);
            }
        }

        game.draw(mouse);
        println!("{:?}", game.shapes.len());

        //draw_circle(100.0, 100.0, 50.0, WHITE);

        next_frame().await;
    }
}
