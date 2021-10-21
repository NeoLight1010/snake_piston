use std::collections::LinkedList;
use std::iter::FromIterator;

use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::constants::SQUARE_SIZE;
use crate::direction::Direction;

pub struct Snake {
    pub body: LinkedList<(i32, i32)>,
    pub previous_segment: Option<(i32, i32)>,
    pub direction: Direction,
}

impl Default for Snake {
    fn default() -> Snake {
        Snake {
            body: LinkedList::from_iter([(0, 0), (0, 1)]),
            previous_segment: None,
            direction: Direction::Right,
        }
    }
}

impl Snake {
    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const RED: [f32; 4] = [1., 0., 0., 1.];

        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| {
                graphics::rectangle::square(
                    x as f64 * SQUARE_SIZE,
                    y as f64 * SQUARE_SIZE,
                    SQUARE_SIZE,
                )
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
            Direction::Down => new_head.1 += 1,
        }

        self.body.push_front(new_head);
        self.previous_segment = self.body.pop_back();
    }

    /// Returns a reference to the head of the snake.
    pub fn get_head(&mut self) -> Option<&(i32, i32)> {
        self.body.front()
    }

    pub fn grow(&mut self) {
        if self.previous_segment.is_some() {
            self.body.push_back(self.previous_segment.unwrap());
        }

    }
}
