use crate::coordinates::Coordinates;
use crate::direction::Direction;
use sdl2::rect::Rect;

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Block {
    pub position: Coordinates
}

impl Block {
    pub fn new(x: u32, y: u32) -> Block {
        Block {
            position: Coordinates::new(x, y)
        }
    }

    pub fn to_rect(&self) -> Rect {
        Rect::new(
            self.position.x as i32,
            self.position.y as i32,
            20,
            20
        )
    }

    pub fn next(base: Block, direction: Direction) -> Block{
        let mut next_block = base;
        match direction {
            Direction::Up => {
                next_block.shift(0, -20);
            },
            Direction::Down => {
                next_block.shift(0, 20);
            },
            Direction::Left => {
                next_block.shift(-20, 0);
            },
            Direction::Right => {
                next_block.shift(20, 0);
            }
        }

        next_block
    }

    pub fn shift(&mut self, x: i32, y: i32){
        let new_x;
        let new_y;

        if y < 0 {
            new_y = self.position.y - y.abs() as u32;
        }else{
            new_y = self.position.y + y as u32;
        }

        if x < 0 {
            new_x = self.position.x - x.abs() as u32;
        }else{
            new_x = self.position.x + x as u32;
        }

        self.position = Coordinates::new(new_x, new_y);
    }

    pub fn random_block(){

    }
}

