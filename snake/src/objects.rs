// LinkedList => snake & pedals
// Vector => enemy
use std::collections::LinkedList; 
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0]; //RGB, opacity
const PEDAL_COLOR: Color = [0.00, 0.00, 0.00, 1.0]; 
const ENEMY_COLOR: Color = [0.80, 0.00, 0.00, 1.0];

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd)]
// direction type
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

// direction design
impl Direction {
    // preventing snake from going to its opposite way
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Block {
    x: i32,
    y: i32
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>
}

pub struct Enemy {
    gang: Vec<Block>
}

pub struct LeftPedal {
    body: LinkedList<Block>
}

pub struct RightPedal {
    body: LinkedList<Block>    
}

// snake design
impl Snake {
    // create a new snake
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        // start with a length of 2
        body.push_back(Block {
            x : x+1,
            y
        });
        body.push_back(Block {
            x : x,
            y
        });

        Snake {
            direction: Direction::Right,
            body,
            tail: None
        }

    }

    // draw snake
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    // return head's position
    pub fn head_position(&self) -> (i32, i32) {
        // provides a reference to the front element
        // unwrap the element without error handling
        let head_block = self.body.front().unwrap();    
        (head_block.x, head_block.y)
    }

    // moving directions
    pub fn move_forward(&mut self, dir:Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => ()
        }

        // get the head position
        let (last_x, last_y): (i32, i32) = self.head_position();

        // depending on its direction, create a new block to its next position
        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y
            }
        };
        // add a new block to its head
        self.body.push_front(new_block);
        // remove the tail node
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    // get its direction
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    // get the next position where snake is heading to
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }
        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    // adding a new block to snake's tail
    pub fn restore_tail(&mut self) {
        let block = self.tail.clone().unwrap();
        self.body.push_back(block);        
    }

    // check if the next block position of the head overlaps with the current tail block position
    pub fn overlap_tail(&mut self, x: i32, y: i32) -> bool {
        let mut ch = 0;

        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }
            ch += 1;
            // if it overlaps, return false
            // -> means if the next position of head node = the current position of tail node,
            // it is an acceptable move (as soon as head -> next position, tail -> escape from the position)
            if ch == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }  
}

// enemy design
impl Enemy {

    // create a enemy vector
    pub fn new() -> Enemy {
        let mut gang : Vec<Block> = Vec::new();

        // initial enemy position
        gang.push(Block {
            x: 10,
            y: 6
        });

        Enemy {
            gang
        }
    }
    
    // draw enemies
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for e in &self.gang {
            draw_block(ENEMY_COLOR, e.x, e.y, con, g);
        }
    }

    // adding a new enemy into the gameboard
    pub fn add_enemy(&mut self) {
        let last_index: usize = self.gang.len()-1;        
        let mut new_point_x: i32 = 0;
        let mut new_point_y: i32 = 0;

        // designated locations of enemies
        if last_index == 0 {
            new_point_x = 20;
            new_point_y = 6;            
        }

        if last_index == 1 {
            new_point_x = 7;
            new_point_y = 12;         
        }

        if last_index == 2 {
            new_point_x = 15;
            new_point_y = 12;
        }

        if last_index == 3 {
            new_point_x = 22;
            new_point_y = 12;
        }

        if last_index == 4 {
            new_point_x = 7;
            new_point_y = 18;
        }

        if last_index == 5 {
            new_point_x = 15;
            new_point_y = 18;            
        }

        if last_index == 6 {
            new_point_x = 22;
            new_point_y = 18;            
        }

        if last_index == 7 {
            new_point_x = 10;
            new_point_y = 24;            
        }

        if last_index == 8 {
            new_point_x = 20;
            new_point_y = 24;            
        }                        

        let new_block: Block = Block {
            x : new_point_x, 
            y : new_point_y
        };
        // add a new enemy 
        self.gang.push(new_block);
        return;
        
    }

    // check if the snake contacts with any of the enemy
    pub fn contact(&mut self, x: i32, y: i32) -> bool {
        let block = Block {
            x, y
        };

        // check if any coordinates of enemies match with snake's head position
        if self.gang.contains(&block) {
            return true;
        }

        return false;
    }
}

