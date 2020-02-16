extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use rand::Rng;

mod coordinates;
mod block;
mod map;
mod snake;
mod direction;
mod fruit;

const BLACK: Color = Color::RGB(0, 0, 0);
const SNAKE_COLOR: Color = Color::RGB(255, 0, 0);
const FRUIT_COLOR: Color = Color::RGB(0, 255, 50);
const WALL_COLOR: Color = Color::RGB(50, 50, 50);

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("crab-snake", 640, 480)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(BLACK);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rng = rand::thread_rng();

    // Stuff
    let map = map::Map::new();
    let mut snake = snake::Snake::new(300, 400);
    let mut fruit = fruit::Fruit::new(240, 40);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    snake.set_direction(direction::Direction::Up);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    snake.set_direction(direction::Direction::Left);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    snake.set_direction(direction::Direction::Down);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    snake.set_direction(direction::Direction::Right);
                }
                _ => {}
            }
        }


        // Game logic
        snake.mv();

        match check_collision_with_fruit(snake.get_head(), fruit) {
            true => fruit = {
                let new_x = rng.gen_range(1, 31) * 20;
                let new_y = rng.gen_range(1, 23) * 20;
                fruit::Fruit::new(new_x, new_y)
            },
            false => {snake.shrink();}
        }

        match check_collision(&snake.block_list, &map.block_list) {
            true => {break 'running},
            false => {}
        }

        // Game drawing
        canvas.set_draw_color(BLACK);
        canvas.clear();
        canvas.set_draw_color(WALL_COLOR);

        let map_rects = map.get_rects();
        for rect in map_rects {
            canvas.draw_rect(rect).unwrap();
            canvas.fill_rect(rect).unwrap();
        }

        canvas.set_draw_color(SNAKE_COLOR);

        let snake_rects = snake.get_rects();
        for rect in snake_rects {
            canvas.draw_rect(rect).unwrap();
            canvas.fill_rect(rect).unwrap();
        }

        canvas.set_draw_color(FRUIT_COLOR);
        let fruit_rect = fruit.to_rect();
        canvas.draw_rect(fruit_rect).unwrap();
        canvas.fill_rect(fruit_rect).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 12));
    }
}

pub fn check_collision_with_fruit(head: block::Block, fruit: fruit::Fruit) -> bool {
    fruit.position.x == head.position.x && fruit.position.y == head.position.y
}

pub fn check_collision(v1: &[block::Block], v2: &[block::Block]) -> bool {
    let mut buffer = v1.to_vec();
    buffer.extend_from_slice(v2);
    let previous_len = buffer.len();
    buffer.sort_unstable();
    buffer.dedup();

    buffer.len() != previous_len
}
