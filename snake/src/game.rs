// piston game engine libraries
use piston_window::*;
use piston_window::types::Color;

// random number generator
use rand::{thread_rng, Rng};

// functions implemented in snake.rs & draw.rs
use crate::objects::{Direction, Snake, Enemy, LeftPedal, RightPedal};
use crate::draw::{draw_block, draw_rectangle};

// colors
// food -> blue, enemy -> red, border -> black, gameover -> light red, gamewin -> light blue
const FOOD_COLOR: Color = [0.00, 0.00, 0.80, 1.0];      
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const SIDE_COLOR: Color = [0.00, 0.00, 0.00, 0.5];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];
const GAMEWIN_COLOR: Color = [0.00, 0.00, 0.90, 0.5];

// moving speed & response time
const MOVING_PERIOD: f64 = 0.3;
const RESTART_TIME: f64 = 2.0;
 
// game components
pub struct Game {
    snake: Snake,        

    l_pedal: LeftPedal,
    r_pedal: RightPedal,    

    enemy: Enemy,    

    food_exists: bool,
    food_x: i32,
    food_y: i32,
    food_speed_x: i32,
    food_speed_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    game_win: bool,
    waiting_time: f64,
    score: i32    
}

// game design
impl Game {
    // starting up a new game
    pub fn new(width: i32, height: i32) -> Game {
        Game {            
            snake: Snake::new(5, 5),    // start moving at (5,5)
            l_pedal: LeftPedal::new(),  // initially located at the top of both sides
            r_pedal: RightPedal::new(),                     
            enemy: Enemy::new(),  
            waiting_time: 0.0,
            food_exists: true,
            food_x: 3,                 // start moving at (3,3)
            food_y: 3,
            food_speed_x: 1,
            food_speed_y: 1,                      
            width,
            height,
            game_over: false,
            game_win: false,
            score: 0
        }
    }

    // keyboard inputs
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        // snake control
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction())
        };

        // left pedal control
        match key {
            Key::Q => self.l_pedal.move_up(),
            Key::A => self.l_pedal.move_down(),
            _ => ()
        };

        // right pedal control
        match key {
            Key::R => self.r_pedal.move_up(),
            Key::F => self.r_pedal.move_down(),
            _ => ()
        };

        // prevent a snake from turning to the opposite direction
        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        // update snake status
        self.update_snake(dir);
    }

    // draw components
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // snake
        self.snake.draw(con, g);        

        // pedals
        self.l_pedal.draw(con, g);
        self.r_pedal.draw(con, g);

        // enemy
        self.enemy.draw(con, g);

        // food 
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);            
        }
        
        // boundaries
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(SIDE_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(SIDE_COLOR, self.width - 1, 0, 1, self.height, con, g);

        // game over screen
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }

        // game win screen
        if self.game_win {
            draw_rectangle(GAMEWIN_COLOR, 0, 0, self.width, self.height, con, g);          
        }

    }

    // update components
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        // if score reaches 21, win
        if self.score == 21 {
            self.game_win = true;                                                            
        }

        // if game over, restart the game after restart time (2.0)
        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        // if game win, restart the game after restart time (2.0)
        if self.game_win {            
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        // if snake eats the food, generate another food
        if !self.food_exists {
            self.add_food();            
        }

        // update 
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
            self.food_moving();            
        }
    }

    // food moving function
    fn food_moving(&mut self) {
                
        // only ceiling and floor
        let top_bound = 0;
        let bottom_bound = self.height;        

        // moving with a pattern        
        self.food_x += self.food_speed_x;        
        self.food_y += self.food_speed_y;
        
        if self.food_y <= top_bound+1 || self.food_y >= bottom_bound-2 {
            self.food_speed_y = -self.food_speed_y;
        }

        // blocking with the left pedal 
        if self.food_x-1 == 0 {
            if self.l_pedal.block_food(self.food_y+1) || self.l_pedal.block_food(self.food_y-1) {
                self.food_speed_x = -self.food_speed_x;
            } else {
                self.game_over = true;
            }            
        }

        // blocking with the right pedal
        if self.food_x+2 == self.width {
            if self.r_pedal.block_food(self.food_y+1) || self.r_pedal.block_food(self.food_y-1) {
               self.food_speed_x = -self.food_speed_x;
            } else {
                self.game_over = true;
            }       
        }

    }

    // return score
    pub fn score(&mut self) -> i32 {
        return self.score;
    }

    // return game over state
    pub fn game_over(&mut self) -> bool {
        return self.game_over
    }

    // return game over state
    pub fn game_win(&mut self) -> bool {
        return self.game_win
    }

    // check if snake eats
    pub fn check_eating(&mut self) -> bool {
        // head position of the snake
        let (head_x, head_y): (i32, i32) = self.snake.head_position();        

        // if it matches 
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
            if self.score % 2 == 1 {
                self.enemy.add_enemy();
            }            
            self.score += 1;
            return true;
        }
        return false;
    }

    // check if snake contacts with itself or with enemy
    fn check_if_snake_alive(&mut self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);        

        if self.snake.overlap_tail(next_x, next_y) 
            || self.enemy.contact(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    // adding another food in a random spot 
    // 5 block size away from boundaries -> for pedals to move
    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(5, self.width - 5);
        let mut new_y = rng.gen_range(5, self.height - 5);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(5, self.width - 5);
            new_y = rng.gen_range(5, self.height - 5);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    // updating snake's status
    fn update_snake(&mut self, dir: Option<Direction>) {
        // if snake is alive,
        if self.check_if_snake_alive(dir) {
            // always on the move & checking if it eats an apple
            self.snake.move_forward(dir);
            self.check_eating();            
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    // restart the game
    // reinitialize all the variables
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.l_pedal = LeftPedal::new();
        self.r_pedal = RightPedal::new(); 
        self.enemy = Enemy::new();     
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 10;
        self.food_y = 10;
        self.food_speed_x = 1;
        self.food_speed_y = 1;
        self.game_over = false;
        self.game_win = false;
        self.score = 0;
    }
} 