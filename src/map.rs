use crate::block::Block;

use sdl2::rect::Rect;

pub struct Map {
    pub block_list: Vec<Block>
}

impl Map {
    pub fn new() -> Map {
        let mut block_list = Vec::new();

        for i in 0..32 {
            block_list.push(Block::new(i * 20, 0));
            block_list.push(Block::new(i * 20, 460));
        }

        for i in 1..23 {
            block_list.push(Block::new(0, i * 20));
            block_list.push(Block::new(620, i * 20));
        }

        Map {
            block_list: block_list
        }
    }

    pub fn get_rects(&self) -> Vec<Rect> {
        self.block_list.iter().map(|x| { Block::to_rect(&x) }).rev().collect()
    }
}
