use std::collections::LinkedList;

use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::direction::Direction;

pub struct Snake {
    pub body: LinkedList<(i32, i32)>,
    pub direction: Direction,
}

impl Snake {
    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const RED: [f32; 4] = [1., 0., 0., 1.];

        let squares: Vec<graphics::types::Rectangle> = self.body.iter()
            .map(|&(x, y)| {
                graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, 20.)
            })
        .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            for square in squares {
                graphics::rectangle(RED, square, transform, gl);
            }
        });
    }

    pub fn update(&mut self) {
        let mut new_head = self.body.front().expect("Snake has no body!").clone();

        match self.direction {
            Direction::Up => new_head.1 -= 1,
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Down => new_head.1 += 1
        }

        self.body.push_front(new_head);
        self.body.pop_back();
    }
}
