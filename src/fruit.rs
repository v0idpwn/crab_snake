use crate::coordinates::Coordinates;
use sdl2::rect::Rect;

#[derive(Copy, Clone, Debug)]
pub struct Fruit {
    pub position: Coordinates
}

impl Fruit {
    pub fn new(x: u32, y: u32) -> Fruit {
        Fruit {
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
}

