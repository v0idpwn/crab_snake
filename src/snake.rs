use crate::block::Block;
use crate::direction::Direction;
use sdl2::rect::Rect;

pub struct Snake {
    pub block_list: Vec<Block>,
    pub heading: Direction
}

impl Snake {
    pub fn new(x: u32, y: u32) -> Snake {
        let mut block_list = Vec::new();

        block_list.push(Block::new(x-80,y));
        block_list.push(Block::new(x-60,y));
        block_list.push(Block::new(x-40,y));
        block_list.push(Block::new(x-20,y));
        block_list.push(Block::new(x,y));

        Snake {
            block_list: block_list,
            heading: Direction::Up
        }
    }

    pub fn set_direction(&mut self, direction: Direction) -> () {
        let new_direction;

        match(self.heading, direction){
            (Direction::Up, Direction::Down) => new_direction = self.heading,
            (Direction::Down, Direction::Up) => new_direction = self.heading,
            (Direction::Left, Direction::Right) => new_direction = self.heading,
            (Direction::Right, Direction::Left) => new_direction = self.heading,
            (_, _) => new_direction = direction
        }

        self.heading = new_direction;
    }

    pub fn mv(&mut self) -> () {
        let next_block = Block::next(*self.block_list.last().unwrap(), self.heading);
        self.block_list.push(next_block);
    }

    pub fn shrink(&mut self) -> () {
        self.block_list.drain(0..1);
    }

    pub fn get_rects(&self) -> Vec<Rect> {
        self.block_list.iter().map(|x| { Block::to_rect(&x) }).rev().collect()
    }

    pub fn get_head(&self) -> Block {
        *self.block_list.last().unwrap()
    }
}
