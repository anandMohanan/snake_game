use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

pub enum Direction {
    up,
    down,
    left,
    right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::up => Direction::down,
            Direction::down => Direction::up,
            Direction::left => Direction::right,
            Direction::right => Direction::left,
        }
    }
}

struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x: x, y });
    }

    Snake{
        direction: Direction::right,
        body,
        tail: None
    }
    

    pub fn draw(&self,con: &Context,g: G2d){
        for block in &self.body{
            draw::draw_block(SNAKE_COLOR,block.x,block.y,con,g)
        }
    }

    pub fn head_position(&self) -> (i32,i32){
        let head_block = self.body.front().unwrap();
        (head_block.x , head_block.y)
    }
}

