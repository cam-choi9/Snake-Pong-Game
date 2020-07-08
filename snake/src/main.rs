extern crate piston_window;
extern crate rand;
extern crate find_folder;
extern crate opengl_graphics;

mod draw;
mod objects;
mod game;

use piston_window::*;
use piston_window::types::Color;

use crate::game::Game;
use crate::draw::to_coord_u32;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    // size of the gameboard
    let (width, height) = (30, 30);

    // open up the gameboard with width & height
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .fullscreen(true)
            .exit_on_esc(true)
            .build()
            .unwrap();

    // create a game
    let mut game = Game::new(width, height);    

    // load font 
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("FiraMono-Bold.ttf");    
    let mut glyphs = window.load_font(font).unwrap();

    // game end statements
    let gameover: &str = "GAME OVER";
    let gamewin: &str = "WIN!";

    // game loop
    while let Some(e) = window.next() {        
        // get score & make it a string
        let score: i32 = game.score();
        let mut scoreboard = "Score: ".to_owned();
        let s_score : String = score.to_string().to_owned();    
        scoreboard.push_str(&s_score);        

        // game end status
        let gameover_state: bool = game.game_over();
        let gamewin_state: bool = game.game_win();

        // key inputs
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_pressed(key);
        }        

        // draw game components & texts 
        window.draw_2d(&e, |c, g, device| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);

            let transform = c.transform.trans(900.0, 300.0);
            let gameover_statement = c.transform.trans(450.0, 400.0);
            let gamewin_statement = c.transform.trans(450.0, 400.0);
            
            // score text
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 20).draw(
                &scoreboard,
                &mut glyphs,
                &c.draw_state,
                transform, g
            ).unwrap();            

            // game over statement text
            if gameover_state {
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 70).draw(
                    &gameover,
                    &mut glyphs,
                    &c.draw_state,
                    gameover_statement, g
                ).unwrap();     
            }

            // game win statement text
            if gamewin_state {
                text::Text::new_color([0.0, 0.0, 0.0, 1.0], 70).draw(
                    &gamewin,
                    &mut glyphs,
                    &c.draw_state,
                    gamewin_statement, g
                ).unwrap();     
            }

            // flush so that the string buffer can be passed to GPU completely
            glyphs.factory.encoder.flush(device);
        });

        // update game status
        e.update(|arg| {
            game.update(arg.dt);
        });

    }
}