// left pedal design
impl LeftPedal {
    // create a new left pedal
    pub fn new() -> LeftPedal {
        let mut body: LinkedList<Block> = LinkedList::new();
        // pedal - length of 5
        body.push_back(Block {
            x : 0,
            y : 1
        });
        body.push_back(Block {
            x : 0,
            y : 2
        });
        body.push_back(Block {
            x : 0,
            y : 3
        });
        body.push_back(Block {
            x : 0,
            y : 4
        });
        body.push_back(Block {
            x : 0,
            y : 5
        });

        LeftPedal {
            body            
        }
    }

    // draw left pedal
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(PEDAL_COLOR, block.x, block.y, con, g);
        }
    }

    // return head's position
    pub fn head_position(&self) -> (i32, i32) {        
        let head_block = self.body.front().unwrap();            
        (head_block.x, head_block.y)
    }

    // return tail's position
    pub fn tail_position(&self) -> (i32, i32) {
        let tail_block = self.body.back().unwrap();
        (tail_block.x, tail_block.y)
    }

    // moving up
    pub fn move_up(&mut self) {        
        // get the head position
        let (last_x, last_y): (i32, i32) = self.head_position();
        // stop if it reaches the top
        if last_y == 1 {
            return;
        }

        // add another block to one pixel above the original pedal
        let new_block = Block {
                x: last_x,
                y: last_y - 1
        };
        // add to the linkedlist
        self.body.push_front(new_block);
        // remove the very bottom block, which is the tail node
        self.body.pop_back().unwrap();        
    }

    // moving down
    pub fn move_down(&mut self) {        
        // get the tail position
        let (last_x, last_y): (i32, i32) = self.tail_position();

        // stop if it reaches the bottom 
        if last_y == 28 {
            return;
        }

        // add another block to one pixel below the original pedal
        let new_block = Block {
                x: last_x,
                y: last_y + 1
        };
        // add to the linkedlist
        self.body.push_back(new_block);
        // remove the very top block, which is the head node
        self.body.pop_front().unwrap();
    }

    // check if the pedal blocks the food
    pub fn block_food(&mut self, y: i32) -> bool {
        // comparing the positions of the food and of the pedal's body
        for block in &self.body {
            if y == block.y {
                return true;
            }            
        }
        return false;
    }

}

// right pedal design
impl RightPedal {
    // create a new right pedal
    pub fn new() -> RightPedal {
        let mut body: LinkedList<Block> = LinkedList::new();
        // pedal - length of 5
        body.push_back(Block {
            x : 29,
            y : 1
        });
        body.push_back(Block {
            x : 29,
            y : 2
        });
        body.push_back(Block {
            x : 29,
            y : 3
        });
        body.push_back(Block {
            x : 29,
            y : 4
        });
        body.push_back(Block {
            x : 29,
            y : 5
        });

        RightPedal {
            body
        }
    }

    // draw right pedal
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(PEDAL_COLOR, block.x, block.y, con, g);
        }
    }

    // return head's position
    pub fn head_position(&self) -> (i32, i32) {        
        let head_block = self.body.front().unwrap();            
        (head_block.x, head_block.y)
    }

    // return tail's position
    pub fn tail_position(&self) -> (i32, i32) {
        let tail_block = self.body.back().unwrap();
        (tail_block.x, tail_block.y)
    }

    // moving up
    pub fn move_up(&mut self) {
        
        // get the head position
        let (last_x, last_y): (i32, i32) = self.head_position();

        // stop if it reaches the top
        if last_y == 1 {
            return;
        }

        // add another block to one pixel above the original pedal
        let new_block = Block {
                x: last_x,
                y: last_y - 1
        };
        // add it to the linkedlist
        self.body.push_front(new_block);
        // remove the very bottom block, which is the tail node
        self.body.pop_back().unwrap();
    }

    // moving down
    pub fn move_down(&mut self) {
        // get the tail position
        let (last_x, last_y): (i32, i32) = self.tail_position();

        // stop if it reaches the bottom
        if last_y == 28 {
            return;
        }

        // add another block to one pixel below the original pedal
        let new_block = Block {
                x: last_x,
                y: last_y + 1
        };
        // add it to the linkedlist
        self.body.push_back(new_block);
        // remove the very top block, which is the head node
        self.body.pop_front().unwrap();                
    }

    // check if the pedal blocks the food
    pub fn block_food(&mut self, y: i32) -> bool {
        
        // comparing the positions of the food and of the pedal's body
        for block in &self.body {
            if y == block.y {
                return true;
            }            
        }
        return false;
    }

}